#![warn(missing_debug_implementations)]
#![allow(dead_code)] //temporary
use std::collections::HashMap;

// EdgeMap is used to represent the incoming/outgoing edges from a node.
type EdgeMap<'a> = HashMap<Node<'a>, Edge<'a>>;

// TagMap is a collection of tags, classified by their name.
type TagMap<'t> = HashMap<&'t str, &'t Tag>;

// Graph summarizes a performance profile into a format that is
// suitable for visualization.
#[derive(Clone, Debug)]
struct Graph<'graph> {
    nodes: &'graph Node<'graph>,
}

// Node is an entry on a profiling report. It represents a unique
// program location.
#[derive(Clone, Debug)]
struct Node<'node> {
    // Info describes the source location associated to this node.
    info: NodeInfo<'node>,

    // Function represents the function that this node belongs to. On
    // graphs with sub-function resolution (eg line number or
    // addresses), two nodes in a NodeMap that are part of the same
    // function have the same value of Node.Function. If the Node
    // represents the whole function, it points back to itself.
    function: &'node Node<'node>,

    // Values associated to this node. Flat is exclusive to this node,
    // Cum includes all descendents.
    flat: i64,
    flat_div: i64,
    cum: i64,
    cum_div: i64,
    // TODO edge lifetime??
    // In and out Contains the nodes immediately reaching or reached by
    // this node.
    r#in: EdgeMap<'node>,
    out: EdgeMap<'node>,
    // label_tags:
}

// NodeInfo contains the attributes for a node.
#[derive(Copy, Clone, Debug)]
struct NodeInfo<'n> {
    name: &'n str,
    orig_name: &'n str,
    address: u64,
    file: &'n str,
    start_line: i64,
    lineno: i64,
    objfile: &'n str,
}

// Edge contains any attributes to be represented about edges in a graph.
#[derive(Copy, Clone, Debug)]
struct Edge<'edge> {
    src: &'edge Node<'edge>,
    dest: &'edge Node<'edge>,
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
struct Tag<'t> {
    name: &'t str,
    // Describe the value, "" for non-numeric tags
    unit: &'t str,
    value: i64,
    flat: i64,
    flat_div: i64,
    cum: i64,
    cum_div: i64,
}

impl<'t> Tag<'t> {
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
