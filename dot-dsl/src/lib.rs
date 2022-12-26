pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::{edge::Edge,node::Node};

    pub mod graph_items{
        pub mod edge{
            use std::collections::HashMap;

            #[derive(Debug,Default,Clone,PartialEq,Eq)]
            pub struct Edge{
                pub from:String,
                pub to:String,
                pub attrs: HashMap<String,String>,
            }

            impl Edge{
                pub fn new(from: &str, to: &str) -> Self{
                    Self{
                        from: from.to_string(),
                        to: to.to_string(),
                        ..Default::default()
                    }
                }
                pub fn with_attrs(mut self, attrs:&[(&str,&str)])->Self{
                    self.attrs=attrs
                        .iter()
                        .map(|(k,v)| (k.to_string(),v.to_string()))
                        .collect();
                    self
                }
                pub fn attr(&self,key:&str)->Option<&str>{
                    self.attrs.get(key).map(|s| &**s) // <-- **s
                }
            }
        }

        pub mod node{
            use std::collections::HashMap;
            #[derive(Debug,Default,Clone,PartialEq,Eq)]

            pub struct Node{
                pub name: String,
                pub attrs: HashMap<String,String>,
            }

            impl Node{
                pub fn new(value:&str)->Self{
                    Self{
                        name: value.to_string(),
                        ..Default::default()
                    }
                }
                pub fn with_attrs(mut self,attrs:&[(&str,&str)])->Self{
                    self.attrs = attrs
                        .iter()
                        .map(|(k,v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }
                pub fn attr(&self, key:&str)-> Option<&str>{
                    self.attrs.get(key).map(|s| &**s) // <--**s
                }
            }
        }
    }

    #[derive(Debug,Default,Clone)]
    pub struct Graph{
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String,String>,
    }

    impl Graph {
        pub fn new() -> Self {
            //unimplemented!("Construct a new Graph struct.");
            Self::default()
        }
        pub fn with_nodes(mut self,nodes:&[Node])->Self{
            self.nodes=nodes.to_vec();
            self
        }
        pub fn with_edges(mut self,edges:&[Edge])->Self{
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self,attrs:&[(&str,&str)])->Self{
            self.attrs = attrs
                .iter()
                .map(|(k,v)|(k.to_string(),v.to_string()))
                .collect();
            self
        }
        pub fn node(&self, name:&str) -> Option<&Node>{
            self.nodes.iter().find(|n| n.name==name)
        }
    }
}
