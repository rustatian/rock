#![warn(missing_debug_implementations)]
#![allow(dead_code)] //temporary
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

// EdgeMap is used to represent the incoming/outgoing edges from a node.
// type EdgeMap<'a> = HashMap<Node<'a>, Edge<'a>>;

// TagMap is a collection of tags, classified by their name.
// type TagMap = HashMap<String, Tag>;

// Graph summarizes a performance profile into a format that is
// suitable for visualization.
#[derive(Clone, Debug)]
struct Graph<'a> {
    nodes: Node<'a>,
}

// Node is an entry on a profiling report. It represents a unique
// program location.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Node<'a, T> where T:Eq + Hash {
    // Info describes the source location associated to this node.
    info: NodeInfo,

    // Function represents the function that this node belongs to. On
    // graphs with sub-function resolution (eg line number or
    // addresses), two nodes in a NodeMap that are part of the same
    // function have the same value of Node.Function. If the Node
    // represents the whole function, it points back to itself.
    function: Box<T>,

    // Values associated to this node. Flat is exclusive to this node,
    // Cum includes all descendents.
    flat: i64,
    flat_div: i64,
    cum: i64,
    cum_div: i64,
    // TODO edge lifetime??
    // In and out Contains the nodes immediately reaching or reached by
    // this node.
    r#in: HashMap<&'a T, &'a T>,
    out: HashMap<&'a T, &'a T>,
    // LabelTags provide additional information about subsets of a sample.
    label_tags: HashMap<String, Tag>,

    // NumericTags provide additional values for subsets of a sample.
    // Numeric tags are optionally associated to a label tag. The key
    // for NumericTags is the name of the LabelTag they are associated
    // to, or "" for numeric tags not associated to a label tag.
    numeric_tags: HashMap<String, HashMap<String, Tag>>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
        self.function.hash(state);
        self.flat.hash(state);
        self.flat_div.hash(state);
        self.cum.hash(state);
        self.cum_div.hash(state);
    }
}

impl<'a> Node<'a> {
    pub fn new() -> Self {
        Node::default()
    }
    // FlatValue returns the exclusive value for this node, computing the
    // mean if a divisor is available.
    pub fn flat_value(&self) -> i64 {
        if self.flat_div == 0 {
            return self.flat;
        }
        self.flat / self.flat_div
    }

    // CumValue returns the inclusive value for this node, computing the
    // mean if a divisor is available.
    pub fn cum_value(&self) -> i64 {
        if self.cum_div == 0 {
            return self.cum;
        }
        self.cum / self.cum_div
    }

    pub fn add_to_edge(&mut self, to: &mut Node<'_>, v: i64, residual: bool, inline: bool) {}

    // fn keys_match<T: Eq + Hash, U>(map1: &HashMap<T, U>, map2: &HashMap<T, U>) -> bool {
    //     map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
    // }

    pub fn add_to_edge_div<T: Eq + Hash>(
        &mut self,
        to: &mut Node<'_>,
        dv: i64,
        v: i64,
        residual: bool,
        inline: bool,
    ) {
        // let node1 = self.input.
        // if self.r#in[to] != self.r#in[self] {
        //     panic!("error");
        // }
    }
}

// NodeInfo contains the attributes for a node.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
struct NodeInfo {
    name: String,
    orig_name: String,
    address: u64,
    file: String,
    start_line: i64,
    lineno: i64,
    objfile: String,
}

// Edge contains any attributes to be represented about edges in a graph.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Edge<'a> {
    src: &'a Node<'a>,
    dest: &'a Node<'a>,
    // The summary weight of the edge
    weight: i64,
    weight_div: i64,
    // residual edges connect nodes that were connected through a
    // separate node, which has been removed from the report.
    residual: bool,
    // An inline edge represents a call that was inlined into the caller.
    inline: bool,
}

impl<'a> Edge<'a> {
    // WeightValue returns the weight value for this edge, normalizing if a
    // divisor is available.
    pub fn weight_value(&self) -> i64 {
        if self.weight_div == 0 {
            return self.weight;
        }
        self.weight / self.weight_div
    }
}

// Tag represent sample annotations
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Tag {
    name: String,
    // Describe the value, "" for non-numeric tags
    unit: String,
    value: i64,
    flat: i64,
    flat_div: i64,
    cum: i64,
    cum_div: i64,
}

impl Tag {
    // CumValue returns the inclusive value for this tag, computing the
    // mean if a divisor is available.
    pub fn cum_value(&self) -> i64 {
        if self.cum_div == 0 {
            return self.cum;
        }
        self.cum / self.cum_div
    }

    // FlatValue returns the exclusive value for this tag, computing the
    // mean if a divisor is available.
    pub fn flat_value(&self) -> i64 {
        if self.flat_div == 0 {
            return self.flat;
        }
        self.flat / self.flat_div
    }
}
