#![warn(missing_debug_implementations)]
#![allow(dead_code)] //temporary

// Graph summarizes a performance profile into a format that is
// suitable for visualization.
struct Graph<'graph> {
    nodes: &'graph Node<'graph>,
}

// Node is an entry on a profiling report. It represents a unique
// program location.
struct Node<'node> {
    // Info describes the source location associated to this node.
    info: NodeInfo<'node>,

    // Function represents the function that this node belongs to. On
    // graphs with sub-function resolution (eg line number or
    // addresses), two nodes in a NodeMap that are part of the same
    // function have the same value of Node.Function. If the Node
    // represents the whole function, it points back to itself.
    function: &'node Node<'node>
}

// NodeInfo contains the attributes for a node.
struct NodeInfo<'n> {
    name: &'n str,
    orig_name: &'n str,
    address: u64,
    file: &'n str,
    start_line: i64,
    lineno: i64,
    objfile: &'n str,
}
