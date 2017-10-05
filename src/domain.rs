use na::Vector3;

/// A two dimensional point with a defined vector from the origin
trait Point2 {
    fn from_origin(&self) -> Vector2<f64>;
}

/// A three dimensional point with a defined vector from the origin
trait Point3 {
    fn from_origin(&self) -> Vector3<f64>;
}

/// A three dimensional plane defined by a point3 and a vector3
struct Plane3 { 
    position : Point3, 
    normal : Vector3<f64> 
}

/// An approach capable of answering spatial queries
trait SpatialQueryStrategy<T : Point3> {
    fn find_closest_point(&self, p : Point3) -> T;
    fn find_closest_point_within_range(&self, p : Point3, range : f64) -> T;
    fn find_k_nearest_points(&self, p : Point3, k: u32) -> Box<Iterator<Item=T>>;
    fn find_k_nearest_points_within_range(&self, p : Point3, k : u32, range : f64) -> Box<Iterator<Item=T>>;
}