use std::borrow::Cow;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    Other(String),
}

struct Node<'path, V> {
    path: Cow<'path, [u8]>,
    priority: usize,
    static_indices: Vec<Cow<'path, [u8]>>,
    static_child: Vec<Self>,
    catch_all_child: Option<Box<Self>>,
    wildcard_child: Option<Box<Self>>,
    add_slash: bool,
    catch_all: bool,
    implicit_head: bool,
    leaves: HashMap<Method, V>,
    leaf_wilcards: Vec<Cow<'path, [u8]>>,
}

impl<'path, V> Node<'path, V> {
    fn sort_static_child(&mut self, i: usize) {
        let mut i = i;
        while i > 0 && self.static_child[i].priority > self.static_child[i - 1].priority {
            self.static_child.swap(i - 1, i);
            self.static_indices.swap(i - 1, i);
            i -= 1;
        }
    }

    fn set_handler(&mut self, method: Method, value: V, implicit_head: bool) {
        if self.leaves.contains_key(&method) {}
    }
}
