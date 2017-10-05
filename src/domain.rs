use na::Vector3;

trait Point2 {
    fn from_origin(&self) -> Vector2<f64>;
}

trait Point3 {
    fn from_origin(&self) -> Vector3<f64>;
}

trait SpatialTree<T : Point3> {
    fn find_closest_point(&self, p : Point3) -> T;
    fn find_closest_point_within_range(&self, p : Point3, range : f64) -> T;
    fn find_k_nearest_points(&self, p : Point3, k: u32) -> Box<Iterator<Item=T>>;
    fn find_k_nearest_points_within_range(&self, p : Point3, k : u32, range : f64) -> Box<Iterator<Item=T>>;
}