pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::{edge::Edge, node::Node};

    pub trait WithAttrs<'a> {
        fn with_attrs(
            ah: &'a mut HashMap<String, String>,
            attrs: &[(&str, &str)],
        ) -> &'a HashMap<String, String> {
            attrs.iter().for_each(|(k, v)| {
                ah.insert(k.to_string(), v.to_string());
            });
            ah
        }
    }

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

        pub fn with_nodes<S: AsRef<[T]>, T: AsRef<Node>>(mut self, nodes: S) -> Self {
            nodes
                .as_ref()
                .iter()
                .for_each(|n| self.nodes.push(n.as_ref().clone()));
            self
        }
        pub fn with_edges<S: AsRef<[T]>, T: AsRef<Edge>>(mut self, edges: S) -> Self {
            edges
                .as_ref()
                .iter()
                .for_each(|e| self.edges.push(e.as_ref().clone()));
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

            impl AsRef<Self> for Node {
                fn as_ref(&self) -> &Self {
                    &self
                }
            }

            // impl<'a> crate::graph::WithAttrs<'a> for Node {}
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
            impl AsRef<Self> for Edge {
                fn as_ref(&self) -> &Self {
                    &self
                }
            }
        }
    }
}
