// Date: Sun Oct 22 10:05:27 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::{
    collections::HashMap,
    hash::Hash
};

use tui::layout::{Layout, Rect};

pub enum NameOrLayout<T> {
    Name(T),
    Layout(Box<NestedLayout<T>>)
}

pub struct NestedLayout<T> {
    layout: Layout,
    inner: Vec<NameOrLayout<T>>
}

impl<T> NestedLayout<T> 
where T: Hash + Eq + Clone
{
    pub fn new(layout: Layout, inner: Vec<NameOrLayout<T>>) -> Self {
        Self {
            layout,
            inner
        }
    }

    fn run(nested: &Self, rect: Rect, map: &mut HashMap<T, Rect>) {
        let chunks = nested.layout.split(rect);
        nested.inner.iter().enumerate()
            .for_each(|(index, item)| {
                match item {
                    NameOrLayout::Name(name) => {
                        map.insert(name.clone(), chunks[index]);
                    },
                    NameOrLayout::Layout(inner_nested) => {
                        Self::run(inner_nested.as_ref(), chunks[index], map);
                    }
                }
            });
    }

    pub fn split(&self, rect: Rect) -> HashMap<T, Rect> {
        let mut hash_map = HashMap::<T, Rect>::new();
        Self::run(&self, rect, &mut hash_map);
        hash_map
    }
}
