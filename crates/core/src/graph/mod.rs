#![warn(missing_debug_implementations)]

// Graph summarizes a performance profile into a format that is
// suitable for visualization.
struct Graph<'graph> {
    nodes: &'graph Node<'graph>,
}

// Node is an entry on a profiling report. It represents a unique
// program location.
struct Node<'node> {
    info: NodeInfo<'node>,
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
