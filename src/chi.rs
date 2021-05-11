use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum NodeType {
    Static,
    Regex,
    Param,
    CatchAll,
}

impl Default for NodeType {
    fn default() -> Self {
        Self::Static
    }
}

pub struct Node<'path, K, V> {
    typ: NodeType,
    label: u8,
    tail: u8,
    prefix: Cow<'path, [u8]>,
    regex: Option<regex::Regex>,
    endpoints: HashMap<K, Route<'path, V>>,
    children: [Vec<Node<'path, K, V>>; 4],
}

impl<K, V> Default for Node<'_, K, V> {
    fn default() -> Self {
        Self {
            typ: Default::default(),
            label: Default::default(),
            tail: Default::default(),
            prefix: Default::default(),
            regex: Default::default(),
            endpoints: Default::default(),
            children: Default::default(),
        }
    }
}

struct Route<'path, V> {
    value: Option<V>,
    path: Cow<'path, [u8]>,
    params: Vec<String>,
}

impl<V> Default for Route<'_, V> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            path: Default::default(),
            params: Default::default(),
        }
    }
}

impl<'path, K, V> Node<'path, K, V> {
    pub fn insert(mut self, path: &'path str, key: K, value: V) {
        let mut parent: Option<&mut Self> = None;
        let mut current = &self;
        let search = path.as_bytes();
        loop {
            // key exhaustation
            if search.is_empty() {
                // insert/update the node's leaf handler
                return current.set_endpoint(path, key, value);
            }

            // search for a wild node next
            // we need to get the tail
            let label = search[0];
            let mut segment = Segment::default();
            if label == b'{' || label == b'*' {
                segment = Segment::next(search);
            }
            let mut prefix: &[u8] = &[];
            if segment.typ == NodeType::Regex {
                prefix = segment.regex;
            }

            // look for an edge to attach to
            let tobecurrent = match current.get_edge(segment.typ, label, segment.tail, prefix) {
                Some(edge) => edge,
                // no edge, create one
                None => {
                    let child = Node {
                        label,
                        tail: segment.tail,
                        prefix: search.into(),
                        ..Node::default()
                    };
                    let mut hn = parent.unwrap().add_child(child, search);
                    hn.set_endpoint(path, key, value);
                    return;
                }
            };
            parent = Some(current);
            current = tobecurrent;
            return;
        }
    }

    fn get_edge(
        &self,
        typ: NodeType,
        label: u8,
        tail: u8,
        prefix: &[u8],
    ) -> Option<&Self> {
        None
    }

    pub fn set_endpoint(&mut self, path: &'path str, key: K, value: V) {}
    pub fn add_child(&mut self, child: Self, prefix: &[u8]) -> Self {
        Self::default()
    }
}

#[derive(Default)]
struct Segment<'pattern> {
    tail: u8,
    end: u8,
    typ: NodeType,
    regex: &'pattern [u8],
}

impl Segment<'_> {
    fn next(search: &'_ [u8]) -> Self {
        Self {
            tail: 0,
            end: 0,
            typ: NodeType::Static,
            regex: &[],
        }
    }
}
