use na::{DefaultAllocator, Dim, VectorN, U1, U2, U3};
use na::allocator::Allocator;
use domain::NPoint;

/// A kd tree
pub struct KdTree<N : Dim>(Box<kd_tree::KdTreeImpl<N>>) where DefaultAllocator: Allocator<f64, N>;

impl<N : Dim> KdTree<N> where DefaultAllocator: Allocator<f64, N> {
    /// Get all the points from the kd-tree
    fn get_points(self) -> Vec<Box<NPoint<N>>>  {
        let mut pts = Vec::new();
        let KdTree(tree) = self;
        kd_tree::add_points_to_vector(tree, &mut pts);
        pts
    }

    fn build<TItem : NPoint<N> + Copy + 'static>(pts : &Vec<TItem>) -> Self where DefaultAllocator: Allocator<f64, N> {
        KdTree(Box::new(kd_tree::build(pts, 0)))
    }
}

mod kd_tree {
    use na::{DefaultAllocator, Dim, Vector3, VectorN, U1, U2, U3};
    use na::allocator::Allocator;
    use std::cmp::Ordering;
    use domain::NPoint;
    use super::{KdTree};

    /// kd-Tree implementation structure
    pub(super) enum KdTreeImpl<N : Dim> where DefaultAllocator: Allocator<f64, N> {
        Node(usize, Box<KdTreeImpl<N>>, Box<NPoint<N>>, Box<KdTreeImpl<N>>),
        Empty()
    }

    /// mutably enunerate points from a kd tree into a vector
    pub(super) fn add_points_to_vector<N : Dim>(tree : Box<KdTreeImpl<N>>, pts : &mut Vec<Box<NPoint<N>>>) where DefaultAllocator: Allocator<f64, N> {
        let unboxed_tree = *tree;
        match unboxed_tree {
            KdTreeImpl::Node(_, l, pt, r) => {
                add_points_to_vector(l, pts);
                pts.push(pt);
                add_points_to_vector(r, pts);
            },
            KdTreeImpl::Empty() => {}
        }
    }

    fn split_vec_with_median<N : Dim, TItem : NPoint<N> + Copy + 'static>(pts : &Vec<TItem>, dim_idx :usize) -> (Option<Vec<TItem>>, Option<TItem>, Option<Vec<TItem>>) where DefaultAllocator: Allocator<f64, N> {
        unsafe {
            let mut pts = pts.to_vec();
            pts.sort_by( |a,b| a.from_origin().get_unchecked(0, dim_idx).partial_cmp(b.from_origin().get_unchecked(0, dim_idx)).unwrap() );
            if pts.len() >= 3 {
                let length = pts.len();
                let mut middle = pts.split_off(length / 2);
                let right = middle.split_off(1);
                (Some(pts), Some(middle[0]), Some(right))
            } else if pts.len() == 2 {
                let length = pts.len();
                let mut middle = pts.split_off(1);
                (Some(pts), Some(middle[0]), None)
            } else if pts.len() == 1 {
                (None, Some(pts[0]), None)
            } else{
                (None, None, None)
            }
        }
    }

    pub(super) fn build<N : Dim, TItem : NPoint<N> + Copy + 'static>(pts : &Vec<TItem>, dimIdx :usize) -> KdTreeImpl<N> where DefaultAllocator: Allocator<f64, N> {
        match pts.first() {
            Some(first_pt) => {
                let next_dim_idx = if dimIdx + 1 >= first_pt.from_origin().ncols() {0} else {dimIdx + 1};
                match split_vec_with_median(pts, dimIdx) {
                    (Some(vec1), Some(pt), Some(vec2))  => KdTreeImpl::Node(next_dim_idx,     Box::new(build(&vec1, next_dim_idx)),   Box::new(pt),     Box::new(build(&vec2, next_dim_idx))),
                    (Some(vec1), Some(pt), None      )  => KdTreeImpl::Node(next_dim_idx,     Box::new(build(&vec1, next_dim_idx)),   Box::new(pt),     Box::new(KdTreeImpl::Empty())),
                    (None,       Some(pt), Some(vec2))  => KdTreeImpl::Node(next_dim_idx,     Box::new(KdTreeImpl::Empty()),        Box::new(pt),     Box::new(build(&vec2, next_dim_idx))),
                    (None,       Some(pt), None      )  => KdTreeImpl::Node(next_dim_idx,     Box::new(KdTreeImpl::Empty()),        Box::new(pt),     Box::new(KdTreeImpl::Empty())),
                    (None,       None,     None      )  => panic!("Should be impossible to have a first point but still split into (None, None, None)."),
                    (_      ,    None,     _         )  => panic!("Meaningless result: point must be contained in a kd-tree node.")
                }
            }
            None => {
                KdTreeImpl::Empty()
            }
        }
    }
}
