use std::cmp::Ordering;
use foundation::res::shape_res::ShapeRes;
use foundation::i_overlay::core::fill_rule::FillRule;
use foundation::i_overlay::core::overlay_rule::OverlayRule;
use foundation::i_overlay::core::overlay::{Overlay, ShapeType};
use foundation::i_overlay::i_shape::int::path::{IntPath, PointPathExtension};
use foundation::i_overlay::i_shape::int::shape::{IntShape, PointsCount};
use foundation::i_overlay::line_range::LineRange;
use foundation::space::scan_space::{ScanSegment, ScanSpace};
use foundation::space_extension::square_range::SquareRange;
use crate::solution::bool_func::{BoolFunc, RectBool};
use foundation::space::line_range::LineRangeExt;

struct IdRange {
    index: usize,
    range: SquareRange,
}

pub struct EnclosureRectSolver {
    float_res: ShapeRes,
    rects_res: ShapeRes,
    bool_func: BoolFunc,
    value_0: i32,
    value_1: i32,
    is_two_values: bool,
    only_inner: bool,
}

impl EnclosureRectSolver {

    pub fn new(expression: &str, float_res: ShapeRes, rects_res: ShapeRes, value_0: i32, value_1: Option<i32>, only_inner: Option<bool>) -> Result<Self, String> {
        let only_inner = only_inner.unwrap_or(true);

        let value_0 = value_0;
        let (value_1, is_two_values) = if let Some(value) = value_1 {
            (value, true)
        } else {
            (0, false)
        };

        let bool_func = BoolFunc::new(expression, is_two_values)?;

        Ok(Self { float_res, rects_res, bool_func, value_0, value_1, is_two_values, only_inner })
    }

    pub fn find_problems(&self) -> Vec<IntPath> {
        let rects_shapes = self.rects_res.shapes();
        let float_shapes = self.float_res.shapes();

        let max = 0.max(self.value_0.max(self.value_1));

        let ranges = rects_shapes.ranges();
        let e_ranges: Vec<SquareRange> = ranges.iter().map(|a| a.delta(max)).collect();
        let paths: Vec<IntPath> = e_ranges.iter().map(|a| a.path().to_vec()).collect();

        let mut overlay = Overlay::new(float_shapes.points_count() + 4 * paths.len());
        overlay.add_shapes(&float_shapes, ShapeType::Subject);
        overlay.add_paths(&paths, ShapeType::Clip);

        let i_polys = overlay.into_graph(FillRule::NonZero).extract_shapes(OverlayRule::Intersect);
        let e_shapes = Self::connect(&e_ranges, &i_polys);

        let mut result = Vec::new();
        for i in 0..ranges.len() {
            let i_shapes = &e_shapes[i];
            let i_rect = &ranges[i];
            if i_shapes.is_empty() {
                if !self.only_inner {
                    result.push(ranges[i].path().to_vec())
                }
                continue;
            }

            if self.only_inner && !i_shapes.is_intersect(i_rect) {
                continue;
            }

            let bool0 = Self::test(i_rect, self.value_0, i_shapes);

            let is_pass = if self.is_two_values {
                let bool1 = Self::test(i_rect, self.value_1, i_shapes);

                self.bool_func.value_2x(bool0, bool1)
            } else {
                self.bool_func.value(bool0)
            };

            if !is_pass {
                result.push(i_rect.path().to_vec())
            }
        }

        result
    }

    fn test(rect: &SquareRange, value: i32, shapes: &Vec<IntShape>) -> RectBool {
        if value > 0 {
            let top = shapes.is_contain(&rect.top(value));
            let bottom = shapes.is_contain(&rect.bottom(value));
            let left = shapes.is_contain(&rect.left(value));
            let right = shapes.is_contain(&rect.right(value));

            RectBool { left, right, top, bottom }
        } else {
            let top = shapes.is_contain_any_width(&rect.top(value));
            let bottom = shapes.is_contain_any_width(&rect.bottom(value));
            let left = shapes.is_contain_any_height(&rect.left(value));
            let right = shapes.is_contain_any_height(&rect.right(value));

            RectBool { left, right, top, bottom }
        }
    }

    fn connect(e_ranges: &Vec<SquareRange>, i_polys: &Vec<IntShape>) -> Vec<Vec<IntShape>> {
        let mut id_pol_ranges = Vec::with_capacity(i_polys.len());

        for i in 0..i_polys.len() {
            id_pol_ranges.push(IdRange { index: i, range: SquareRange::with_shape(&i_polys[i]) });
        }
        id_pol_ranges.sort_by(|a, b| a.range.x_min.order_by_x(&b.range.x_min));

        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        let mut id_enc_ranges = Vec::with_capacity(e_ranges.len());

        for i in 0..e_ranges.len() {
            let range = &e_ranges[i];
            y_min = y_min.min(range.y_min);
            y_max = y_max.max(range.y_max);
            id_enc_ranges.push(IdRange { index: i, range: range.clone() });
        }
        id_enc_ranges.sort_by(|a, b| a.range.x_min.order_by_x(&b.range.x_min));

        let mut result: Vec<Vec<IntShape>> = vec![Vec::new(); e_ranges.len()];

        let space_range = LineRange { min: y_min, max: y_max };
        let mut scan_list: ScanSpace<usize, i32> = ScanSpace::new(space_range, e_ranges.len());
        let mut ids = Vec::new();

        let mut i = 0;
        let mut j = 0;
        while i < id_pol_ranges.len() {
            let p_range = &id_pol_ranges[i];

            while j < id_enc_ranges.len() && id_enc_ranges[j].range.x_min < p_range.range.x_max {
                let e_range = &id_enc_ranges[j];
                scan_list.insert(ScanSegment { id: j, range: e_range.range.y_range(), stop: e_range.range.x_max });
                j += 1;
            }

            let query_range = p_range.range.y_range().clamp(&space_range);

            scan_list.ids_in_range(query_range, p_range.range.x_min, &mut ids);
            if !ids.is_empty() {
                for &id in ids.iter() {
                    let e_range = &id_enc_ranges[id];
                    if p_range.range.is_overlap(&e_range.range) {
                        if let Some(shapes) = result.get_mut(e_range.index) {
                            let poly = &i_polys[p_range.index];
                            shapes.push(poly.clone());
                        }
                    }
                }

                ids.clear();
            }

            i += 1;
        }

        result
    }
}

trait RectShape {
    fn top(&self, delta: i32) -> SquareRange;
    fn bottom(&self, delta: i32) -> SquareRange;
    fn left(&self, delta: i32) -> SquareRange;
    fn right(&self, delta: i32) -> SquareRange;
    fn with_shape(shape: &IntShape) -> Self;
}

impl RectShape for SquareRange {
    fn top(&self, delta: i32) -> SquareRange {
        if delta > 0 {
            SquareRange {
                x_min: self.x_min,
                y_min: self.y_max,
                x_max: self.x_max,
                y_max: self.y_max + delta,
            }
        } else {
            let a = 1.max(-delta);
            SquareRange {
                x_min: self.x_min,
                y_min: self.y_max - a,
                x_max: self.x_max,
                y_max: self.y_max,
            }
        }
    }

    fn bottom(&self, delta: i32) -> SquareRange {
        if delta > 0 {
            SquareRange {
                x_min: self.x_min,
                y_min: self.y_min - delta,
                x_max: self.x_max,
                y_max: self.y_min,
            }
        } else {
            let a = 1.max(-delta);
            SquareRange {
                x_min: self.x_min,
                y_min: self.y_min,
                x_max: self.x_max,
                y_max: self.y_min + a,
            }
        }
    }

    fn left(&self, delta: i32) -> SquareRange {
        if delta > 0 {
            SquareRange {
                x_min: self.x_min - delta,
                y_min: self.y_min,
                x_max: self.x_min,
                y_max: self.y_max,
            }
        } else {
            let a = 1.max(-delta);
            SquareRange {
                x_min: self.x_min,
                y_min: self.y_min,
                x_max: self.x_min + a,
                y_max: self.y_max,
            }
        }
    }

    fn right(&self, delta: i32) -> SquareRange {
        if delta > 0 {
            SquareRange {
                x_min: self.x_max,
                y_min: self.y_min,
                x_max: self.x_max + delta,
                y_max: self.y_max,
            }
        } else {
            let a = 1.max(-delta);
            SquareRange {
                x_min: self.x_max - a,
                y_min: self.y_min,
                x_max: self.x_max,
                y_max: self.y_max,
            }
        }
    }

    fn with_shape(shape: &IntShape) -> Self {
        let mut x_min = i32::MAX;
        let mut y_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_max = i32::MIN;
        for p in shape[0].iter() {
            x_min = p.x.min(x_min);
            x_max = p.x.max(x_max);
            y_min = p.y.min(y_min);
            y_max = p.y.max(y_max);
        }

        Self {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }
}

trait ShapeBool {
    fn intersect(&self, rect: &SquareRange) -> Vec<IntShape>;
}

impl ShapeBool for IntShape {
    fn intersect(&self, rect: &SquareRange) -> Vec<IntShape> {
        let mut overlay = Overlay::new(2);
        overlay.add_shape(self, ShapeType::Subject);
        overlay.add_path(&rect.path().to_vec(), ShapeType::Clip);

        overlay.into_graph(FillRule::NonZero).extract_shapes(OverlayRule::Intersect)
    }
}

trait ShapeExt {
    fn is_intersect(&self, rect: &SquareRange) -> bool;
    fn is_contain(&self, rect: &SquareRange) -> bool;
    fn is_contain_any_width(&self, rect: &SquareRange) -> bool;
    fn is_contain_any_height(&self, rect: &SquareRange) -> bool;
    fn ranges(&self) -> Vec<SquareRange>;
}

impl ShapeExt for Vec<IntShape> {
    fn is_intersect(&self, rect: &SquareRange) -> bool {
        for shape in self.iter() {
            let i_rects = shape.intersect(rect);
            if !i_rects.is_empty() {
                return true;
            }
        }

        false
    }

    fn is_contain(&self, rect: &SquareRange) -> bool {
        for shape in self.iter() {
            let i_rects = shape.intersect(rect);
            if i_rects.len() == 1 && i_rects[0][0].unsafe_area() == 2 * rect.area() {
                return true;
            }
        }

        false
    }

    fn is_contain_any_width(&self, rect: &SquareRange) -> bool {
        for shape in self.iter() {
            let i_rects = shape.intersect(rect);
            for i_rect in i_rects.iter() {
                if i_rect[0].width() == rect.width() {
                    return true;
                }
            }
        }

        false
    }

    fn is_contain_any_height(&self, rect: &SquareRange) -> bool {
        for shape in self.iter() {
            let i_rects = shape.intersect(rect);
            for i_rect in i_rects.iter() {
                if i_rect[0].height() == rect.height() {
                    return true;
                }
            }
        }

        false
    }

    fn ranges(&self) -> Vec<SquareRange> {
        let mut result = Vec::with_capacity(self.len());

        for shape in self.iter() {
            if shape.len() == 1 {
                let path = &shape[0];
                if path.len() == 4 {
                    let mut x_min = path[3].x;
                    let mut y_min = path[3].y;
                    let mut x_max = x_min;
                    let mut y_max = y_min;

                    for i in 0..3 {
                        let x = path[i].x;
                        let y = path[i].y;
                        x_min = x.min(x_min);
                        x_max = x.max(x_max);
                        y_min = y.min(y_min);
                        y_max = y.max(y_max);
                    }

                    result.push(SquareRange { x_min, y_min, x_max, y_max });
                }
            }
        }

        result
    }
}

trait Size {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

impl Size for IntPath {
    fn width(&self) -> i32 {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;

        for p in self.iter() {
            x_min = x_min.min(p.x);
            x_max = x_max.max(p.x);
        }

        x_max - x_min
    }

    fn height(&self) -> i32 {
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;

        for p in self.iter() {
            y_min = y_min.min(p.y);
            y_max = y_max.max(p.y);
        }

        y_max - y_min
    }
}

trait XOrder {
    fn order_by_x(&self, other: &Self) -> Ordering;
}

impl XOrder for i32 {
    fn order_by_x(&self, other: &Self) -> Ordering {
        if self < other {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}