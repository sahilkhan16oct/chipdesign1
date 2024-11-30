use gds21::GdsPoint;
use i_overlay::i_float::f64_point::F64Point;
use i_overlay::i_float::point::IntPoint;
use i_overlay::i_shape::int::path::IntPath;

/// `UnitScaler` is responsible for converting between different coordinate systems:
/// raw file format coordinates, GDS (unit) coordinates, and user-defined coordinate systems.
/// It supports transformations for points and paths, allowing efficient scaling and conversion
/// across these systems.
///
/// # Fields
///
/// * `unit_size`: The size of a unit in the user-defined coordinate system. This is abstract value means nothing. It’s only purpose to reduce values.
/// * `inv_unit`: The inverse of the unit size, precalculated for efficiency.
/// * `db_size`: The database size, this is a size of 1 unit. This value needed to get a real value in meters.
#[derive(Debug, Clone, Copy)]
pub struct UnitScaler {
    pub unit_size: f64,
    pub inv_unit: f64,
    pub db_size: f64
}

// Example rectangle
// user units:   0.08 x 0.03
// units:           80 x 30
// meters:       0.000_000_08 x 0.000_000_03

impl UnitScaler {

    /// Constructs a new `UnitScaler`.
    ///
    /// # Arguments
    ///
    /// * `unit_size` - The size of a single unit in the user-defined coordinate system.
    /// * `db_size` - The database size (currently not used in scaling operations).
    ///
    /// # Returns
    ///
    /// A new instance of `UnitScaler`.
    pub fn new(unit_size: f64, db_size: f64) -> Self {
        Self { unit_size, inv_unit: 1.0 / unit_size, db_size }
    }

    /// Converts a single point from the raw coordinates to the unit coordinates.
    ///
    /// # Arguments
    ///
    /// * `vec` - in the raw(user) coordinate, with the format `vec![x, y]`.
    ///
    /// # Returns
    ///
    /// point in the unit coordinates.
    ///
    /// # Panics
    ///
    /// Panics if the input vector does not have exactly 2 elements.
    pub fn raw_point_to_unit(&self, vec: &[f64; 2]) -> IntPoint {
        assert_eq!(vec.len(), 2);
        let x = (self.inv_unit * vec[0]).round() as i32;
        let y = (self.inv_unit * vec[1]).round() as i32;
        IntPoint::new(x, y)
    }

    /// Converts points from the raw coordinates to the unit coordinates.
    ///
    /// # Arguments
    ///
    /// * `points` - in the raw(user) coordinates.
    ///
    /// # Returns
    ///
    /// path in the unit coordinates.
    pub fn raw_points_to_unit(&self, points: &Vec<[f64; 2]>) -> IntPath {
        points.iter().map(|p| self.raw_point_to_unit(p)).collect()
    }

    /// Converts paths from the raw(user) coordinates to the unit coordinates.
    ///
    /// # Arguments
    ///
    /// * `paths` - in the raw(user) coordinate system.
    ///
    /// # Returns
    ///
    /// paths in unit coordinate system.
    pub fn raw_paths_to_unit(&self, paths: &Vec<Vec<[f64; 2]>>) -> Vec<IntPath> {
        paths.iter().map(|p| self.raw_points_to_unit(p)).collect()
    }

    /// Converts a single GDS point to the unit coordinates.
    ///
    /// # Arguments
    ///
    /// * `point` - in GDS(unit) coordinates.
    ///
    /// # Returns
    ///
    /// A point in the unit coordinate.
    pub fn gds_point_to_unit(&self, point: &GdsPoint) -> IntPoint {
        IntPoint::new(point.x, point.y)
    }

    /// Converts GDS points to the unit coordinates.
    ///
    /// # Arguments
    ///
    /// * `points` - in GDS(unit) coordinates.
    ///
    /// # Returns
    ///
    /// A path in the unit coordinate.
    pub fn gds_points_to_unit(&self, points: &Vec<GdsPoint>) -> IntPath {
        points.iter().map(|p| self.gds_point_to_unit(p)).collect()
    }

    /// Converts a single point from unit coordinates to GDS(unit) coordinates.
    ///
    /// # Arguments
    ///
    /// * `point` - in the unit coordinates.
    ///
    /// # Returns
    ///
    /// A point in GDS(unit) coordinates.
    pub fn unit_point_to_gds(&self, point: &IntPoint) -> GdsPoint {
        GdsPoint::new(point.x, point.y)
    }

    /// Converts points from the unit coordinates to GDS(unit) coordinates.
    ///
    /// # Arguments
    ///
    /// * `points` - in the unit coordinates.
    ///
    /// # Returns
    ///
    /// points in GDS(unit) coordinates.
    pub fn unit_points_to_gds(&self, points: &Vec<IntPoint>) -> Vec<GdsPoint> {
        points.iter().map(|p| self.unit_point_to_gds(p)).collect()
    }

    /// Converts a single point from the unit coordinates to the raw coordinates.
    ///
    /// # Arguments
    ///
    /// * `point` - in the unit coordinates.
    ///
    /// # Returns
    ///
    /// A point in the raw(user) coordinates, with the format `vec![x, y]`.
    pub fn unit_point_to_raw(&self, point: &IntPoint) -> [f64; 2] {
        let x = self.unit_size * (point.x as f64);
        let y = self.unit_size * (point.y as f64);

        [x, y]
    }

    /// Converts points from the unit coordinates to the raw(user) coordinates.
    ///
    /// # Arguments
    ///
    /// * `points` - in the unit coordinates.
    ///
    /// # Returns
    ///
    /// points in the raw(user) coordinates.
    pub fn unit_points_to_raw(&self, points: &Vec<IntPoint>) -> Vec<[f64; 2]> {
        points.iter().map(|p| self.unit_point_to_raw(p)).collect()
    }

    /// Converts a value from user coordinates to the unit coordinates.
    ///
    /// # Arguments
    ///
    /// * `value` - in the user coordinates.
    ///
    /// # Returns
    ///
    /// A value in the unit coordinates.
    pub fn user_value_to_unit(&self, value: f64) -> i32 {
        (self.inv_unit * value).round() as i32
    }

    /// Converts a square value from the user coordinates to the unit coordinates.
    /// This is useful for area conversions or when working with squared dimensions.
    ///
    /// # Arguments
    ///
    /// * `value` - in user coordinates.
    ///
    /// # Returns
    ///
    /// A value in the unit coordinates.
    pub fn user_sqr_value_to_unit(&self, value: f64) -> i64 {
        (self.inv_unit * self.inv_unit * value).round() as i64
    }

    /// Converts a point from the user coordinates to the unit coordinates.
    ///
    /// # Arguments
    ///
    /// * `point` - in the user coordinates.
    ///
    /// # Returns
    ///
    /// A point in the unit coordinates.
    pub fn user_point_to_unit(&self, point: &F64Point) -> IntPoint {
        let x = self.user_value_to_unit(point.x);
        let y = self.user_value_to_unit(point.y);
        IntPoint::new(x, y)
    }

    /// Converts points from the user coordinates to the unit coordinates.
    ///
    /// # Arguments
    ///
    /// * `points` - in the user coordinates.
    ///
    /// # Returns
    ///
    /// points in the unit coordinates.
    pub fn user_points_to_unit(&self, points: &Vec<F64Point>) -> Vec<IntPoint> {
        points.iter().map(|p| self.user_point_to_unit(p)).collect()
    }

    /// Converts value from unit coordinates to user coordinates.
    ///
    /// # Arguments
    ///
    /// * `value` - in the unit coordinates.
    ///
    /// # Returns
    ///
    /// A value in user coordinate.
    pub fn unit_value_to_user(&self, value: i32) -> f64 {
        self.unit_size * (value as f64)
    }

    /// Converts point from the unit coordinates to the user coordinates.
    ///
    /// # Arguments
    ///
    /// * `value` - in the unit coordinates.
    ///
    /// # Returns
    ///
    /// A point in the user coordinates.
    pub fn unit_point_to_user(&self, point: &IntPoint) -> F64Point {
        let x = self.unit_value_to_user(point.x);
        let y = self.unit_value_to_user(point.y);
        F64Point::new(x, y)
    }

    /// Converts points from the unit coordinates to the user coordinates.
    ///
    /// # Arguments
    ///
    /// * `value` - in the unit coordinates.
    ///
    /// # Returns
    ///
    /// points in the user coordinates.
    pub fn unit_points_to_user(&self, points: &Vec<IntPoint>) -> Vec<F64Point> {
        points.iter().map(|p| self.unit_point_to_user(p)).collect()
    }
}
