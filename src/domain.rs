use vector::Vector3;

trait Point3 {
    fn from_origin(&self) -> Vector3;
}

trait SpatialTree<T : Point3> {
    fn find_closest_point(&self, p : Point3) -> T;
    fn find_closest_point_within_range(&self, p : Point3, range : f64) -> T;
    fn find_k_nearest_points(&self, p : Point3, k: u32) -> Iterator<Item=T>;
    fn find_k_nearest_points_within_range(&self, p : Point3, k : u32, range : f64) -> Iterator<Item=T>;
}