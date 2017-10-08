use na::{DefaultAllocator};
use na::allocator::Allocator;
use domain::{Hyperplane, NPoint};

/// A kd tree
pub struct KdTree<P : NPoint>(kd_tree::KdTreeImpl<P>) where DefaultAllocator: Allocator<f64, P::N> ;

impl<P : NPoint> KdTree<P> where DefaultAllocator: Allocator<f64, P::N> {
    /// Get all the points from the kd-tree
    fn get_points(&self) -> Vec<P>  {
        let mut pts = Vec::new();
        let &KdTree(ref tree) = self;
        kd_tree::add_points_to_vector(tree, &mut pts);
        pts
    }

    fn build(pts : Vec<P>) -> Self where DefaultAllocator: Allocator<f64, P::N> {
        KdTree(kd_tree::build(pts, 0))
    }

    fn splitting_hyperplane(&self) -> Option<Hyperplane<P::N>> {
        let &KdTree(ref tree) = self;
        kd_tree::splitting_hyperplane(tree)
    }

}

mod kd_tree {
    use na::{DefaultAllocator, DimName, U1, VectorN};
    use na::allocator::Allocator;
    use domain::{Hyperplane, Hypersphere, NPoint};
    use std::iter;

    /// kd-Tree implementation structure
    pub(super) enum KdTreeImpl<P : NPoint> where DefaultAllocator: Allocator<f64, P::N> {
        Node(usize, Box<KdTreeImpl<P>>, P, Box<KdTreeImpl<P>>),
        Empty()
    }

    /// mutably enunerate points from a kd tree into a vector
    pub(super) fn add_points_to_vector<P: NPoint>(tree : &KdTreeImpl<P>, pts : &mut Vec<P>) where DefaultAllocator: Allocator<f64, P::N> {
        if let &KdTreeImpl::Node(_, ref l, ref pt, ref r) = tree {
            add_points_to_vector(l, pts);
            pts.push(pt.clone());
            add_points_to_vector(r, pts);
        }
    }

    pub(super) fn build<P: NPoint>(pts: Vec<P>, dim_idx: usize) -> KdTreeImpl<P> where DefaultAllocator: Allocator<f64, P::N>  {
        if pts.is_empty() {
            return KdTreeImpl::Empty();
        }

        // The dimensionality is statically known
        let next_dim_idx = if dim_idx + 1 > P::N::dim() {
            0
        } else {
            dim_idx + 1
        };

        let (left, point, right) = split_vec_with_median(pts, dim_idx).unwrap(); // panics on empty vec
        KdTreeImpl::Node(
            dim_idx,
            Box::new(build(left, next_dim_idx)),
            point,
            Box::new(build(right, next_dim_idx)),
        )
    }

    /// Returns None on an empty vec. Returns empty l/r subtrees if too few elements
    fn split_vec_with_median<P: NPoint>(mut pts: Vec<P>, dim_idx: usize) -> Option<(Vec<P>, P, Vec<P>)>
    where
        DefaultAllocator: Allocator<f64, P::N>,
    {
        pts.sort_by(|a, b| unsafe {
            a.from_origin()
                .vget_unchecked(dim_idx)
                .partial_cmp(b.from_origin().vget_unchecked(dim_idx))
                .unwrap()
        });
        if pts.len() >= 3 {
            let length = pts.len();
            let mut middle = pts.split_off(length / 2);
            let right = middle.split_off(1);
            // `remove` takes the item out of the vec, which is preferable to copying
            Some((pts, middle.remove(0), right))
        } else if pts.len() == 2 {
            let mut middle = pts.split_off(1);
            Some((pts, middle.remove(0), Vec::new()))
        } else if pts.len() == 1 {
            Some((Vec::new(), pts.remove(0), Vec::new()))
        } else {
            None
        }

    }

    pub(super) fn splitting_hyperplane<P : NPoint>(tree : &KdTreeImpl<P>) -> Option<Hyperplane<P::N>> where DefaultAllocator: Allocator<f64, P::N> {
        match tree{
            &KdTreeImpl::Node(dim_idx, _, ref pt, _)  => unsafe {
                let mut normal_vector = vec![0.0; P::N::dim()];
                normal_vector[dim_idx] = 1.0;
                let normal = VectorN::from_data_statically_unchecked(DefaultAllocator::allocate_from_iterator(P::N::name(), U1, normal_vector));
                Some(Hyperplane {origin : pt.from_origin().clone(), normal : normal })
            },
            &KdTreeImpl::Empty() => None
        }
    }

    fn closest_to<KP : NPoint, P : NPoint<N = KP::N>>(opt_pt1 : &Option<KP>, pt : &KP, opt_pt2 : &Option<KP>, search_pt : &P) -> KP where DefaultAllocator: Allocator<f64, KP::N> {
        opt_pt1.iter().chain(opt_pt2.iter()).chain(iter::once(pt)).min_by(|a, b| {
            (a.from_origin() - search_pt.from_origin()).norm()
                .partial_cmp(&(b.from_origin() - search_pt.from_origin()).norm())
                .unwrap()
        }).unwrap().clone()
    }

    pub(super) fn find_nearest<KP : NPoint, P : NPoint<N = KP::N>>(tree : &KdTreeImpl<KP>, search_pt : &P) -> Option<KP> where DefaultAllocator: Allocator<f64, KP::N>  {
        match tree {
            &KdTreeImpl::Node(dim_idx, ref l, ref cur_pt, ref r) => unsafe {
                let cur_pt_vec = cur_pt.from_origin();
                let pt_vec = search_pt.from_origin();
                let (first, second) = if pt_vec.vget_unchecked(dim_idx) < cur_pt_vec.vget_unchecked(dim_idx) { (l, r) } else { (r, l) };
                let first_possible_nearest = find_nearest(first, search_pt);
                match first_possible_nearest {
                    Some(_) => {
                        let search_sphere = Hypersphere {origin : cur_pt_vec.clone(), radius : (cur_pt_vec - pt_vec).norm()};
                        let plane = splitting_hyperplane(&tree).expect("Hyperplane should never be null, we have already ensured the kdtree type is not empty.");
                        if plane.intersects_hypersphere(&search_sphere) {
                            let second_possible_nearest = find_nearest(second, search_pt);
                            Some(closest_to(&first_possible_nearest, cur_pt, &second_possible_nearest, search_pt))
                        } else {
                            Some(closest_to(&first_possible_nearest, cur_pt, &None, search_pt))
                        }
                    },
                    None => Some(cur_pt.clone())
                }
            },
            &KdTreeImpl::Empty() => {
                None
            }
        }
        
    }
}
