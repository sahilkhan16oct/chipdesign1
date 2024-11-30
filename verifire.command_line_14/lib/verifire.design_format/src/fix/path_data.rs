use gds21::GdsPath;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{Overlay, ShapeType};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::i_float::f64_point::F64Point;
use i_overlay::i_float::point::IntPoint;
use i_overlay::i_shape::int::path::IntPath;
use i_overlay::i_shape::int::shape::IntShapes;
use crate::fix::layer::LayerData;
use crate::fix::scaler::UnitScaler;

pub struct PathData {
    pub layer: LayerData,
    pub points: IntPath,
    pub width: Option<i32>,
    path_type: Option<i16>,
    begin_extn: Option<i32>,
    end_extn: Option<i32>,
}

impl PathData {
    pub(crate) fn with_gds(gds: &GdsPath, scaler: UnitScaler) -> Self {
        let layer = LayerData { number: gds.layer as i32, datatype: gds.datatype as i32 };
        let points = scaler.gds_points_to_unit(&gds.xy);
        let width = gds.width;
        let path_type = gds.path_type;
        let begin_extn = gds.begin_extn;
        let end_extn = gds.end_extn;

        Self { layer, points, width, path_type, begin_extn, end_extn }
    }

    pub(crate) fn as_gds(&self, scaler: UnitScaler) -> GdsPath {
        let xy = scaler.unit_points_to_gds(&self.points);

        GdsPath {
            layer: self.layer.number as i16,
            datatype: self.layer.datatype as i16,
            xy,
            width: self.width,
            path_type: self.path_type,
            begin_extn: self.begin_extn,
            end_extn: self.end_extn,
            elflags: None,
            plex: None,
            properties: vec![],
        }
    }

    pub(crate) fn as_shapes(&self) -> IntShapes {
        let w = self.width.unwrap_or(0);
        if w == 0 || self.points.len() < 2 {
            return Vec::new();
        }
        let r = 0.5 * (w as f64);

        let (e0, e1) = match self.path_type.unwrap_or(0) {
            1 | 2 => {
                (r, r)
            }
            4 => {
                let e0 = self.begin_extn.unwrap_or(0) as f64;
                let e1 = self.end_extn.unwrap_or(0) as f64;
                (e0, e1)
            }
            _ => {
                (0.0, 0.0)
            }
        };


        let points = if self.points.is_plain() {
            &self.points
        } else {
            &self.points.make_plain()
        };

        if points.len() == 2 {
            return points.build_edge(r, e0, e1);
        }

        if points.is_simple(r) {
            points.build_simple(r, e0, e1)
        } else {
            points.build_composite(r, e0, e1)
        }
    }
}

struct PathBuilderUtil;

impl PathBuilderUtil {
    fn is_simple_joint(p0: IntPoint, p: IntPoint, p1: IntPoint, n0: F64Point, n1: F64Point, r: f64) -> bool {
        let dot = n0.dot_product(n1);

        if dot > 0.0 {
            return true;
        }

        let t0 = n0.tangent();
        let t1 = n1.tangent();

        let dot = n0.dot_product(n1);
        let k = r / (1.0 + dot);
        let m = (t0 + t1) * k;
        let mx = m.x.round() as i64;
        let my = m.y.round() as i64;

        let a = p0.sqr_distance(p);
        let b = p1.sqr_distance(p);
        let mm = mx * mx + my * my;
        let rr = (r * r) as i64;

        a + rr >= mm && b + rr >= mm
    }
}


trait PathBuilder {
    fn is_simple(&self, r: f64) -> bool;
    fn is_plain(&self) -> bool;
    fn make_plain(&self) -> IntPath;
    fn build_edge(&self, r: f64, e0: f64, e1: f64) -> IntShapes;
    fn build_simple(&self, r: f64, e0: f64, e1: f64) -> IntShapes;
    fn build_composite(&self, r: f64, e0: f64, e1: f64) -> IntShapes;
}

impl PathBuilder for IntPath {
    fn is_simple(&self, r: f64) -> bool {
        let mut p0 = self[0];
        let mut p1 = self[1];
        let mut n0 = VecUtil::direction(p0, p1);
        let mut ni: F64Point;
        let mut i = 2;
        while i < self.len() {
            let pi = self[i];
            ni = VecUtil::direction(p1, pi);
            if !PathBuilderUtil::is_simple_joint(p0, p1, pi, n0, ni, r) {
                return false;
            }
            n0 = ni;
            p0 = p1;
            p1 = pi;
            i += 1;
        }

        true
    }

    fn is_plain(&self) -> bool {
        let mut v0 = self[1].subtract(self[0]);
        for i in 2..self.len() {
            let vi = self[i].subtract(self[i - 1]);
            let cross = vi.cross_product(v0);
            if cross == 0 {
                return false;
            }
            v0 = vi;
        }

        true
    }
    fn make_plain(&self) -> IntPath {
        let mut result = Vec::with_capacity(self.len());
        let mut v0 = self[1].subtract(self[0]);
        result.push(self[0]);
        for i in 2..self.len() {
            let vi = self[i].subtract(self[i - 1]);
            let cross = vi.cross_product(v0);
            if cross != 0 {
                result.push(self[i - 1]);
            }
            v0 = vi;
        }
        result.push(self[self.len() - 1]);
        result
    }

    fn build_edge(&self, r: f64, e0: f64, e1: f64) -> IntShapes {
        let p0 = self[0];
        let p1 = self[1];
        let n = VecUtil::direction(p0, p1);
        let t = n.tangent();

        let ne0 = n * e0;
        let ne1 = n * e1;
        let tr = t * r;

        let q0 = p0 + IntPoint::new_f64_point(tr - ne0);
        let q1 = p0 - IntPoint::new_f64_point(tr + ne0);
        let q2 = p1 + IntPoint::new_f64_point(tr + ne1);
        let q3 = p1 - IntPoint::new_f64_point(tr - ne1);

        vec![vec![vec![q1, q0, q2, q3]]]
    }

    fn build_simple(&self, r: f64, e0: f64, e1: f64) -> IntShapes {
        let n = self.len();
        let mut path = SimplePath::new(n);

        let p0 = self[0];
        let mut p1 = self[1];
        let mut n0 = VecUtil::direction(p0, p1);
        let mut ni = F64Point::ZERO;

        path.end(p0, n0, r, -e0);

        for i in 2..n {
            let pi = self[i];
            ni = VecUtil::direction(p1, pi);
            path.add_joint(p1, n0, ni, r);
            n0 = ni;
            p1 = pi;
        }

        path.end(self[n - 1], ni, r, e1);

        path.to_shapes()
    }

    fn build_composite(&self, r: f64, e0: f64, e1: f64) -> IntShapes {
        let n = self.len();
        let mut path = CompositePath::new(n);

        let p0 = self[0];
        let mut p1 = self[1];
        let mut n0 = VecUtil::direction(p0, p1);
        let mut ni: F64Point;

        path.add_start(p0, p1, n0, r, e0);

        for i in 2..n - 1 {
            let pi = self[i];
            ni = VecUtil::direction(p1, pi);
            path.add_edge(p1, pi, ni, r);
            path.add_joint(p1, n0, ni, r);
            n0 = ni;
            p1 = pi;
        }

        let pe = self[n - 1];
        let ne = VecUtil::direction(p1, pe);
        path.add_end(p1, pe, ne, r, e1);
        path.add_joint(p1, n0, ne, r);

        path.to_shapes()
    }
}


struct CompositePath {
    overlay: Overlay,
}

impl CompositePath {
    fn new(count: usize) -> Self {
        let n = count - 1;
        Self { overlay: Overlay::new(count * 4 + n * 3) }
    }

    fn add_edge(&mut self, p: IntPoint, p1: IntPoint, n1: F64Point, r: f64) {
        let t1 = n1.tangent();

        let tr = IntPoint::new_f64_point(t1 * r);
        let q0 = p + tr;
        let q1 = p - tr;
        let q2 = p1 + tr;
        let q3 = p1 - tr;

        self.overlay.add_path(&[q1, q0, q2, q3], ShapeType::Subject);
    }

    fn add_start(&mut self, p0: IntPoint, p: IntPoint, n: F64Point, r: f64, e: f64) {
        let t = n.tangent();

        let ne = n * -e;
        let tr = t * r;

        let q0 = p0 + IntPoint::new_f64_point(ne + tr);
        let q1 = p0 + IntPoint::new_f64_point(ne - tr);
        let q2 = p + IntPoint::new_f64_point(tr);
        let q3 = p - IntPoint::new_f64_point(tr);

        self.overlay.add_path(&[q1, q0, q2, q3], ShapeType::Subject);
    }

    fn add_end(&mut self, p: IntPoint, p1: IntPoint, n: F64Point, r: f64, e: f64) {
        let t = n.tangent();

        let ne = n * e;
        let tr = t * r;

        let q0 = p + IntPoint::new_f64_point(tr);
        let q1 = p - IntPoint::new_f64_point(tr);
        let q2 = p1 + IntPoint::new_f64_point(ne + tr);
        let q3 = p1 + IntPoint::new_f64_point(ne - tr);

        self.overlay.add_path(&[q1, q0, q2, q3], ShapeType::Subject);
    }

    fn add_joint(&mut self, p: IntPoint, n0: F64Point, n1: F64Point, r: f64) {
        let t0 = n0.tangent();
        let t1 = n1.tangent();

        let cross = t0.cross_product(t1);

        let s = -cross.signum();

        let q0 = t0 * r * s;
        let q1 = t1 * r * s;

        let dot = t0.dot_product(t1);

        if dot < 0.0 {
            let p0 = p + IntPoint::new_f64_point(q0);
            let p00 = p + IntPoint::new_f64_point(q0 + n0 * r);
            let p11 = p + IntPoint::new_f64_point(q1 - n1 * r);
            let p1 = p + IntPoint::new_f64_point(q1);

            self.overlay.add_path(&[p0, p00, p11, p1, p], ShapeType::Subject);
        } else {
            let k = s * r / (1.0 + dot);
            let m = p + IntPoint::new_f64_point((t0 + t1) * k);
            let p0 = p + IntPoint::new_f64_point(q0);
            let p1 = p + IntPoint::new_f64_point(q1);

            self.overlay.add_path(&[p0, m, p1, p], ShapeType::Subject);
        }
    }

    fn to_shapes(self) -> IntShapes {
        self.overlay.into_graph(FillRule::NonZero).extract_shapes(OverlayRule::Subject)
    }
}


struct SimplePath {
    a_path: Vec<IntPoint>,
    b_path: Vec<IntPoint>,
}

impl SimplePath {
    fn new(count: usize) -> Self {
        Self {
            a_path: Vec::with_capacity(2 * count),
            b_path: Vec::with_capacity(2 * count),
        }
    }

    fn end(&mut self, p: IntPoint, n: F64Point, r: f64, e: f64) {
        let t = n.tangent();

        let ne = n * e;
        let tr = t * r;

        let a = p + IntPoint::new_f64_point(ne + tr);
        let b = p + IntPoint::new_f64_point(ne - tr);

        self.a_path.push(a);
        self.b_path.push(b);
    }

    fn add_joint(&mut self, p: IntPoint, n0: F64Point, n1: F64Point, r: f64) {
        let cross = n0.cross_product(n1);

        let t0 = n0.tangent();
        let t1 = n1.tangent();

        let dot = n0.dot_product(n1);
        let k = r / (1.0 + dot);
        let m = IntPoint::new_f64_point((t0 + t1) * k);

        if dot < 0.0 {
            let s = -cross.signum();
            let q0 = (t0 * s + n0) * r;
            let q1 = (t1 * s - n1) * r;

            if cross < 0.0 {
                self.b_path.push(p - m);
                self.a_path.push(p + IntPoint::new_f64_point(q0));
                self.a_path.push(p + IntPoint::new_f64_point(q1));
            } else {
                self.a_path.push(p + m);
                self.b_path.push(p + IntPoint::new_f64_point(q0));
                self.b_path.push(p + IntPoint::new_f64_point(q1));
            }
        } else {
            self.a_path.push(p + m);
            self.b_path.push(p - m);
        }
    }

    fn to_shapes(mut self) -> IntShapes {
        let mut path = self.a_path;
        self.b_path.reverse();
        path.append(&mut self.b_path);

        let mut overlay = Overlay::new(path.len());
        overlay.add_path(&path, ShapeType::Subject);
        overlay.into_graph(FillRule::NonZero).extract_shapes(OverlayRule::Subject)
    }
}

struct VecUtil;

impl VecUtil {
    fn direction(p0: IntPoint, p1: IntPoint) -> F64Point {
        let v = p1.subtract(p0);
        if v.x == 0 {
            let s = v.y.signum() as f64;
            F64Point { x: 0.0, y: s }
        } else if v.y == 0 {
            let s = v.x.signum() as f64;
            F64Point { x: s, y: 0.0 }
        } else {
            F64Point::new_i64(v.x, v.y).normalize()
        }
    }
}

trait VecExt {
    fn tangent(&self) -> Self;
}

impl VecExt for F64Point {
    fn tangent(&self) -> Self {
        F64Point { x: -self.y, y: self.x }
    }
}


#[cfg(test)]
mod tests {
    use i_overlay::i_float::point::IntPoint;
    use crate::fix::path_data::{PathBuilder, SimplePath, VecUtil};

    #[test]
    fn test_00() {
        let p0 = IntPoint::new(0, 0);
        let p1 = IntPoint::new(0, 2);
        let p2 = IntPoint::new(2, 2);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 1.0);

        assert_eq!(path.a_path[0], IntPoint::new(-1, 3));
        assert_eq!(path.b_path[0], IntPoint::new(1, 1));
    }

    #[test]
    fn test_01() {
        let p0 = IntPoint::new(0, 2);
        let p1 = IntPoint::new(2, 2);
        let p2 = IntPoint::new(2, 0);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 1.0);

        assert_eq!(path.a_path[0], IntPoint::new(3, 3));
        assert_eq!(path.b_path[0], IntPoint::new(1, 1));
    }

    #[test]
    fn test_02() {
        let p0 = IntPoint::new(2, 2);
        let p1 = IntPoint::new(2, 0);
        let p2 = IntPoint::new(0, 0);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 1.0);

        assert_eq!(path.a_path[0], IntPoint::new(3, -1));
    }

    #[test]
    fn test_03() {
        let p0 = IntPoint::new(2, 0);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(0, 2);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 1.0);

        assert_eq!(path.a_path[0], IntPoint::new(-1, -1));
    }


    #[test]
    fn test_10() {
        let p0 = IntPoint::new(2, -4);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(2, 4);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 2.0 * 5_f64.sqrt());

        assert_eq!(path.a_path[0], IntPoint::new(-5, 0));
    }

    #[test]
    fn test_11() {
        let p0 = IntPoint::new(-4, -2);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(4, -2);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 2.0 * 5_f64.sqrt());

        assert_eq!(path.a_path[0], IntPoint::new(0, 5));
    }

    #[test]
    fn test_12() {
        let p0 = IntPoint::new(-2, 4);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(-2, -4);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 2.0 * 5_f64.sqrt());

        assert_eq!(path.a_path[0], IntPoint::new(5, 0));
    }

    #[test]
    fn test_13() {
        let p0 = IntPoint::new(4, 2);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(-4, 2);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 2.0 * 5_f64.sqrt());

        assert_eq!(path.a_path[0], IntPoint::new(0, -5));
    }

    #[test]
    fn test_20() {
        let p0 = IntPoint::new(0, -2);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(2, 0);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 1.0);

        assert_eq!(path.a_path[0], IntPoint::new(-1, 1));
    }

    #[test]
    fn test_21() {
        let p0 = IntPoint::new(-2, 0);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(0, -2);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 1.0);

        assert_eq!(path.a_path[0], IntPoint::new(1, 1));
    }

    #[test]
    fn test_22() {
        let p0 = IntPoint::new(0, 2);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(-2, 0);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 1.0);

        assert_eq!(path.a_path[0], IntPoint::new(1, -1));
    }

    #[test]
    fn test_23() {
        let p0 = IntPoint::new(2, 0);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(0, 2);

        let mut path = SimplePath::new(3);
        let n0 = VecUtil::direction(p0, p1);
        let n1 = VecUtil::direction(p1, p2);

        path.add_joint(p1, n0, n1, 1.0);

        assert_eq!(path.a_path[0], IntPoint::new(-1, -1));
    }


    #[test]
    fn test_30() {
        let p0 = IntPoint::new(0, -4);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(4, 0);

        let path = vec![p0, p1, p2];
        let shapes = path.build_composite(1.0, 2.0, 2.0);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);

        let points = &shapes[0][0];
        assert_eq!(points.len(), 6);
        assert_eq!(points,
                   &vec![
                       IntPoint::new(-1, -6),
                       IntPoint::new(-1, 1),
                       IntPoint::new(6, 1),
                       IntPoint::new(6, -1),
                       IntPoint::new(1, -1),
                       IntPoint::new(1, -6),
                   ]
        );
    }

    #[test]
    fn test_31() {
        let p0 = IntPoint::new(-4, 0);
        let p1 = IntPoint::new(0, 0);
        let p2 = IntPoint::new(0, -4);

        let path = vec![p0, p1, p2];
        let shapes = path.build_composite(1.0, 2.0, 2.0);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);

        let points = &shapes[0][0];
        assert_eq!(points.len(), 6);
        assert_eq!(points,
                   &vec![
                       IntPoint::new(-6, -1),
                       IntPoint::new(-6, 1),
                       IntPoint::new(1, 1),
                       IntPoint::new(1, -6),
                       IntPoint::new(-1, -6),
                       IntPoint::new(-1, -1),
                   ]
        );
    }

    #[test]
    fn test_32() {
        let path = vec![
            IntPoint::new(-2, -4),
            IntPoint::new(0, 0),
            IntPoint::new(2, -4),
        ];
        let shapes = path.build_composite(5.0f64.sqrt(), 5.0f64.sqrt(), 5.0f64.sqrt());

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);

        let points = &shapes[0][0];
        assert_eq!(points.len(), 7);
        assert_eq!(points,
                   &vec![

                       IntPoint::new(-5, -5),
                       IntPoint::new(-1, 3),
                       IntPoint::new(1, 3),
                       IntPoint::new(5, -5),
                       IntPoint::new(1, -7),
                       IntPoint::new(0, -5),
                       IntPoint::new(-1, -7)
                   ]
        );
    }

    #[test]
    fn test_33() {
        let path = vec![
            IntPoint::new(-8, -4),
            IntPoint::new(0, 0),
            IntPoint::new(8, -4),
        ];
        let shapes = path.build_composite(2.0 * 5.0f64.sqrt(), 0.0f64, 0.0f64);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);

        let points = &shapes[0][0];
        assert_eq!(points.len(), 6);
        assert_eq!(points,
                   &vec![

                       IntPoint::new(-10, 0),
                       IntPoint::new(0, 5),
                       IntPoint::new(10, 0),
                       IntPoint::new(6, -8),
                       IntPoint::new(0, -5),
                       IntPoint::new(-6, -8)
                   ]
        );
    }

    #[test]
    fn test_34() {
        let path = vec![
            IntPoint::new(0, 0),
            IntPoint::new(10, 0),
        ];
        assert_eq!(path.is_simple(1.0), true);
        assert_eq!(path.is_plain(), true);
        let shapes = path.build_edge(1.0, 2.0, 2.0);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);

        let points = &shapes[0][0];
        assert_eq!(points.len(), 4);
        assert_eq!(points,
                   &vec![
                       IntPoint::new(-2, -1),
                       IntPoint::new(-2, 1),
                       IntPoint::new(12, 1),
                       IntPoint::new(12, -1)
                   ]
        );
    }
}