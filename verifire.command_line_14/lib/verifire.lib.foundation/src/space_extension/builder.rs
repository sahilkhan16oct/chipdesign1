use crate::i_overlay::i_shape::int::shape::{IntShape, PointsCount};
use crate::space_extension::square_index::{ShapeLayer, SquareIndex};
use crate::space_extension::square_vector::SquareVector;

pub trait SquareVectorBuilder {
    fn build_square_vectors(&self, layer: ShapeLayer, start_id: usize) -> Vec<SquareVector>;
}

impl SquareVectorBuilder for IntShape {
    fn build_square_vectors(&self, layer: ShapeLayer, start_id: usize) -> Vec<SquareVector> {
        let mut vectors = Vec::with_capacity(self.points_count());
        fill_square_vectors(self, start_id, layer, 0, &mut vectors);
        vectors
    }
}

impl SquareVectorBuilder for Vec<IntShape> {
    fn build_square_vectors(&self, layer: ShapeLayer, start_id: usize) -> Vec<SquareVector> {
        let mut vectors = Vec::with_capacity(self.points_count());

        let mut id = start_id;
        for shape_index in 0..self.len() {
            fill_square_vectors(&self[shape_index], id, layer, shape_index, &mut vectors);
            if let Some(last) = vectors.last() {
                id = last.id + 1;
            }
        }

        vectors
    }
}

fn fill_square_vectors(shape: &IntShape, start_id: usize, layer: ShapeLayer, shape_index: usize, vectors: &mut Vec<SquareVector>) {
    let mut id = start_id;

    for path_index in 0..shape.len() {
        let path = &shape[path_index];
        let last_index = path.len() - 1;
        let mut ai = last_index;

        for bi in 0..path.len() {
            let index = SquareIndex::new(layer, shape_index, path_index, ai, last_index);
            let vector = SquareVector::new(id, path[ai], path[bi], index);
            vectors.push(vector);
            ai = bi;
            id += 1;
        }
    }
}
