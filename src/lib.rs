#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![feature(iter_array_chunks)]
#![feature(const_for)]
#![feature(box_patterns)]
#![feature(closure_track_caller)]
#![feature(backtrace_frames)]

pub use ::tap::*;
pub use btree_vec::BTreeVec;
pub use cached::proc_macro::cached;
pub use compact_str::*;
pub use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
pub use indexmap::{IndexMap, IndexSet};
pub use itertools::Itertools;
pub use magiclist::MagicList;
pub use multimap::MultiMap;
use num::rational::Ratio;
pub use num::*;
pub use pathfinding::directed::astar::*;
pub use pathfinding::directed::bfs::*;
pub use pathfinding::directed::count_paths::*;
pub use pathfinding::directed::cycle_detection::*;
pub use pathfinding::directed::dfs::*;
pub use pathfinding::directed::dijkstra::*;
pub use pathfinding::directed::edmonds_karp::*;
pub use pathfinding::directed::fringe::*;
pub use pathfinding::directed::idastar::*;
pub use pathfinding::directed::iddfs::*;
pub use pathfinding::directed::strongly_connected_components::*;
pub use pathfinding::directed::topological_sort::*;
pub use pathfinding::directed::yen::*;
pub use pathfinding::grid::*;
pub use pathfinding::kuhn_munkres::*;
pub use pathfinding::undirected::connected_components::*;
pub use pathfinding::undirected::kruskal::*;
pub use pathfinding::utils::*;
pub use prime_factorization::*;
pub use rand::{seq::SliceRandom, thread_rng};
pub use range_ext::intersect::Intersect;
pub use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use reqwest::blocking::Client;
pub use rustc_hash::{FxHashMap, FxHashSet};
use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use std::any::Any;
use std::array;
use std::backtrace::Backtrace;
pub use std::cmp::Ordering;
pub use std::collections::*;
pub use std::fmt::{Debug, Display};
pub use std::fs::{File, read_to_string};
pub use std::hash::Hash;
use std::io::Read;
pub use std::io::Write;
pub use std::iter::from_fn;
pub use std::ops::Mul;
use std::ops::Range;
use std::ops::RangeBounds;
pub use std::process::{Command, Stdio};
use std::ptr::null;
use std::str::FromStr;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
pub use std::{env, io};

pub type Z = i128;
pub type Q = Ratio<Z>;

pub mod aoc_util;
pub mod cartesian;
pub mod defaultmap;
pub mod ktree;
pub mod printer;

pub use crate::aoc_util::*;
pub use crate::cartesian::*;
pub use crate::defaultmap::*;
pub use crate::ktree::*;
pub use crate::printer::*;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn fetch(url: &str) -> Result<String, reqwest::Error> {
    Client::new()
        .get(url)
        .header(
            "cookie",
            format!("session={}", env::var("AOC_SESSION").unwrap()),
        )
        .header(
            "user-agent",
            "github.com/danielhuang/aoc-2024 - hello@danielh.cc",
        )
        .send()?
        .error_for_status()?
        .text()
}

#[cfg(debug_assertions)]
pub const DEBUG: bool = true;

#[cfg(not(debug_assertions))]
pub const DEBUG: bool = false;

fn read_clipboard() -> Option<String> {
    let mut cmd = Command::new("xclip")
        .arg("-o")
        .arg("clip")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdout = cmd.stdout.take().unwrap();
    cmd.wait().unwrap();
    let mut s = String::new();
    match stdout.read_to_string(&mut s) {
        Ok(_) => Some(s),
        Err(e) => {
            dbg!(e);
            None
        }
    }
}

pub trait CollectionExt<T> {
    fn min_c(&self) -> T;
    fn max_c(&self) -> T;
    fn iter_c(&self) -> impl Iterator<Item = T>;
}

impl<T: Clone + Ord, C: IntoIterator<Item = T> + Clone> CollectionExt<T> for C {
    fn min_c(&self) -> T {
        self.clone().into_iter().min().unwrap()
    }

    fn max_c(&self) -> T {
        self.clone().into_iter().max().unwrap()
    }

    fn iter_c(&self) -> impl Iterator<Item = T> {
        self.clone().into_iter()
    }
}

pub fn collect_2d<T>(s: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> Vec<Vec<T>> {
    s.into_iter().map(|x| x.into_iter().collect()).collect()
}

pub fn transpose_vec<T: Default + Clone>(s: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(s.iter().map(|x| x.len()).all_equal());
    assert!(!s.is_empty());
    assert!(!s[0].is_empty());
    let mut result = vec![vec![T::default(); s.len()]; s[0].len()];
    for (i, row) in s.iter().cloned().enumerate() {
        for (j, x) in row.iter().cloned().enumerate() {
            result[j][i] = x;
        }
    }
    result
}

pub fn transpose<T: Default + Clone>(
    s: impl IntoIterator<Item = impl IntoIterator<Item = T>>,
) -> Vec<Vec<T>> {
    transpose_vec(collect_2d(s))
}

pub fn set_n<T: Eq + Hash + Clone>(
    a: impl IntoIterator<Item = T>,
    b: impl IntoIterator<Item = T>,
) -> HashSet<T> {
    let a: HashSet<_> = a.into_iter().collect();
    let b: HashSet<_> = b.into_iter().collect();
    a.intersection(&b).cloned().collect()
}

pub fn set_u<T: Eq + Hash + Clone>(
    a: impl IntoIterator<Item = T>,
    b: impl IntoIterator<Item = T>,
) -> HashSet<T> {
    let a: HashSet<_> = a.into_iter().collect();
    let b: HashSet<_> = b.into_iter().collect();
    a.union(&b).cloned().collect()
}

pub trait ExtraItertools: IntoIterator + Sized {
    fn collect_set(self) -> HashSet<Self::Item>
    where
        Self::Item: Eq + Hash,
    {
        self.ii().collect()
    }

    fn cset(self) -> HashSet<Self::Item>
    where
        Self::Item: Eq + Hash,
    {
        self.ii().collect()
    }

    fn collect_string(self) -> String
    where
        Self::Item: Display,
    {
        self.ii().map(|x| x.to_string()).collect()
    }

    fn cstr(self) -> String
    where
        Self::Item: Display,
    {
        self.collect_string()
    }

    fn cii(&self) -> std::vec::IntoIter<Self::Item>
    where
        Self: Clone,
    {
        self.clone().into_iter().collect_vec().into_iter()
    }

    fn ii(self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    fn test(
        self,
        mut pass: impl FnMut(&Self::Item) -> bool,
        mut fail: impl FnMut(&Self::Item) -> bool,
    ) -> bool {
        for item in self {
            if pass(&item) {
                return true;
            }
            if fail(&item) {
                return false;
            }
        }
        unreachable!("the iterator does not pass or fail");
    }

    fn one(&self) -> Self::Item
    where
        Self: Clone,
    {
        let mut iter = self.cii();
        let item = iter.next().unwrap();
        assert!(iter.next().is_none());
        item
    }

    fn cv(self) -> Vec<Self::Item> {
        self.into_iter().collect()
    }

    fn cvd(self) -> VecDeque<Self::Item> {
        self.into_iter().collect()
    }

    fn cbv(self) -> BTreeVec<Self::Item> {
        self.into_iter().collect()
    }

    fn cbt(self) -> BTreeVec<Self::Item> {
        self.into_iter().collect()
    }

    fn cm(self) -> MagicList<Self::Item> {
        self.into_iter().collect()
    }

    fn ca<const N: usize>(self) -> [Self::Item; N]
    where
        <Self as std::iter::IntoIterator>::Item: std::fmt::Debug,
    {
        self.cv().try_into().unwrap()
    }

    fn sumi(self) -> Self::Item
    where
        <Self as std::iter::IntoIterator>::Item: std::ops::Add<Output = Self::Item> + Default,
    {
        self.ii().fold(Default::default(), |a, b| a + b)
    }

    fn index_of(self, x: &Self::Item) -> Option<usize>
    where
        Self::Item: PartialEq,
    {
        self.ii().position(|y| x == &y)
    }

    fn has(self, x: &Self::Item) -> bool
    where
        Self::Item: PartialEq,
    {
        self.index_of(x).is_some()
    }
}

impl<T: IntoIterator + Sized> ExtraItertools for T {}

pub fn freqs<T: Hash + Eq>(i: impl IntoIterator<Item = T>) -> DefaultHashMap<T, usize> {
    let mut result = DefaultHashMap::new(0);
    for x in i {
        result[x] += 1;
    }
    result
}

pub trait SignedExt: Signed {
    fn with_abs(self, f: impl FnOnce(Self) -> Self) -> Self {
        f(self.abs()) * self.signum()
    }
}

impl<T: Signed> SignedExt for T {}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Snailfish {
    Num(Z),
    Array(Vec<Snailfish>),
}

impl Snailfish {
    pub fn from_value(v: &Value) -> Self {
        match v {
            Value::Number(x) => Self::Num(x.as_i64().unwrap() as Z),
            Value::Array(x) => Self::Array(x.iter().map(Self::from_value).collect_vec()),
            _ => unreachable!("invalid"),
        }
    }
}

impl FromStr for Snailfish {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl PartialOrd for Snailfish {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Snailfish {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Snailfish::Num(a), Snailfish::Num(b)) => a.cmp(b),
            (Snailfish::Num(a), Snailfish::Array(b)) => vec![Snailfish::Num(*a)].cmp(b),
            (Snailfish::Array(a), Snailfish::Num(b)) => a.cmp(&vec![Snailfish::Num(*b)]),
            (Snailfish::Array(a), Snailfish::Array(b)) => a.cmp(b),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IntervalEdge {
    Start,
    End,
}

#[derive(Default, Debug, Clone)]
pub struct Intervals {
    bounds: BTreeMap<Z, IntervalEdge>,
}

impl Intervals {
    fn remove_between(&mut self, range: impl RangeBounds<Z>) {
        let (start, end) = (range.start_bound(), range.end_bound());
        match (start, end) {
            (Bound::Excluded(s), Bound::Excluded(e)) if s == e => {
                return;
            }
            (Bound::Included(s) | Bound::Excluded(s), Bound::Included(e) | Bound::Excluded(e))
                if s > e =>
            {
                return;
            }
            _ => {}
        }
        for to_remove in self.bounds.range(range).map(|x| *x.0).collect_vec() {
            self.bounds.remove(&to_remove);
        }
    }

    pub fn add(&mut self, start_inclusive: Z, end_exclusive: Z) {
        if end_exclusive <= start_inclusive {
            return;
        }

        match (self.contains(start_inclusive), self.contains(end_exclusive)) {
            (true, true) => {
                self.remove_between(start_inclusive..=end_exclusive);
            }
            (true, false) => {
                self.remove_between(start_inclusive..=end_exclusive);
                self.bounds.insert(end_exclusive, IntervalEdge::End);
            }
            (false, true) => {
                self.remove_between(start_inclusive..=end_exclusive);
            }
            (false, false) => {
                self.remove_between(start_inclusive..=end_exclusive);
                self.bounds.insert(end_exclusive, IntervalEdge::End);
            }
        }
        if !self.contains(start_inclusive) {
            self.bounds.insert(start_inclusive, IntervalEdge::Start);
        }
        assert!(self.contains(start_inclusive));
        assert!(self.contains(end_exclusive - 1));
    }

    pub fn add_one(&mut self, x: Z) {
        self.add(x, x + 1);
    }

    pub fn remove(&mut self, start_inclusive: Z, end_exclusive: Z) {
        if end_exclusive <= start_inclusive {
            return;
        }

        match (self.contains(start_inclusive), self.contains(end_exclusive)) {
            (true, true) => {
                self.remove_between(start_inclusive..end_exclusive);
                self.bounds.insert(start_inclusive, IntervalEdge::End);
                self.bounds.insert(end_exclusive, IntervalEdge::Start);
            }
            (true, false) => {
                self.remove_between(start_inclusive..end_exclusive);
                self.bounds.insert(start_inclusive, IntervalEdge::End);
            }
            (false, true) => {
                self.remove_between(start_inclusive..end_exclusive);
                self.bounds.insert(end_exclusive, IntervalEdge::Start);
            }
            (false, false) => {
                self.remove_between(start_inclusive..end_exclusive);
            }
        }
        assert!(!self.contains(start_inclusive));
        assert!(!self.contains(end_exclusive - 1));
    }

    pub fn remove_one(&mut self, x: Z) {
        self.remove(x, x + 1);
    }

    pub fn contains(&self, x: Z) -> bool {
        if let Some(edge) = self.bounds.range(..=x).next_back() {
            edge.1 == &IntervalEdge::Start
        } else {
            false
        }
    }

    pub fn contains_all(&self, start_inclusive: Z, end_exclusive: Z) -> bool {
        if start_inclusive <= end_exclusive {
            return true;
        }

        self.contains(start_inclusive)
            && self.contains(end_exclusive - 1)
            && self
                .bounds
                .range((start_inclusive + 1)..end_exclusive)
                .next()
                .is_none()
    }

    pub fn covered_size(&self) -> Z {
        let mut total = 0;
        for (left, right) in self.bounds.iter().tuple_windows() {
            match left.1 {
                IntervalEdge::Start => {
                    total += right.0 - left.0;
                }
                IntervalEdge::End => {}
            }
        }
        total
    }

    pub fn iter(&self) -> IntervalsIter<'_> {
        IntervalsIter {
            intervals: self,
            last_seen_upward: None,
            last_seen_downward: None,
        }
    }

    pub fn split_off(&mut self, lowest_right_value: Z) -> Self {
        let patch_left = self.contains(lowest_right_value - 1);
        let patch_right = self.contains(lowest_right_value);
        let mut right = self.bounds.split_off(&lowest_right_value);
        if patch_left {
            self.bounds.insert(lowest_right_value, IntervalEdge::End);
        }
        if patch_right {
            right.insert(lowest_right_value, IntervalEdge::Start);
        }
        if let Some(first) = right.iter().next().map(|(k, v)| (*k, *v)) {
            if first.1 == IntervalEdge::End {
                right.remove(&first.0);
            }
        }
        Self { bounds: right }
    }

    pub fn split_at(mut self, lowest_right_value: Z) -> (Self, Self) {
        let right = self.split_off(lowest_right_value);
        (self, right)
    }

    pub fn all_intervals(&self) -> impl Iterator<Item = Range<Z>> + '_ {
        let mut iter = self.bounds.iter();
        std::iter::from_fn(move || {
            let (&lo, lo_edge) = iter.next()?;
            let (&hi, hi_edge) = iter.next()?;
            assert!(lo_edge == &IntervalEdge::Start);
            assert!(hi_edge == &IntervalEdge::End);
            Some(lo..hi)
        })
    }

    pub fn extend(&mut self, other: &Self) {
        for interval in other.all_intervals() {
            self.add(interval.start, interval.end);
        }
    }

    pub fn union(mut a: Self, b: Self) -> Self {
        a.extend(&b);
        a
    }

    pub fn take_range(&mut self, start_inclusive: Z, end_exclusive: Z) -> Self {
        let mid_right = self.split_off(start_inclusive);
        let (mid, right) = mid_right.split_at(end_exclusive);
        for interval in right.all_intervals() {
            self.add(interval.start, interval.end);
        }
        mid
    }

    pub fn subtract(&mut self, to_remove: &Intervals) {
        for interval in to_remove.all_intervals() {
            self.remove(interval.start, interval.end);
        }
    }

    pub fn shift(&mut self, offset: Z) {
        self.bounds = std::mem::take(&mut self.bounds)
            .into_iter()
            .map(|(k, v)| (k + offset, v))
            .collect();
    }
}

#[derive(Clone)]
pub struct IntervalsIter<'a> {
    last_seen_upward: Option<Z>,
    last_seen_downward: Option<Z>,
    intervals: &'a Intervals,
}

impl Iterator for IntervalsIter<'_> {
    type Item = Z;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last_seen_upward) = self.last_seen_upward {
            if self.intervals.contains(last_seen_upward + 1) {
                self.last_seen_upward = Some(last_seen_upward + 1);
                assert!(self.intervals.contains(last_seen_upward + 1));
                Some(last_seen_upward + 1)
            } else {
                for (num, bound) in self.intervals.bounds.range((last_seen_upward + 1)..) {
                    if bound == &IntervalEdge::Start {
                        self.last_seen_upward = Some(*num);
                        assert!(self.intervals.contains(*num));
                        return Some(*num);
                    }
                }
                None
            }
        } else {
            for (num, bound) in self.intervals.bounds.iter() {
                if bound == &IntervalEdge::Start {
                    self.last_seen_upward = Some(*num);
                    assert!(self.intervals.contains(*num));
                    return Some(*num);
                }
            }
            None
        }
    }
}

impl DoubleEndedIterator for IntervalsIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(last_seen_downward) = self.last_seen_downward {
            if self.intervals.contains(last_seen_downward - 1) {
                self.last_seen_downward = Some(last_seen_downward - 1);
                assert!(self.intervals.contains(last_seen_downward - 1));
                Some(last_seen_downward - 1)
            } else {
                for (num, bound) in self.intervals.bounds.range(..=last_seen_downward).rev() {
                    if bound == &IntervalEdge::End {
                        self.last_seen_downward = Some(*num - 1);
                        assert!(self.intervals.contains(*num - 1));
                        return Some(*num - 1);
                    }
                }
                None
            }
        } else {
            for (num, bound) in self.intervals.bounds.iter().rev() {
                if bound == &IntervalEdge::End {
                    self.last_seen_downward = Some(*num - 1);
                    assert!(self.intervals.contains(*num - 1));
                    return Some(*num - 1);
                }
            }
            None
        }
    }
}

pub fn bfs2<T: Clone + Hash + Eq, I: IntoIterator<Item = T>>(
    start: T,
    mut find_nexts: impl FnMut(usize, T) -> I,
) -> impl Iterator<Item = (usize, T)> {
    let mut edge = VecDeque::new();
    let mut seen = HashSet::new();

    seen.insert(start.clone());
    edge.push_back(start);

    let mut i = 0;

    from_fn(move || {
        let mut result = vec![];
        for _ in 0..edge.len() {
            let item = edge.pop_front()?;
            let nexts = find_nexts(i, item.clone());
            for next in nexts {
                seen.insert(next.clone());
                edge.push_back(next);
            }
            result.push((i, item));
        }
        i += 1;
        if result.is_empty() {
            return None;
        }
        Some(result)
    })
    .flatten()
}

pub fn sometimes() -> bool {
    static PREV: Mutex<Option<Instant>> = Mutex::new(None);
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    let count = COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let mut s = PREV.lock().unwrap();
    let result = s.is_none() || s.is_some_and(|x| x.elapsed() > Duration::from_millis(250));
    if result {
        println!("sometimes count: {count}");
        *s = Some(Instant::now());
    }
    result
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Cuboid<const N: usize> {
    // points, inclusive
    pub min: [Z; N],
    pub max: [Z; N],
}

impl<const N: usize> Cuboid<N> {
    pub fn length(&self, dim: usize) -> Z {
        self.max[dim] - self.min[dim]
    }

    pub fn lengths(&self) -> [Z; N] {
        array::from_fn(|dim| self.length(dim))
    }

    pub fn size(&self) -> Z {
        self.lengths().into_iter().product()
    }

    pub fn volume(&self) -> Z {
        self.size()
    }

    pub fn surface_area(&self) -> Z {
        2 * (0..N)
            .map(|k| {
                (0..N)
                    .filter(|&j| k != j)
                    .map(|j| self.length(j))
                    .product::<Z>()
            })
            .sum::<Z>()
    }

    fn assert(&self) {
        for dim in 0..N {
            assert!(self.max[dim] >= self.min[dim]);
        }
    }

    pub fn resize(&self, amount: Z) -> Self {
        let mut new = *self;
        for dim in 0..N {
            new.min[dim] -= amount;
            new.max[dim] += amount;
        }
        new.assert();
        new
    }

    pub fn grow(&self) -> Self {
        self.resize(1)
    }

    pub fn shrink(&self) -> Self {
        self.resize(-1)
    }

    pub fn contains_point(&self, p: Point<N>) -> bool {
        (0..N).all(|dim| (self.min[dim]..=self.max[dim]).contains(&p.0[dim]))
    }

    pub fn contains(&self, p: impl Boundable<N>) -> bool {
        p.points().all(|x| self.contains_point(x))
    }

    fn things_inside(&self, length_add: Z) -> Vec<[Z; N]> {
        let total: Z = (0..N).map(|x| self.length(x) + length_add).product();
        let mut output = vec![];
        for mut n in 0..total {
            let mut pos = self.min;
            for (dim, x) in pos.iter_mut().enumerate() {
                *x += n % (self.length(dim) + length_add);
                n /= self.length(dim) + length_add;
            }
            output.push(pos);
        }
        output
    }

    pub fn points(&self) -> Vec<Point<N>> {
        self.things_inside(1).into_iter().map(Point).collect()
    }

    pub fn cells(&self) -> Vec<Cell<N>> {
        self.things_inside(0).into_iter().map(Cell).collect()
    }

    pub fn corner_cells(&self) -> [Cell<N>; 2] {
        [Cell(self.min), Cell(self.max.map(|x| x - 1))]
    }

    pub fn corner_points(&self) -> [Point<N>; 2] {
        [Point(self.min), Point(self.max)]
    }

    pub fn corner_vector(&self) -> Vector<N> {
        let [a, b] = self.corner_points();
        b - a
    }

    pub fn all_corner_points(&self) -> [Point<N>; (2usize).pow(N as _)]
    where
        [(); (2usize).pow(N as _)]:,
    {
        let corner = self.corner_points()[0];
        let vector = self.corner_vector();
        let mut v = [Default::default(); (2usize).pow(N as _)];
        for (i, x) in v.iter_mut().enumerate() {
            let mut point = corner;
            for n in 0..N {
                if i & (1 << n) != 0 {
                    point.0[n] += vector[n];
                }
            }
            *x = point;
        }
        v
    }

    pub fn all_corner_cells(&self) -> Vec<Cell<N>> {
        let [corner1, corner2] = self.corner_cells();
        let vector = corner2 - corner1;
        let mut v = vec![Default::default(); count_corners(N)];
        for (i, x) in v.iter_mut().enumerate() {
            let mut cell = corner1;
            for n in 0..N {
                if i & (1 << n) != 0 {
                    cell.0[n] += vector[n];
                }
            }
            *x = cell;
        }
        v
    }

    pub fn wrap<T: Cartesian<N>>(&self, value: T) -> T {
        T::new(array::from_fn(|dim| {
            let relative = value[dim] - self.min[dim];
            let wrapped_relative = relative.rem_euclid(self.length(dim));
            wrapped_relative + self.min[dim]
        }))
    }

    /// counts tangents (if faces/edges touch, return true)
    pub fn intersect_points(&self, other: Self) -> bool {
        (0..N).all(|dim| {
            let a = self.min[dim]..=self.max[dim];
            let b = other.min[dim]..=other.max[dim];
            a.does_intersect(&b)
        })
    }

    /// only returns true if there is a mutual cell within both
    pub fn intersect_cells(&self, other: Self) -> bool {
        (0..N).all(|dim| {
            let a = self.min[dim]..self.max[dim];
            let b = other.min[dim]..other.max[dim];
            a.does_intersect(&b)
        })
    }

    pub fn extend(&mut self, x: impl Boundable<N>) {
        for p in x.points() {
            for (dim, val) in p.0.into_iter().enumerate() {
                if val < self.min[dim] {
                    self.min[dim] = val;
                }
                if val > self.max[dim] {
                    self.max[dim] = val;
                }
            }
        }
    }

    pub fn min_cell(&self) -> Cell<N> {
        Cell::new(self.min)
    }

    pub fn max_cell(&self) -> Cell<N> {
        Cell::new(self.min)
    }

    pub fn min_point(&self) -> Point<N> {
        Point::new(self.max)
    }

    pub fn max_point(&self) -> Point<N> {
        Point::new(self.max)
    }
}

impl Cuboid<2> {
    pub fn top_left_point(&self) -> Point<2> {
        p2(self.min[0], self.max[1])
    }

    pub fn top_right_point(&self) -> Point<2> {
        p2(self.max[0], self.max[1])
    }

    pub fn bottom_left_point(&self) -> Point<2> {
        p2(self.min[0], self.min[1])
    }

    pub fn bottom_right_point(&self) -> Point<2> {
        p2(self.max[0], self.min[1])
    }

    pub fn top_left_cell(&self) -> Cell<2> {
        c2(self.min[0], self.max[1] - 1)
    }

    pub fn top_right_cell(&self) -> Cell<2> {
        c2(self.max[0] - 1, self.max[1] - 1)
    }

    pub fn bottom_left_cell(&self) -> Cell<2> {
        c2(self.min[0], self.min[1])
    }

    pub fn bottom_right_cell(&self) -> Cell<2> {
        c2(self.max[0] - 1, self.min[1])
    }

    pub fn width(&self) -> Z {
        self.length(0)
    }

    pub fn height(&self) -> Z {
        self.length(1)
    }
}

impl<const N: usize> std::ops::Add<Vector<N>> for Cuboid<N> {
    type Output = Self;

    fn add(self, rhs: Vector<N>) -> Self::Output {
        Self {
            min: (Vector::new(self.min) + rhs).inner(),
            max: (Vector::new(self.max) + rhs).inner(),
        }
    }
}

impl<const N: usize> std::ops::Sub<Vector<N>> for Cuboid<N> {
    type Output = Self;

    fn sub(self, rhs: Vector<N>) -> Self::Output {
        Self {
            min: (Vector::new(self.min) - rhs).inner(),
            max: (Vector::new(self.max) - rhs).inner(),
        }
    }
}

impl<const N: usize> Boundable<N> for Cuboid<N>
where
    [(); (2usize).pow(N as _)]:,
{
    fn points(&self) -> impl Iterator<Item = Point<N>> {
        self.all_corner_points().into_iter()
    }
}

pub fn bounds_points<const N: usize>(i: impl IntoIterator<Item = Point<N>>) -> Cuboid<N> {
    let mut i = i.into_iter();

    let first = i.next().expect("must have at least one point");
    let mut bounds = Cuboid {
        min: first.0,
        max: first.0,
    };

    for c in i {
        for (dim, val) in c.0.into_iter().enumerate() {
            if val < bounds.min[dim] {
                bounds.min[dim] = val;
            }
            if val > bounds.max[dim] {
                bounds.max[dim] = val;
            }
        }
    }

    bounds.assert();

    bounds
}

pub fn bounds<const N: usize>(i: impl IntoIterator<Item = impl Boundable<N>>) -> Cuboid<N> {
    bounds_points(i.into_iter().flat_map(|x| x.points().collect_vec()))
}

pub trait DefaultHashMapExt<K, V> {
    fn find(&self, f: impl FnMut(V) -> bool) -> Vec<K>;
    fn findv(&self, value: V) -> Vec<K>;
    fn undef(&self) -> HashMap<K, V>;
    fn invert(&self) -> DefaultHashMap<V, Vec<K>>
    where
        V: Clone + Eq + Hash,
        K: Clone;
    fn uninvert(&self) -> HashMap<<V as IntoIterator>::Item, K>
    where
        K: Clone + Eq + Hash,
        V: IntoIterator,
        <V as IntoIterator>::Item: Clone,
        <V as IntoIterator>::Item: std::cmp::Eq + Hash;
    fn subset(&self, f: impl FnMut(K) -> bool) -> Self;
    fn map_key<T: Eq + Hash + Clone>(&self, f: impl FnMut(K) -> T) -> DefaultHashMap<T, V>
    where
        V: Clone;
    fn map_value<U: Clone>(&self, f: impl FnMut(V) -> U) -> DefaultHashMap<K, U>
    where
        K: Eq + Hash + Clone;
}

impl<K: Eq + Hash + Clone, V: Clone + PartialEq> DefaultHashMapExt<K, V> for DefaultHashMap<K, V> {
    fn find(&self, mut f: impl FnMut(V) -> bool) -> Vec<K> {
        let mut l = vec![];
        for (k, v) in self.iter() {
            if f(v.clone()) {
                l.push(k.clone());
            }
        }
        l
    }

    fn findv(&self, value: V) -> Vec<K> {
        self.find(|v| v == value)
    }

    fn undef(&self) -> HashMap<K, V> {
        self.iter()
            .filter(|(_, v)| **v != self.default)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    fn invert(&self) -> DefaultHashMap<V, Vec<K>>
    where
        V: Clone + Eq + Hash,
        K: Clone,
    {
        let mut map = DefaultHashMap::new(vec![]);
        for (k, v) in self.iter() {
            map[v.clone()].push(k.clone());
        }
        map
    }

    fn uninvert(&self) -> HashMap<<V as IntoIterator>::Item, K>
    where
        K: Clone + Eq + Hash,
        V: IntoIterator,
        <V as IntoIterator>::Item: Clone,
        <V as IntoIterator>::Item: std::cmp::Eq + Hash,
    {
        let mut map = HashMap::new();
        for (k, v) in self.iter() {
            for v in v.clone().into_iter() {
                map.insert(v.clone(), k.clone());
            }
        }
        map
    }

    fn subset(&self, mut f: impl FnMut(K) -> bool) -> Self {
        let mut new = self.clone();
        new.clear();
        for key in self.keys() {
            if f(key.clone()) {
                new[key.clone()] = self[key.clone()].clone();
            }
        }
        new
    }

    fn map_key<T: Eq + Hash + Clone>(&self, mut f: impl FnMut(K) -> T) -> DefaultHashMap<T, V> {
        let mut new = DefaultHashMap::new(self.default.clone());
        for key in self.keys() {
            new[f(key.clone())] = self[key.clone()].clone();
        }
        new
    }

    fn map_value<U: Clone>(&self, mut f: impl FnMut(V) -> U) -> DefaultHashMap<K, U> {
        let mut new = DefaultHashMap::new(f(self.default.clone()));
        for key in self.keys() {
            new[key.clone()] = f(self[key.clone()].clone());
        }
        new
    }
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct DisjointSet<T: Hash + Eq + Clone + Debug> {
    map: FxHashMap<T, T>,
    added: FxHashSet<T>,
}

impl<T: Hash + Eq + Clone + Debug> DisjointSet<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
            added: HashSet::default(),
        }
    }

    pub fn using(to_add: impl IntoIterator<Item = T>) -> Self {
        let mut s = Self::new();
        for x in to_add {
            s.add(x);
        }
        s
    }

    fn find_root(&mut self, x: T) -> T {
        if let Some(parent) = self.map.get(&x) {
            let root = self.find_root(parent.clone());
            self.map.insert(x, root.clone()).unwrap();
            root
        } else {
            x
        }
    }

    pub fn add(&mut self, x: T) {
        self.added.insert(x);
    }

    /// Returns true if the set was changed, false if the items were already joined.
    pub fn join(&mut self, a: T, b: T) -> bool {
        self.add(a.clone());
        self.add(b.clone());
        let a = self.find_root(a);
        let b = self.find_root(b);
        if a == b {
            false
        } else {
            assert!(self.map.insert(a, b).is_none());
            true
        }
    }

    pub fn joined(&mut self, a: T, b: T) -> bool {
        self.find_root(a) == self.find_root(b)
    }

    pub fn all(&self) -> Vec<T> {
        self.added.cii().cv()
    }

    pub fn sets(&mut self) -> Vec<Vec<T>> {
        let mut seen_roots = MultiMap::new();
        for x in self.all() {
            seen_roots.insert(self.find_root(x.clone()), x.clone());
        }
        seen_roots.cii().map(|(_, x)| x).collect_vec()
    }
}

pub fn grab_nums<const N: usize>(s: &str) -> [Z; N] {
    s.grab().ints().ca()
}

pub fn grab_unums<const N: usize>(s: &str) -> [usize; N] {
    s.grab().uints().ca()
}

pub trait DisplayExt: Display {
    fn grab(&self) -> String {
        self.to_string()
    }

    fn tos(&self) -> String {
        self.to_string()
    }

    #[track_caller]
    fn int(&self) -> Z {
        self.to_string().trim().parse().unwrap_or_else(
            #[track_caller]
            |_| panic!("tried to parse {} as int", self),
        )
    }

    #[track_caller]
    fn uint(&self) -> usize {
        self.to_string().trim().parse().unwrap_or_else(
            #[track_caller]
            |_| panic!("tried to parse {} as uint", self),
        )
    }

    #[track_caller]
    fn velocity(&self) -> Vector2 {
        charvel(self.to_string().chars().next().unwrap())
    }

    fn ints(&self) -> Vec<Z> {
        self.to_string()
            .split(|c: char| !c.is_numeric() && c != '-')
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.trim().int())
                }
            })
            .collect_vec()
    }

    fn uints(&self) -> Vec<usize> {
        self.to_string()
            .split(|c: char| !c.is_numeric())
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.trim().uint())
                }
            })
            .collect_vec()
    }

    fn uints2(&self) -> Vec<Z> {
        self.to_string()
            .split(|c: char| !c.is_numeric())
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.trim().int())
                }
            })
            .collect_vec()
    }

    fn words(&self) -> Vec<String> {
        self.to_string()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }

    fn paragraphs(&self) -> Vec<String> {
        self.to_string()
            .split("\n\n")
            .map(|x| x.to_string())
            .collect()
    }

    fn paras(&self) -> Vec<String> {
        self.paragraphs()
    }

    fn digits(&self) -> Vec<Z> {
        self.to_string()
            .chars()
            .filter_map(|x| x.to_digit(10).map(|x| x as Z))
            .collect()
    }

    fn json<T: DeserializeOwned>(&self) -> T {
        serde_json::from_str(&self.to_string()).unwrap()
    }

    fn alphanumeric_words(&self) -> Vec<String> {
        self.to_string()
            .replace(|x: char| !x.is_alphanumeric(), " ")
            .words()
    }

    fn regex_capture_groups(&self, regex: &str) -> Vec<Vec<String>> {
        #[cached]
        fn compile_regex(regex: String) -> Regex {
            Regex::new(&regex).unwrap()
        }

        let regex = compile_regex(regex.to_string());
        regex
            .captures_iter(&self.to_string())
            .map(|x| {
                x.iter()
                    .skip(1)
                    .map(|x| x.unwrap().as_str().to_string())
                    .collect_vec()
            })
            .collect_vec()
    }

    fn regex_matches(&self, regex: &str) -> Vec<String> {
        #[cached]
        fn compile_regex(regex: String) -> Regex {
            Regex::new(&regex).unwrap()
        }

        let regex = compile_regex(regex.to_string());
        regex
            .find_iter(&self.to_string())
            .map(|x| x.as_str().to_string())
            .collect_vec()
    }

    fn grid<T: Clone>(
        &self,
        f: impl FnMut(char) -> T,
        default: T,
    ) -> (DefaultHashMap<Cell2, T>, Cuboid<2>) {
        let grid = parse_grid(&self.to_string(), f, default);
        let b = bounds(grid.keys().cloned());
        (grid, b)
    }
}

impl<T: Display> DisplayExt for T {}

#[derive(Default, Clone)]
pub struct OpaqueId {
    inner: Option<Arc<()>>,
}

impl OpaqueId {
    pub fn new() -> Self {
        Self {
            inner: Some(Arc::new(())),
        }
    }

    fn as_ptr(&self) -> *const () {
        self.inner.as_ref().map(Arc::as_ptr).unwrap_or(null())
    }

    fn as_num(&self) -> usize {
        self.as_ptr() as usize
    }
}

impl PartialEq for OpaqueId {
    fn eq(&self, other: &Self) -> bool {
        self.as_num() == other.as_num()
    }
}

impl Eq for OpaqueId {}

impl Ord for OpaqueId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_num().cmp(&other.as_num())
    }
}

impl PartialOrd for OpaqueId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for OpaqueId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_num().hash(state);
    }
}

pub fn fresh() -> usize {
    static STATE: AtomicUsize = AtomicUsize::new(0);
    STATE.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

pub fn memo_dfs<N, FN, IN, FS>(start: N, mut successors: FN, mut success: FS) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    fn step<N, FN, IN, FS>(
        path: &mut Vec<N>,
        successors: &mut FN,
        success: &mut FS,
        cache: &mut FxHashSet<N>,
    ) -> bool
    where
        N: Eq + Hash + Clone,
        FN: FnMut(&N) -> IN,
        IN: IntoIterator<Item = N>,
        FS: FnMut(&N) -> bool,
    {
        if cache.contains(path.last().unwrap()) {
            return false;
        }
        if success(path.last().unwrap()) {
            true
        } else {
            let successors_it = successors(path.last().unwrap());
            for n in successors_it {
                if !path.contains(&n) {
                    path.push(n);
                    if step(path, successors, success, cache) {
                        return true;
                    }
                    path.pop();
                }
            }
            cache.insert(path.last().cloned().unwrap());
            false
        }
    }

    let mut path = vec![start];
    let mut cache = FxHashSet::default();
    step(&mut path, &mut successors, &mut success, &mut cache).then_some(path)
}

pub fn factorize(n: Z) -> Vec<Z> {
    Factorization::run(n as u64)
        .factors
        .into_iter()
        .map(|x| x as Z)
        .collect()
}

pub fn parse_grid<T: Clone>(
    s: &str,
    mut f: impl FnMut(char) -> T,
    default: T,
) -> DefaultHashMap<Cell2, T> {
    let mut grid = DefaultHashMap::new(default);

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[c2(x as _, -(y as Z))] = f(c);
        }
    }

    grid
}

pub fn parse_hashset(s: &str, mut f: impl FnMut(char) -> bool) -> HashSet<Cell2> {
    let mut grid = HashSet::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if f(c) {
                grid.insert(c2(x as _, -(y as Z)));
            }
        }
    }

    grid
}

pub fn max<T: Ord>(items: impl IntoIterator<Item = T>) -> T {
    items.into_iter().max().unwrap()
}

pub fn min<T: Ord>(items: impl IntoIterator<Item = T>) -> T {
    items.into_iter().min().unwrap()
}

pub fn set<T: Hash + Eq>(items: impl IntoIterator<Item = T>) -> HashSet<T> {
    items.into_iter().collect()
}

pub fn defaultdict<K: Hash + Eq, V: Clone>(default: V) -> DefaultHashMap<K, V> {
    DefaultHashMap::new(default)
}

pub fn pause() {
    println!("Press enter to continue");
    io::stdin().read_line(&mut String::new()).unwrap();
}

pub trait BetterToOwned: ToOwned {
    fn to(&self) -> <Self as ToOwned>::Owned {
        self.to_owned()
    }
}

impl<T: ToOwned> BetterToOwned for T {}

pub fn ord(x: char) -> Z {
    x as Z
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Robot<const N: usize> {
    pub pos: Cell<N>,
    pub vel: Vector<N>,
}

impl<const N: usize> Robot<N> {
    pub fn new(pos: Cell<N>, vel: Vector<N>) -> Self {
        Self { pos, vel }
    }

    pub fn tick(&mut self) {
        self.pos += self.vel;
    }
}

pub trait IsIn: PartialEq + Sized {
    fn is_in(&self, iter: impl IntoIterator<Item = Self>) -> bool {
        iter.into_iter().contains(self)
    }
}

impl<T: PartialEq> IsIn for T {}

pub fn bag<T: Eq + Hash>() -> DefaultHashMap<T, Z> {
    DefaultHashMap::new(0)
}

pub fn use_cycles<T, K: Eq + Hash>(
    mut state: T,
    mut next: impl FnMut(T) -> T,
    mut key_fn: impl FnMut(&T) -> K,
    count: usize,
) -> T {
    let mut cache = FxHashMap::default();
    for i in 0..count {
        let key = key_fn(&state);
        if let Some(&v) = cache.get(&key) {
            let cycle_length = i - v;
            let iters_left = count - i;
            let iters_after_cycles = iters_left % cycle_length;
            for _ in 0..iters_after_cycles {
                state = next(state);
            }
            return state;
        }
        cache.insert(key, i);
        state = next(state);
    }
    state
}

pub trait Serde: Serialize {
    fn serde<T: DeserializeOwned>(&self) -> T {
        serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap()
    }
}

impl<T: Serialize> Serde for T {}

pub trait DebugExt: Debug + Sized {
    fn dbg(self) -> Self {
        self.dbgr();
        self
    }

    fn dbgr(&self) {
        let bt = Backtrace::capture();
        let file = bt
            .frames()
            .iter()
            .find(|x| format!("{x:?}").contains("/src/bin/"))
            .unwrap();
        let trace = format!("{:?}", file);
        let trace = trace.split(", ");
        let [_, file, line] = trace.ca();
        let file = file
            .split_once("/src/bin/")
            .unwrap()
            .1
            .strip_suffix('"')
            .unwrap();
        let line = line.ints()[0];
        eprintln!("[src/bin/{}:{}] {:#?}", file, line, self);
    }

    fn cd(&self) -> Self
    where
        Self: Clone,
    {
        self.dbgr();
        self.clone()
    }

    fn to_debug_string(&self) -> String {
        format!("{self:?}")
    }
}

impl<T: Debug> DebugExt for T {}

pub fn bar() {
    let size = terminal_size::terminal_size().unwrap();
    eprintln!("{}", "⎯".repeat(size.0.0 as usize))
}

pub fn unparse_grid(grid: &DefaultHashMap<Cell<2>, char>) -> String {
    let b = bounds(grid.find(|x| x != grid.default));
    let mut s = String::new();
    let mut i = 0;
    let mut cells = b.cells();
    cells.sort_unstable_by_key(|x| (-x[1], x[0]));
    for cell in cells {
        s.push(grid[cell]);
        i += 1;
        if i == b.length(0) {
            i = 0;
            s.push('\n');
        }
    }
    s
}

pub fn zip<T, U>(a: impl IntoIterator<Item = T>, b: impl IntoIterator<Item = U>) -> Vec<(T, U)> {
    a.into_iter().zip(b).collect()
}

pub fn derivative<T: std::ops::Sub<Output = T> + Clone>(a: &[T]) -> Vec<T> {
    let mut result = vec![];
    if let Some(first) = a.first() {
        result.push(first.clone());
    }
    for i in 1..a.len() {
        result.push(a[i].clone() - a[i - 1].clone());
    }
    result
}

pub fn derivative_excl_first<T: std::ops::Sub<Output = T> + Clone>(a: &[T]) -> Vec<T> {
    let mut result = vec![];
    for i in 1..a.len() {
        result.push(a[i].clone() - a[i - 1].clone());
    }
    result
}

pub fn integral<T: std::ops::Add<Output = T> + Clone>(a: &[T]) -> Vec<T> {
    let mut result = a.to_vec();
    for i in 1..result.len() {
        let n = result[i].clone() + result[i - 1].clone();
        result[i] = n;
    }
    result
}

pub fn linear_regression(p1: Point<2>, p2: Point<2>, x: Z) -> Q {
    let Point([x1, y1]) = p1;
    let Point([x2, y2]) = p2;
    let a = Q::new(y2 - y1, x2 - x1);
    let b = Q::new(y2, 1) - a * Q::new(x2, 1);
    a * Q::new(x, 1) + b
}

// wait a minute...
pub const INF: Z = Z::MAX >> 32;
// shhhhh its big enough

pub fn parse_2d(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|x| x.chars().collect()).collect()
}

pub fn extrapolate(known: &mut Vec<i64>, n: usize) {
    assert!(!known.is_empty());
    if known.iter().all(|&x| x == 0) {
        known.extend(vec![0; n]);
        return;
    }
    if known.len() == 1 {
        let next = vec![known[0]; n];
        known.extend(next);
        return;
    }
    let mut derivative = vec![];
    for j in 0..(known.len() - 1) {
        derivative.push(known[j + 1] - known[j]);
    }
    extrapolate(&mut derivative, n);
    for _ in 0..n {
        let next = known[known.len() - 1] + derivative[known.len() - 1];
        known.push(next);
    }
}

pub fn area_points(i: impl IntoIterator<Item = Point<2>> + Clone) -> Z {
    i.cii()
        .circular_tuple_windows()
        .map(|(a, b)| Matrix::new([a.vector(), b.vector()]).det())
        .sumi()
        .abs()
        / 2
}

pub fn area_with_border(i: impl IntoIterator<Item = Cell<2>> + Clone) -> Z {
    let inner = area_points(i.cii().map(|x| x.point()));
    let mut edge = 0;
    for (a, b) in i.cii().circular_tuple_windows() {
        assert!(a.is_aligned_with(b));
        edge += (b - a).manhat();
    }
    inner + edge / 2 + 1
}

pub fn area_without_border(i: impl IntoIterator<Item = Cell<2>> + Clone) -> Z {
    let inner = area_points(i.cii().map(|x| x.point()));
    let mut edge = 0;
    for (a, b) in i.cii().circular_tuple_windows() {
        assert!(a.is_aligned_with(b));
        edge += (b - a).manhat();
    }
    inner - edge / 2 + 1
}

#[derive(Clone, Eq, PartialEq)]
pub struct BoundedGrid<const N: usize, T: Clone> {
    pub grid: DefaultHashMap<Cell<N>, T>,
    pub b: Cuboid<N>,
}

impl<const N: usize, T: Clone> DefaultHashMap<Cell<N>, T> {
    pub fn with_bounds(&self, b: Cuboid<N>) -> BoundedGrid<N, T> {
        BoundedGrid {
            grid: self.clone(),
            b,
        }
    }

    pub fn write_cuboid(&mut self, own_bounds: Cuboid<N>, grid: &BoundedGrid<N, T>) {
        assert_eq!(own_bounds.lengths(), grid.b.lengths());
        for pos in grid.b.cells() {
            let canonical_pos = pos - grid.b.min_cell();
            let new_pos = own_bounds.min_cell() + canonical_pos;
            self[new_pos] = grid.grid[pos].clone();
        }
    }
}
