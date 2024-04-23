/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::{Debug, Display};

pub struct Heap<T>
where
    T: Default + Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count = self.count + 1;
        let mut ldx = self.count - 1;
        let mut rdx = self.parent_idx(ldx);
        while (self.comparator)(&self.items[ldx],&self.items[rdx]) && rdx != ldx{
            self.items.swap(ldx, rdx);
            ldx = rdx;
            rdx = self.parent_idx(ldx);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Debug + Display,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        println!("-----");
        println!("cur heap:{:?}", self.items);
        if self.is_empty(){
            None
        }else{
            let l: usize = 0;
            let r = self.count - 1;
            self.items.swap(l, r);
            let res = self.items.remove(self.count - 1);
            self.count = self.count - 1;
            let mut cur = self.items.get(0);
            let mut cur_idx = 0;
            while cur.is_some(){
                let lc = self.left_child_idx(cur_idx);
                let rc = self.right_child_idx(cur_idx);
                let lcv = self.items.get(lc);
                let rcv = self.items.get(rc);
                match (cur,lcv,rcv) {
                    (Some(c),Some(l),Some(r)) => if (self.comparator)(l,r){
                        if (self.comparator)(l,c){
                            self.items.swap(cur_idx, lc);
                            cur_idx = lc;
                        }else{
                            break;
                        }
                    }else{
                        if (self.comparator)(r,c){
                            self.items.swap(cur_idx, rc);
                            cur_idx = rc;
                        }else{
                            break;
                        }
                    },
                    (Some(c), _, Some(r)) => if (self.comparator)(r,c){
                        self.items.swap(cur_idx, rc);
                        cur_idx = rc;
                    }else{
                        break;
                    },
                    (Some(c),Some(l),_) => if (self.comparator)(l,c){
                        self.items.swap(cur_idx, lc);
                        cur_idx = lc;
                    }else{
                        break;
                    }
                    _ => break
                }
                cur = self.items.get(cur_idx)
            }
            println!("pop {}, cur heap:{:?}", res, self.items);
            Some(res)


        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}