#![warn(missing_debug_implementations)]
#![allow(dead_code)] //temporary
use std::collections::HashMap;

// EdgeMap is used to represent the incoming/outgoing edges from a node.
type EdgeMap<'a, 'b> = HashMap<Node<'a>, Edge<'b>>;

// Graph summarizes a performance profile into a format that is
// suitable for visualization.
#[derive(Copy, Clone, Debug)]
struct Graph<'graph> {
    nodes: &'graph Node<'graph>,
}

// Node is an entry on a profiling report. It represents a unique
// program location.
#[derive(Copy, Clone, Debug)]
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
