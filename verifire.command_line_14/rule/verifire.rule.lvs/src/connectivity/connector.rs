use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_float::triangle::Triangle;
use foundation::i_overlay::i_shape::int::path::IntPath;
use crate::connectivity::layer::{Bundle, Piece, StackLayer};
use crate::connectivity::rect::Rect;

pub struct Connector;

impl Connector {
    pub(super) fn find_connections(a_layer: &StackLayer, b_layer: &StackLayer) -> Vec<[usize; 2]> {
        let mut result = Vec::new();

        let mut ia = 0;
        let mut ib = 0;
        while ia < a_layer.bundles.len() && ib < b_layer.bundles.len() {
            let a = &a_layer.bundles[ia];

            // scroll b shapes to scan line
            while ib < b_layer.bundles.len() && b_layer.bundles[ib].rect.max_x < a.rect.min_x {
                ib += 1;
            }

            if ib == b_layer.bundles.len() {
                break;
            }

            let b = &b_layer.bundles[ib];

            // scroll a shapes to scan line
            while ia < a_layer.bundles.len() && a_layer.bundles[ia].rect.max_x < b.rect.min_x {
                ia += 1;
            }

            if ia == a_layer.bundles.len() {
                break;
            }

            let a = &a_layer.bundles[ia];
            let mut i = ib;

            while i < b_layer.bundles.len() {
                let b = &b_layer.bundles[i];
                if b.rect.min_x > a.rect.max_x {
                    break;
                }

                if a.rect.min_x > b.rect.max_x {
                    i += 1;
                    continue;
                }

                if b.collide(a) {
                    result.push([ia, i]);
                }

                i += 1;
            }

            ia += 1;
        }

        result
    }
}

impl Bundle {
    fn collide(&self, other: &Self) -> bool {
        #[cfg(debug_assertions)]
        {
            let x = self.rect.collides_x(&other.rect);
            debug_assert!(x);
        }

        if !self.rect.collides_y(&other.rect) {
            return false;
        }

        let mut ia = 0;
        let mut ib = 0;

        while ia < self.pieces.len() && ib < other.pieces.len() {
            let a = &self.pieces[ia];

            // scroll a pieces to scan line
            while ib < other.pieces.len() && other.pieces[ib].rect.max_x < a.rect.min_x {
                ib += 1
            }

            if ib == other.pieces.len() {
                break;
            }

            let b = &other.pieces[ib];

            // scroll a pieces to scan line
            while ia < self.pieces.len() && self.pieces[ia].rect.max_x < b.rect.min_x {
                ia += 1
            }

            if ia == self.pieces.len() {
                break;
            }

            let a = &self.pieces[ia];
            let mut i = ib;

            while i < other.pieces.len() {
                let b = &other.pieces[i];
                i += 1;
                if b.rect.min_x > a.rect.max_x {
                    break;
                }

                if a.rect.min_x > b.rect.max_x {
                    continue;
                }

                if a.collide(&b) {
                    return true;
                }
            }

            ia += 1
        }

        false
    }
}

impl Piece {
    fn collide(&self, other: &Self) -> bool {
        #[cfg(debug_assertions)]
        {
            let x = self.rect.collides_x(&other.rect);
            debug_assert!(x);
        }

        let y = self.rect.collides_y(&other.rect);
        let is_both_rect = self.is_rect && other.is_rect;
        y && (is_both_rect || Self::collide_path_and_path(&self.path, &other.path))
    }

    fn collide_path_and_path(a_path: &IntPath, b_path: &IntPath) -> bool {
        let mut a0 = a_path[a_path.len() - 1];
        for &a1 in a_path.iter() {
            let mut b0 = b_path[b_path.len() - 1];
            for &b1 in b_path.iter() {
                if Self::collide_segments(a0, a1, b0, b1) {
                    return true;
                }
                b0 = b1;
            }

            a0 = a1;
        }

        false
    }

    fn collide_segments(a0: IntPoint, b0: IntPoint, a1: IntPoint, b1: IntPoint) -> bool {
        let test_x = a0.x > a1.x && a0.x > b1.x
            && b0.x > a1.x && b0.x > b1.x
            || a0.x < a1.x && a0.x < b1.x
            && b0.x < a1.x && b0.x < b1.x;

        if test_x {
            return false;
        }

        let test_y = a0.y > a1.y && a0.y > b1.y
            && b0.y > a1.y && b0.y > b1.y
            || a0.y < a1.y && a0.y < b1.y
            && b0.y < a1.y && b0.y < b1.y;

        if test_y {
            return false;
        }

        let a0b0a1 = Triangle::clock_direction_point(a0, b0, a1);
        let a0b0b1 = Triangle::clock_direction_point(a0, b0, b1);

        let a1b1a0 = Triangle::clock_direction_point(a1, b1, a0);
        let a1b1b0 = Triangle::clock_direction_point(a1, b1, b0);

        let is_collinear = (a0b0a1 | a0b0b1 | a1b1a0 | a1b1b0) == 0;

        a0b0a1 != a0b0b1 && a1b1a0 != a1b1b0 || is_collinear
    }
}

#[cfg(test)]
mod tests {
    use foundation::i_overlay::i_float::point::IntPoint;
    use crate::connectivity::connector::Connector;
    use crate::connectivity::layer::{Piece, StackLayer};

    #[test]
    fn test_collide_seg_0() {
        let a0 = IntPoint::new(0, -1);
        let a1 = IntPoint::new(-1, 0);
        let b0 = IntPoint::new(-3, 2);
        let b1 = IntPoint::new(-1, 2);

        assert_eq!(Piece::collide_segments(a0, a1, b0, b1), false);
    }

    #[test]
    fn test_0() {
        let a_shapes = [
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 5),
                    IntPoint::new(5, 5),
                    IntPoint::new(5, 0)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(20, 0),
                    IntPoint::new(20, 5),
                    IntPoint::new(25, 5),
                    IntPoint::new(25, 0)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let b_shapes = [
            [
                [
                    IntPoint::new(10, 0),
                    IntPoint::new(10, 5),
                    IntPoint::new(15, 5),
                    IntPoint::new(15, 0)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);


        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_1() {
        let a_shapes = [
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 5),
                    IntPoint::new(5, 5),
                    IntPoint::new(5, 0)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(20, 0),
                    IntPoint::new(20, 5),
                    IntPoint::new(25, 5),
                    IntPoint::new(25, 0)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let b_shapes = [
            [
                [
                    IntPoint::new(5, 0),
                    IntPoint::new(5, 5),
                    IntPoint::new(15, 5),
                    IntPoint::new(15, 0)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);


        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_2() {
        let a_shapes = [
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 5),
                    IntPoint::new(5, 5),
                    IntPoint::new(5, 0)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(20, 0),
                    IntPoint::new(20, 5),
                    IntPoint::new(25, 5),
                    IntPoint::new(25, 0)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let b_shapes = [
            [
                [
                    IntPoint::new(5, 0),
                    IntPoint::new(5, 5),
                    IntPoint::new(25, 5),
                    IntPoint::new(25, 0)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);


        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_3() {
        let a_shapes = [
            [
                [
                    IntPoint::new(-3, 0),
                    IntPoint::new(-1, -2),
                    IntPoint::new(-4, -2),
                    IntPoint::new(-4, 2),
                    IntPoint::new(-1, 2),
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(3, 0),
                    IntPoint::new(1, 2),
                    IntPoint::new(4, 2),
                    IntPoint::new(4, -2),
                    IntPoint::new(1, -2),
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let b_shapes = [
            [
                [
                    IntPoint::new(-1, 0),
                    IntPoint::new(0, 1),
                    IntPoint::new(1, 0),
                    IntPoint::new(0, -1)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);


        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_4() {
        let a_shapes = [
            [
                [
                    IntPoint::new(-3, -1),
                    IntPoint::new(-3, 1),
                    IntPoint::new(-1, 1),
                    IntPoint::new(-1, -1)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(1, -1),
                    IntPoint::new(1, 1),
                    IntPoint::new(3, 1),
                    IntPoint::new(3, -1)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(-2, 1),
                    IntPoint::new(-2, 3),
                    IntPoint::new(0, 3),
                    IntPoint::new(0, 1)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(0, 1),
                    IntPoint::new(0, 3),
                    IntPoint::new(2, 3),
                    IntPoint::new(2, 1)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(-2, -3),
                    IntPoint::new(-2, -1),
                    IntPoint::new(0, -1),
                    IntPoint::new(0, -3)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(0, -3),
                    IntPoint::new(0, -1),
                    IntPoint::new(2, -1),
                    IntPoint::new(2, -3)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let b_shapes = [
            [
                [
                    IntPoint::new(-1, 0),
                    IntPoint::new(0, 1),
                    IntPoint::new(1, 0),
                    IntPoint::new(0, -1)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);

        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_5() {
        let a_shapes = [
            [
                [
                    IntPoint::new(-3, -3),
                    IntPoint::new(-3, 3),
                    IntPoint::new(3, 3),
                    IntPoint::new(3, -3)
                ].to_vec(),
                [
                    IntPoint::new(-2, -2),
                    IntPoint::new(2, -2),
                    IntPoint::new(2, 2),
                    IntPoint::new(-2, 2)
                ].to_vec()
            ].to_vec(),
        ].to_vec();

        let b_shapes = [
            [
                [
                    IntPoint::new(-1, -1),
                    IntPoint::new(-1, 1),
                    IntPoint::new(1, 1),
                    IntPoint::new(1, -1)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);

        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_6() {
        let a_shapes = [
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 1),
                    IntPoint::new(2, 1),
                    IntPoint::new(2, 0)
                ].to_vec(),
            ].to_vec(),
            [
                [
                    IntPoint::new(4, 0),
                    IntPoint::new(4, 1),
                    IntPoint::new(6, 1),
                    IntPoint::new(6, 0)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(8, 0),
                    IntPoint::new(8, 1),
                    IntPoint::new(10, 1),
                    IntPoint::new(10, 0)
                ].to_vec()
            ].to_vec(),
        ].to_vec();
        let b_shapes = [
            [
                [
                    IntPoint::new(2, 0),
                    IntPoint::new(2, 2),
                    IntPoint::new(4, 2),
                    IntPoint::new(4, 0)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(6, 0),
                    IntPoint::new(6, 2),
                    IntPoint::new(8, 2),
                    IntPoint::new(8, 0)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);

        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_7() {
        let a_shapes = [
            [
                [
                    IntPoint::new(-2, -2),
                    IntPoint::new(-2, 2),
                    IntPoint::new(2, 2),
                    IntPoint::new(2, -2)
                ].to_vec(),
            ].to_vec(),
        ].to_vec();
        let b_shapes = [
            [
                [
                    IntPoint::new(-1, -1),
                    IntPoint::new(-1, 1),
                    IntPoint::new(1, 1),
                    IntPoint::new(1, -1)
                ].to_vec()
            ].to_vec(),
        ].to_vec();

        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);

        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_8() {
        let a_shapes = [
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 1),
                    IntPoint::new(5, 1),
                    IntPoint::new(5, 3),
                    IntPoint::new(2, 3),
                    IntPoint::new(2, 4),
                    IntPoint::new(5, 4),
                    IntPoint::new(5, 6),
                    IntPoint::new(0, 6),
                    IntPoint::new(0, 7),
                    IntPoint::new(6, 7),
                    IntPoint::new(6, 0)
                ].to_vec(),
            ].to_vec(),
        ].to_vec();
        let b_shapes = [
            [
                [
                    IntPoint::new(1, 2),
                    IntPoint::new(1, 5),
                    IntPoint::new(4, 5),
                    IntPoint::new(4, 2)
                ].to_vec()
            ].to_vec(),
        ].to_vec();

        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);

        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_9() {
        let a_shapes = [
            [
                [
                    IntPoint::new(5950, 1510),
                    IntPoint::new(5950, 1760),
                    IntPoint::new(7400, 1760),
                    IntPoint::new(7400, 3595),
                    IntPoint::new(6900, 3595),
                    IntPoint::new(6900, 3930),
                    IntPoint::new(6530, 3930),
                    IntPoint::new(6530, 5025),
                    IntPoint::new(5690, 5025),
                    IntPoint::new(5690, 5275),
                    IntPoint::new(6710, 5275),
                    IntPoint::new(6710, 4185),
                    IntPoint::new(7130, 4185),
                    IntPoint::new(7130, 5025),
                    IntPoint::new(7300, 5195),
                    IntPoint::new(7300, 6160),
                    IntPoint::new(7480, 6160),
                    IntPoint::new(7480, 5060),
                    IntPoint::new(7310, 4890),
                    IntPoint::new(7310, 4185),
                    IntPoint::new(7580, 4185),
                    IntPoint::new(7580, 1510)
                ].to_vec(),
            ].to_vec(),
        ].to_vec();
        let b_shapes = [
            [
                [
                    IntPoint::new(6170, 670),
                    IntPoint::new(6170, 2715),
                    IntPoint::new(6610, 2715),
                    IntPoint::new(6610, 670),
                ].to_vec()
            ].to_vec(),
        ].to_vec();

        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);

        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_10() {
        let a_shapes = [
            [
                [
                    IntPoint::new(16770, 3735),
                    IntPoint::new(16770, 4325),
                    IntPoint::new(16790, 4325),
                    IntPoint::new(16790, 6535),
                    IntPoint::new(19220, 6535),
                    IntPoint::new(19220, 4085),
                    IntPoint::new(19040, 4085),
                    IntPoint::new(19040, 6285),
                    IntPoint::new(16970, 6285),
                    IntPoint::new(16970, 4325),
                    IntPoint::new(17190, 4325),
                    IntPoint::new(17190, 3990),
                    IntPoint::new(17840, 3990),
                    IntPoint::new(17840, 3065),
                    IntPoint::new(18370, 3065),
                    IntPoint::new(18370, 740),
                    IntPoint::new(22430, 740),
                    IntPoint::new(22430, 2715),
                    IntPoint::new(22130, 2715),
                    IntPoint::new(22130, 3300),
                    IntPoint::new(22610, 3300),
                    IntPoint::new(22610, 490),
                    IntPoint::new(18190, 490),
                    IntPoint::new(18190, 2810),
                    IntPoint::new(17660, 2810),
                    IntPoint::new(17660, 3735),
                    IntPoint::new(17070, 3735),
                    IntPoint::new(17070, 1480),
                    IntPoint::new(16890, 1480),
                    IntPoint::new(16890, 3735)
                ].to_vec(),
            ].to_vec(),
        ].to_vec();

        let b_shapes = [
            [
                [
                    IntPoint::new(15840, 4660),
                    IntPoint::new(15840, 4880),
                    IntPoint::new(16060, 4880),
                    IntPoint::new(16060, 4660)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(15940, 2730),
                    IntPoint::new(15940, 2950),
                    IntPoint::new(16160, 2950),
                    IntPoint::new(16160, 2730)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(16870, 3920),
                    IntPoint::new(16870, 4140),
                    IntPoint::new(17090, 4140),
                    IntPoint::new(17090, 3920)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(18540, 2240),
                    IntPoint::new(18540, 2460),
                    IntPoint::new(18760, 2460),
                    IntPoint::new(18760, 2240)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(19390, 4620),
                    IntPoint::new(19390, 4840),
                    IntPoint::new(19610, 4840),
                    IntPoint::new(19610, 4620)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(20690, 4620),
                    IntPoint::new(20690, 4840),
                    IntPoint::new(20910, 4840),
                    IntPoint::new(20910, 4620)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(22000, 5515),
                    IntPoint::new(22000, 5735),
                    IntPoint::new(22220, 5735),
                    IntPoint::new(22220, 5515)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(22230, 2900),
                    IntPoint::new(22230, 3120),
                    IntPoint::new(22450, 3120),
                    IntPoint::new(22450, 2900)
                ].to_vec()
            ].to_vec(),
        ].to_vec();
        let a_layer = StackLayer::new(a_shapes);
        let b_layer = StackLayer::new(b_shapes);

        let result = Connector::find_connections(&a_layer, &b_layer);

        assert_eq!(result.len(), 2);
    }
}