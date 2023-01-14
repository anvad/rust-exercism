// i had used a convoluted signature for `with_nodes()` and
//  `with_edges()` methods using `AsRef` in my previous implementation.
// looking at rochard4u's solution, i see i could have just used `&[Node]` etc.
// i see roybcr's solution used `.cloned()` to succinctly implement `with_nodes()`

pub mod graph {
    use self::graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.iter().for_each(|(k, v)| {
                self.attrs.insert(k.to_string(), v.to_string());
            });
            self
        }

        pub fn node(&self, node_label: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.label == node_label)
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Default, Clone, PartialEq, Eq)]
            pub struct Node {
                pub label: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Self {
                        label: label.to_string(),
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs.insert(k.to_string(), v.to_string());
                    });
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| &v[..])
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Default, Clone, PartialEq, Eq)]
            pub struct Edge {
                src: String,
                dst: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(src: &str, dst: &str) -> Self {
                    Self {
                        src: src.to_string(),
                        dst: dst.to_string(),
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs.insert(k.to_string(), v.to_string());
                    });
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| &v[..])
                }
            }
        }
    }
}
