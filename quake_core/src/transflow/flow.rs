use std::collections::HashMap;

use crate::entry::EntryDefine;
use crate::parser::quake::{QuakeTransflowNode, Route};

/// Transflow defines the data process flow
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Transflow {
    pub name: String,
    #[serde(skip_serializing)]
    pub defines_map: Option<HashMap<String, EntryDefine>>,
    pub flows: Vec<Flow>,
    pub target: String,
}

impl Transflow {
    /// parse from transflow DSL which is [QuakeTransflowNode]
    ///
    /// # Examples
    ///
    /// transflow define:
    ///
    /// ```bash
    /// transflow { from('todo','blog').to(<quake-calendar>); }
    /// ```
    /// - `quake-calendar` with be [Transflow.target]
    /// - `defines_map` with mapping to `todo` and `blog` [EntryDefine]
    /// - `flows` with have informations of `from('todo','blog').to(<quake-calendar>`
    ///
    pub fn from(defines: Vec<EntryDefine>, node: QuakeTransflowNode) -> Transflow {
        let mut transflow = Transflow::default();
        transflow.name = node.name;

        let mut entries_map: HashMap<String, &EntryDefine> = HashMap::new();
        for define in &defines {
            entries_map.insert(define.entry_type.clone(), define);
        }

        let mut map = HashMap::new();
        for route in &node.routes {
            if route.is_end_way {
                transflow.target = route.to.clone();
            } else if let Some(some) = entries_map.get(route.to.as_str()) {
                map.insert(route.to.clone(), (*some).clone());
            }

            for from in &route.from {
                if let Some(some) = entries_map.get(from.as_str()) {
                    map.insert(from.clone(), (*some).clone());
                }
            }
        }
        transflow.defines_map = Some(map);

        for route in node.routes {
            transflow.flows.push(Flow::from_route(route));
        }

        transflow
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Mapping {
    pub entry: String,
    pub source: Vec<String>,
    pub target: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Flow {
    pub name: String,
    pub from: Vec<String>,
    pub to: String,
    #[serde(rename = "map")]
    pub mappings: Option<Vec<Mapping>>,
    pub filter: String,
}

impl Flow {
    pub fn from_route(route: Route) -> Flow {
        Flow {
            name: route.name,
            from: route.from.clone(),
            to: route.to.clone(),
            mappings: None,
            filter: route.filter,
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

#[cfg(test)]
mod tests {
    use crate::entry::EntryDefine;
    use crate::quake::QuakeTransflowNode;
    use crate::transflow::flow::Transflow;
    use std::fs;
    use std::path::PathBuf;

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
    fn serialize_from_file() {
        let path = PathBuf::from("../")
            .join("examples")
            .join("_quake")
            .join("transflows.yaml");

        let content = fs::read_to_string(path).unwrap();
        let _flows: Vec<Transflow> = serde_yaml::from_str(&*content).unwrap();
    }

    #[test]
    fn stringify_defines() {
        let define = "transflow show_calendar { from('todo','blog').to(<quake-calendar>); }";
        let flow = QuakeTransflowNode::from_text(define).unwrap();

        let flow = Transflow::from(entry_defines(), flow);

        println!("{:}", serde_yaml::to_string(&flow).unwrap());

        assert_eq!(2, flow.defines_map.unwrap().len());
        assert_eq!(1, flow.flows.len());
        assert_eq!("quake-calendar", flow.flows[0].to);
        assert_eq!("quake-calendar", flow.target);
    }
}
