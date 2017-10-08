use na::{DefaultAllocator, DimName, VectorN, U2, U3};
use na::allocator::Allocator;

/// An n-dimensional point with a defined n-vector from the origin
pub trait NPoint: Clone
where
    DefaultAllocator: Allocator<f64, Self::N>,
{
    type N: DimName;

    fn from_origin(&self) -> &VectorN<f64, Self::N>;

    fn sq_distance_to<T : NPoint<N = Self::N>>(&self, pt2 : T) -> f64 {
        (self.from_origin() - pt2.from_origin()).norm_squared()
    }
}

/// A two dimensional point with a defined vector from the origin
pub type Point2 = NPoint<N = U2>;

/// A three dimensional point with a defined vector from the origin
pub type Point3 = NPoint<N = U3>;

/// An n-dimensional plane defined by a vector from the origin and a normal vector
pub struct Hyperplane<N : DimName> 
where
    DefaultAllocator: Allocator<f64, N>
{ 
    pub position : VectorN<f64, N>, 
    pub normal : VectorN<f64, N> 
}

impl<N : DimName> Hyperplane<N> where
    DefaultAllocator: Allocator<f64, N> 
{
    fn distance_to_point<T : NPoint<N = N>> (&self, pt : T) -> f64 {
        let distance_to_plane_point = pt.from_origin() - &self.position;
        self.normal.dot(&distance_to_plane_point)
    }
}

/// An arbitrary structure of points capable of answering spatial queries
pub trait SpatialQueryStructure<T : NPoint>
where
    DefaultAllocator: Allocator<f64, T::N> 
{
    fn find_closest_point<P : NPoint<N = T::N>>(&self, p : P) -> T;
    fn find_closest_point_within_range<P : NPoint<N = T::N>>(&self, p : P, range : f64) -> T;
    fn find_k_nearest_points<P : NPoint<N = T::N>>(&self, p : P, k: u32) -> Box<Iterator<Item=T>>;
    fn find_k_nearest_points_within_range<P : NPoint<N = T::N>>(&self, p : P, k : u32, range : f64) -> Box<Iterator<Item=T>>;
}