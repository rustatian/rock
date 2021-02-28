#![warn(missing_debug_implementations)]
#![allow(dead_code)] //TODO remove later
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[cfg(target_os = "windows")]
const SEPARATOR: &str = "\\";

#[cfg(target_os = "linux")]
const SEPARATOR: &str = "/";

// EdgeMap is used to represent the incoming/outgoing edges from a node.
type EdgeMap = HashMap<Node, Edge>;

// TagMap is a collection of tags, classified by their name.
type TagMap = HashMap<String, Tag>;

// Graph summarizes a performance profile into a format that is
// suitable for visualization.
#[derive(Clone, Debug)]
struct Graph {
    nodes: Node,
}

#[derive(Debug)]
struct Options<T, U>
where
    T: Fn(&[i64]) -> i64,
    U: Fn(i64, String) -> String,
{
    sample_value: T,
    sample_mean_divisor: T,
    format_tag: U,
    obj_names: bool,
    orig_fn_names: bool,

    call_tree: bool,
    drop_negative: bool,

    kept_nodes: HashMap<NodeInfo, bool>,
}

// Node is an entry on a profiling report. It represents a unique
// program location.
#[derive(Clone, Debug, Eq, Default)]
struct Node {
    // Info describes the source location associated to this node.
    info: NodeInfo,

    // Function represents the function that this node belongs to. On
    // graphs with sub-function resolution (eg line number or
    // addresses), two nodes in a NodeMap that are part of the same
    // function have the same value of Node.Function. If the Node
    // represents the whole function, it points back to itself.
    function: Box<Node>,

    // Values associated to this node. Flat is exclusive to this node,
    // Cum includes all descendent.
    flat: i64,
    flat_div: i64,
    cum: i64,
    cum_div: i64,
    // TODO edge lifetime??
    // In and out Contains the nodes immediately reaching or reached by
    // this node.
    r#in: HashMap<Node, Edge>,
    out: HashMap<Node, Edge>,
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

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.flat == other.flat
            && self.info == other.info
            && self.function == other.function
            && self.flat_div == other.flat_div
            && self.cum == other.cum
            && self.cum_div == other.cum_div
            && self.r#in == other.r#in
            && self.out == other.out
            && self.label_tags == other.label_tags
            && self.numeric_tags == other.numeric_tags
    }
}

impl Node {
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

    // AddToEdge increases the weight of an edge between two nodes. If
    // there isn't such an edge one is created.
    pub fn add_to_edge(&mut self, to: &mut Node, v: i64, residual: bool, inline: bool) {
        self.add_to_edge_div(to, 0, v, residual, inline);
    }

    // AddToEdgeDiv increases the weight of an edge between two nodes. If
    // there isn't such an edge one is created.
    pub fn add_to_edge_div(
        &mut self,
        to: &mut Node,
        dv: i64,
        v: i64,
        residual: bool,
        inline: bool,
    ) {
        if let Some(node1) = self.r#in.get(to) {
            if let Some(node2) = self.out.get(self) {
                if node1 != node2 {
                    panic!("asymmetric edges {:?} {:?}", self, to);
                }
            }
        }

        // can be nil
        if let Some(e) = self.r#in.get_mut(to) {
            e.weight_div += dv;
            e.weight += v;
            if residual {
                e.residual = true;
            }
            if !inline {
                e.inline = false;
            }
            return;
        }

        let info = Edge {
            src: self.clone(),
            dest: to.clone(),
            weight_div: dv,
            weight: v,
            residual,
            inline,
        };
        self.out.insert(to.clone(), info.clone());
        to.r#in.insert(self.clone(), info);
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

impl NodeInfo {
    // PrintableName calls the Node's Formatter function with a single space separator.
    pub fn printable_name(&self) -> String {
        self.name_components().join(" ")
    }

    // NameComponents returns the components of the printable name to be used for a node.
    pub fn name_components(&self) -> Vec<String> {
        let mut name = vec![];

        if self.address != 0 {
            name.push(format!("{:x}", self.address));
        }

        if !self.name.is_empty() {
            name.push(self.name.to_string());
        }

        if self.lineno != 0 {
            name.push(format!("{}:{}", self.file, self.lineno));
        }

        if !self.file.is_empty() {
            name.push(self.file.to_string());
        }

        if !self.name.is_empty() {
            name.push(self.name.to_string());
        }

        if !self.objfile.is_empty() {
            name.push(format!("[{}]", get_basename(&self.objfile, SEPARATOR)));
        }

        if name.is_empty() {
            name.push("<unknown>".to_string());
        }

        name
    }
}

fn get_basename<'a>(path: &'a str, pat: &'a str) -> String {
    let mut parts = path.rsplit(pat);

    match parts.next() {
        None => "".into(),
        Some(path) => path.into(),
    }
}

// Edge contains any attributes to be represented about edges in a graph.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Edge {
    src: Node,
    dest: Node,
    // The summary weight of the edge
    weight: i64,
    weight_div: i64,
    // residual edges connect nodes that were connected through a
    // separate node, which has been removed from the report.
    residual: bool,
    // An inline edge represents a call that was inlined into the caller.
    inline: bool,
}

impl Edge {
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

#[cfg(test)]
mod tests {
    use crate::graph::{get_basename, SEPARATOR};

    #[test]
    fn test_get_basename() {
        assert_eq!(get_basename("/usr/data", SEPARATOR), "data");
        assert_eq!(get_basename("/", SEPARATOR), "");
        assert_eq!(get_basename("/root", SEPARATOR), "root");
    }
}
