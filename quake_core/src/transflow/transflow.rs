use crate::entry::EntryDefine;
use crate::parser::quake::{QuakeTransflowNode, Route};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Transflow {
    pub name: String,
    pub defines_map: HashMap<String, EntryDefine>,
    pub flows: Vec<Flow>,
    pub target: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Mapping {
    pub entry: String,
    pub source: Vec<String>,
    pub target: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Filter {}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Flow {
    pub name: String,
    pub from: Vec<String>,
    pub to: String,
    pub mappings: Vec<Mapping>,
    pub filters: Vec<Filter>,
}

impl Flow {
    pub fn from_route(route: Route) -> Flow {
        Flow {
            name: route.name,
            from: route.from.clone(),
            to: route.to.clone(),
            mappings: vec![],
            filters: vec![],
        }
    }
}

impl Default for Transflow {
    fn default() -> Self {
        Transflow {
            name: "".to_string(),
            defines_map: Default::default(),
            flows: vec![],
            target: "".to_string(),
        }
    }
}

impl Transflow {
    pub fn from(defines: Vec<EntryDefine>, node: QuakeTransflowNode) -> Transflow {
        let mut transflow = Transflow::default();

        let mut entries_map: HashMap<String, &EntryDefine> = HashMap::new();
        for define in &defines {
            entries_map.insert(define.entry_type.clone(), define);
        }

        for route in &node.routes {
            if route.is_end_way {
                transflow.target = route.to.clone();
            } else {
                if let Some(some) = entries_map.get(route.to.as_str()) {
                    transflow
                        .defines_map
                        .insert(route.to.clone(), (*some).clone());
                }
            }

            for from in &route.from {
                if let Some(some) = entries_map.get(from.as_str()) {
                    transflow.defines_map.insert(from.clone(), (*some).clone());
                }
            }
        }

        for route in node.routes {
            transflow.flows.push(Flow::from_route(route));
        }

        transflow
    }
}

#[cfg(test)]
mod tests {
    use crate::entry::EntryDefine;
    use crate::quake::QuakeTransflowNode;
    use crate::transflow::transflow::Transflow;

    fn entry_defines() -> Vec<EntryDefine> {
        let yaml = "
- type: todo
  display: Todo
  fields:
    - title: Title
    - content: Body
    - author: Author

- type: blog
  display: Blog
  fields:
    - title: Title
    - content: Body
    - author: Author
";

        let entries: Vec<EntryDefine> = serde_yaml::from_str(yaml).unwrap();
        entries
    }

    #[test]
    fn stringify_defines() {
        let define = "transflow { from('todo','blog').to(<quake-calendar>); }";
        let flow = QuakeTransflowNode::from_text(define).unwrap();

        let flow = Transflow::from(entry_defines(), flow);

        println!("{:?}", flow);
        assert_eq!(2, flow.defines_map.len());
        assert_eq!(1, flow.flows.len());
        assert_eq!("quake-calendar", flow.target);
    }
}
