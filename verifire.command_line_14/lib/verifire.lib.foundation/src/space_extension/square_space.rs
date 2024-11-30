use crate::i_overlay::line_range::LineRange;
use crate::space::line_indexer::LineIndexer;
use crate::space::line_space::IntExtensions;
use crate::space_extension::square_range::SquareRange;
use crate::space_extension::square_segment::SquareSegment;
use crate::space_extension::square_vector::SquareVector;

#[derive(Debug, Clone)]
struct Def {
    x_min: i32,
    x_max: i32,
    indices: Vec<usize>,
}

impl Def {
    fn range(&self) -> LineRange {
        LineRange { min: self.x_min, max: self.x_max }
    }

    fn new() -> Self {
        Def {
            x_min: i32::MAX,
            x_max: i32::MIN,
            indices: Vec::new(),
        }
    }

    fn add(&mut self, range: &SquareRange, index: usize) {
        self.x_min = self.x_min.min(range.x_min);
        self.x_max = self.x_max.max(range.x_max);
        self.indices.push(index);
    }
}

pub struct RowSpace {
    pub indexer: LineIndexer,
    pub heaps: Vec<Option<Vec<SquareSegment>>>, // Assuming SquareSegment is defined
}

impl RowSpace {
    fn new(indexer: LineIndexer, heaps: Vec<Option<Vec<SquareSegment>>>) -> Self {
        RowSpace { indexer, heaps }
    }
}

pub struct SquareSpace {
    pub indexer: LineIndexer,
    pub rows: Vec<Option<RowSpace>>,
}

pub struct SquareSpaceBuffer {
    x: Vec<usize>,
    y: Vec<usize>,
}

impl SquareSpaceBuffer {
    pub fn new() -> Self {
        Self { x: Vec::new(), y: Vec::new() }
    }
}

impl SquareSpace {
    pub fn new(vectors: &Vec<SquareVector>) -> Self {
        let mut segments = Vec::with_capacity(vectors.len());

        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        for i in 0..vectors.len() {
            let range = &vectors[i].range;

            y_min = y_min.min(range.y_min);
            y_max = y_max.max(range.y_max);

            segments.push(SquareSegment { id: i, range: range.clone() });
        }

        let max_level = 2.max(((vectors.len() as f64 / 4.0).sqrt() as usize).log_two());

        let row_indexer = LineIndexer::new(max_level, LineRange { min: y_min, max: y_max });
        let mut defs = vec![Def::new(); row_indexer.size];

        for i in 0..segments.len() {
            let range = &segments[i].range;
            let y_index = row_indexer.unsafe_index(range.y_range());
            defs[y_index].add(range, i);
        }

        let mut rows = Vec::with_capacity(row_indexer.size);

        for i in 0..defs.len() {
            let def = &defs[i];

            if def.indices.len() > 0 {
                let max_level = 2.max((def.indices.len() / 4).log_two());
                let indexer = LineIndexer::new(max_level, def.range());
                let mut heaps: Vec<Option<Vec<SquareSegment>>> = vec![None; indexer.size];

                for &index in def.indices.iter() {
                    let segment = &segments[index];
                    let x_index = indexer.unsafe_index(segment.range.x_range());
                    match heaps[x_index].as_mut() {
                        Some(heap) => heap.push(segment.clone()),
                        None => heaps[x_index] = Some(vec![SquareSegment { id: segment.id, range: segment.range.clone() }]),
                    }
                }

                rows.push(Some(RowSpace::new(indexer, heaps)));
            } else {
                rows.push(None);
            }
        }

        Self {
            indexer: row_indexer,
            rows,
        }
    }

    pub fn ids_in_range(&self, range: &SquareRange, exclude_id: usize, buffer: &mut SquareSpaceBuffer, ids: &mut Vec<usize>) {
        self.indexer.fill_unsafe(range.y_range(), &mut buffer.y);
        if buffer.y.is_empty() {
            return;
        }

        let x_range = range.x_range();
        for &yi in buffer.y.iter() {

            if let Some(row) = &self.rows[yi] {
                row.indexer.fill(x_range, &mut buffer.x);

                for &xi in buffer.x.iter() {
                    if let Some(segments) = &row.heaps[xi] {
                        for segment in segments.iter() {
                            if segment.range.is_overlap(range) && exclude_id != segment.id {
                                ids.push(segment.id);
                            }
                        }
                    }
                }

                buffer.x.clear();
            }
        }

        buffer.y.clear();
    }
}