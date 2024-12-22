use std::{array::from_fn, collections::HashSet, mem::take};

use itertools::Itertools;

use crate::{Boundable, Cell, Cuboid, ExtraItertools, Point, Z, bounds};

const K: usize = 12;

#[derive(Clone, Debug)]
pub enum KTree<const N: usize>
where
    [(); (2usize).pow(N as _)]:,
{
    Leaf(HashSet<Cell<N>>),
    Node {
        children: Box<[KTree<N>; (2usize).pow(N as _)]>,
        split_point: Point<N>,
        total_len: usize,
        bounding_box: Cuboid<N>,
    },
}

impl<const N: usize> Default for KTree<N>
where
    [(); (2usize).pow(N as _)]:,
{
    fn default() -> Self {
        Self::Leaf(HashSet::new())
    }
}

fn find_split_index<const N: usize>(split_point: Point<N>, c: Cell<N>) -> usize {
    let p = c.point();
    let mut result = 0;
    for i in (0..N).rev() {
        result <<= 1;
        if p[i] < split_point[i] {
            result |= 1;
        }
    }
    result
}

pub fn split_cuboid<const N: usize>(
    split_point: Point<N>,
    cuboid: Cuboid<N>,
) -> [Cuboid<N>; (2usize).pow(N as _)] {
    cuboid.all_corner_points().map(|x| bounds([x, split_point]))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SemiCuboid<const N: usize> {
    // points, inclusive
    pub min: [Option<Z>; N],
    pub max: [Option<Z>; N],
}

impl<const N: usize> SemiCuboid<N> {
    pub fn from_split_point(split_point: Point<N>) -> [Self; (2usize).pow(N as _)] {
        dbg!(&split_point);
        todo!()
    }

    pub fn contains_point(&self, p: Point<N>) -> bool {
        (0..N).all(|dim| {
            if let Some(min) = self.min[dim] {
                if p.0[dim] < min {
                    return false;
                }
            }
            if let Some(max) = self.max[dim] {
                if p.0[dim] > max {
                    return false;
                }
            }
            true
        })
    }

    pub fn contains(&self, p: impl Boundable<N>) -> bool {
        p.points().all(|x| self.contains_point(x))
    }
}

impl<const N: usize> KTree<N>
where
    [(); (2usize).pow(N as _)]:,
{
    fn split(&mut self) {
        match take(self) {
            KTree::Leaf(set) => {
                let split_point = Point(from_fn(|i| {
                    let proj = set.iter().copied().map(|x| x[i]).sorted().cv();
                    proj[proj.len() / 2]
                }));
                let bounding_box = bounds(set.iter().copied());
                let mut children = Box::new(from_fn(|_| Self::default()));
                let total_len = set.len();
                for c in set {
                    children[find_split_index(split_point, c)].add(c);
                }
                *self = Self::Node {
                    children,
                    split_point,
                    total_len,
                    bounding_box,
                }
            }
            KTree::Node { .. } => unreachable!(),
        }
    }

    fn write_to(&self, f: &mut impl FnMut(Cell<N>)) {
        match self {
            KTree::Leaf(hash_set) => {
                for c in hash_set.iter().copied() {
                    f(c);
                }
            }
            KTree::Node { children, .. } => {
                for c in &**children {
                    c.write_to(f);
                }
            }
        }
    }

    fn unsplit(&mut self) {
        let mut set = HashSet::new();
        self.write_to(&mut |x| {
            set.insert(x);
        });
        *self = KTree::Leaf(set);
    }

    pub fn add(&mut self, c: Cell<N>) {
        match self {
            KTree::Leaf(set) => {
                set.insert(c);
                if set.len() > K {
                    self.split();
                }
            }
            KTree::Node {
                children,
                split_point,
                total_len,
                bounding_box,
            } => {
                *total_len += 1;
                bounding_box.extend(c);
                children[find_split_index(*split_point, c)].add(c);
            }
        }
    }

    pub fn contains(&self, c: Cell<N>) -> bool {
        match self {
            KTree::Leaf(set) => set.contains(&c),
            KTree::Node {
                children,
                split_point,
                ..
            } => children[find_split_index(*split_point, c)].contains(c),
        }
    }

    pub fn remove(&mut self, c: Cell<N>) -> bool {
        match self {
            KTree::Leaf(set) => set.remove(&c),
            KTree::Node {
                children,
                split_point,
                total_len,
                bounding_box,
            } => {
                let removed = children[find_split_index(*split_point, c)].remove(c);
                *bounding_box = bounds(children.iter().map(|x| x.bounds()));
                if removed {
                    *total_len -= 1;
                    if *total_len < K / 2 {
                        self.unsplit();
                    }
                }
                removed
            }
        }
    }

    pub fn bounds(&self) -> Cuboid<N> {
        match self {
            KTree::Leaf(set) => bounds(set.iter().copied()),
            KTree::Node { bounding_box, .. } => *bounding_box,
        }
    }

    pub fn count(&self, b: Cuboid<N>) -> usize {
        match self {
            KTree::Leaf(set) => set.iter().filter(|x| b.contains(**x)).count(),
            KTree::Node {
                children,
                total_len,
                bounding_box,
                ..
            } => {
                if b.contains(*bounding_box) {
                    *total_len
                } else if b.intersect_cells(*bounding_box) {
                    children.iter().map(|tree| tree.count(b)).sum()
                } else {
                    0
                }
            }
        }
    }
}
