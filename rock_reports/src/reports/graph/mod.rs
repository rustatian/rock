use std::collections::HashMap;
use rock_parser::profile::Profile;
use rock_utils::types::Options;

pub(crate) struct Graph<'gr> {
    profile: &'gr Profile,
    nodes: Nodes<'gr>,
}

#[derive(Default)]
struct Nodes<'gr> {
    nodes: Vec<&'gr Node<'gr>>,
}

#[derive(Default)]
struct NodeSet<'gr> {
    node_set: HashMap<NodeInfo<'gr>, bool>,
}

#[derive(Default)]
struct NodeMap<'gr> {
    node_map: HashMap<NodeInfo<'gr>, &'gr Node<'gr>>,
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
    obj_file: &'gr str,
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
    pub(crate) fn new(p: &'gr Profile) -> Self {
        Graph {
            profile: p,
            nodes: Nodes::default(),
        }
    }

    fn init_graph(&mut self) {}

    // CreateNodes creates graph nodes for all locations in a profile. It
    // returns set of all nodes, plus a mapping of each location to the
    // set of corresponding nodes (one per location.Line).
    fn create_nodes(&mut self, opt: Options) {
        let mut locations: HashMap<u64, Nodes<'_>> =
            HashMap::with_capacity(self.profile.location.len());

        let mut nm = NodeMap::default();

        for (_, l) in self.profile.location.iter().enumerate() {
            let mut lines = l.line.clone();
            let mut nodes = Nodes::default();

            for ln in lines {
                // nodes[ln] =
            }
        }
    }
}
