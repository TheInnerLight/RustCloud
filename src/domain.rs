use na::{DefaultAllocator, Dim, Vector3, VectorN, U1, U2, U3};
use na::allocator::Allocator;

/// An n-dimensional point with a defined n-vector from the origin
pub trait NPoint<N : Dim> where DefaultAllocator: Allocator<f64, N> {
    fn from_origin(&self) -> VectorN<f64, N>;
}

/// A two dimensional point with a defined vector from the origin
pub type Point2 = NPoint<U2>;

/// A three dimensional point with a defined vector from the origin
pub type Point3 = NPoint<U3>;

/// A three dimensional plane defined by a point3 and a vector3
pub struct Plane3 { 
    position : Vector3<f64>, 
    normal : Vector3<f64> 
}

/// An arbitrary structure of points capable of answering spatial queries
pub trait SpatialQueryStructure<T : NPoint<U3>> {
    fn find_closest_point(&self, p : Point3) -> T;
    fn find_closest_point_within_range(&self, p : Point3, range : f64) -> T;
    fn find_k_nearest_points(&self, p : Point3, k: u32) -> Box<Iterator<Item=T>>;
    fn find_k_nearest_points_within_range(&self, p : Point3, k : u32, range : f64) -> Box<Iterator<Item=T>>;
}