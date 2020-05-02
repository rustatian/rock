use std::collections::HashMap;
use crate::profile::Profile;

type Nodes<'gr> = Vec<&'gr Node<'gr>>;

struct Graph<'gr> {
    nodes: Nodes<'gr>,
}

#[derive(Debug, Clone)]
struct Node<'gr> {
    info: NodeInfo<'gr>,
    function: &'gr Node<'gr>,
    flat: i64,
    flat_div: i64,
    cum: i64,
    cum_div: i64,
    r#in: EdgeMap<'gr>,
    out: EdgeMap<'gr>,
    label_tags: TagMap<'gr>,
    numeric_tags: HashMap<&'gr str, TagMap<'gr>>,
}


// gr lifetime used for graph
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
struct NodeInfo<'gr> {
    name: &'gr str,
    orig_name: &'gr str,
    address: u64,
    file: &'gr str,
    start_line: isize,
    lineno: isize,
    objfile: &'gr str,

}

type EdgeMap<'gr> = HashMap<&'gr Node<'gr>, &'gr Edge<'gr>>;

#[derive(Debug, Copy, Clone)]
struct Edge<'gr> {
    src: &'gr Node<'gr>,
    weight: i64,
    weight_div: i64,
    residual: bool,
    inline: bool,
}

type TagMap<'gr> = HashMap<&'gr str, &'gr Tag<'gr>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Tag<'gr> {
    name: &'gr str,
    unit: &'gr str,
    value: i64,
    flat: i64,
    flat_div: i64,
    cum: i64,
    cum_div: i64,
}

impl<'gr> Graph<'gr> {
    fn new(p: &'_ Profile) -> Self {
        Graph {
            nodes: vec![]
        }
    }
}

















