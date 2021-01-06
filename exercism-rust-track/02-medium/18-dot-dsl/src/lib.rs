pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(PartialEq, Eq, Clone, Debug)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_owned(),
                        to: to.to_owned(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, new_attrs: &[(&str, &str)]) -> Self {
                    self.attrs = new_attrs.iter()
                        .map(|pair| (pair.0.to_owned(), pair.1.to_owned()))
                        .collect();

                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|value| value.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Eq, Clone, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_owned(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, new_attrs: &[(&str, &str)]) -> Self {
                    self.attrs = new_attrs.iter()
                        .map(|pair| (pair.0.to_owned(), pair.1.to_owned()))
                        .collect();

                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|value| value.as_str())
                }
            }
        }
    }

    use std::collections::HashMap;

    use graph_items::{edge::Edge, node::Node};

    pub struct Graph {
        pub edges: Vec<Edge>,
        pub nodes: Vec<Node>,
        pub attrs: std::collections::HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                edges: vec![],
                nodes: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, new_nodes: &[Node]) -> Self {
            self.nodes = new_nodes.to_vec();

            self
        }

        pub fn with_edges(mut self, new_edges: &[Edge]) -> Self {
            self.edges = new_edges.to_vec();

            self
        }

        pub fn with_attrs(mut self, new_attrs: &[(&str, &str)]) -> Self {
            self.attrs = new_attrs.iter()
                .map(|pair| (pair.0.to_owned(), pair.1.to_owned()))
                .collect();

            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }

        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|value| value.as_str())
        }
    }
}
