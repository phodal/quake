use crate::transflow::transflow::{Flow, Mapping};
use crate::transflow::Transflow;

pub struct JsFlowGen {}

impl JsFlowGen {
    pub fn gen_transform(trans: &Transflow) -> Vec<String> {
        let mut vec = vec![];
        for flow in &trans.flows {
            let mut func = String::new();

            let params = Self::gen_params(&flow);

            func.push_str(format!("function {:}({:}) {{\n", &flow.name, params).as_str());
            func.push_str("  let results = [];\n");

            if flow.mappings.is_some() {
                let mappings = JsFlowGen::gen_object_forloop(&flow.mappings.as_ref().unwrap());
                func.push_str(mappings.join("\n").as_str());
            }

            func.push_str("  return results;\n");
            func.push_str("}\n");
            vec.push(func)
        }

        vec
    }

    fn gen_params(flow: &Flow) -> String {
        let mut params = String::new();
        for from in &flow.from {
            params.push_str(format!("{:}s, ", from).as_str());
        }
        params.remove(params.len() - 1);
        params.remove(params.len() - 1);
        params
    }

    fn gen_object_forloop(mappings: &Vec<Mapping>) -> Vec<String> {
        let mut vec = vec![];
        for mapping in mappings {
            let mut loop_expr = String::new();
            loop_expr.push_str(
                format!(
                    "  for (let {:} of {:}s) {{\n    results.push(",
                    &mapping.entry, &mapping.entry
                )
                .as_str(),
            );

            loop_expr.push_str(Self::gen_mapping(&mapping).as_str());
            loop_expr.push_str(")\n  }\n");
            vec.push(loop_expr);
        }

        vec
    }

    fn gen_mapping(m: &&Mapping) -> String {
        let mut result = String::new();
        result.push_str(format!("{{\n      type: {:?},", &m.entry).as_str());
        let len = m.source.len();

        if len == m.target.len() {
            for (index, field) in m.source.iter().enumerate() {
                let mut end = ",";
                if index == len - 1 {
                    end = ""
                }

                let map = format!(
                    "\n      {:}: {:}.{:}{:}",
                    &m.target[index], &m.entry, field, end
                );
                result.push_str(map.as_str());
            }
        }
        result.push_str("\n    }");

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::entry::EntryDefine;
    use crate::quake::QuakeTransflowNode;
    use crate::transflow::js_flow_codegen::JsFlowGen;
    use crate::transflow::Transflow;
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

    #[cfg(not(windows))]
    #[test]
    fn mapping_to_file() {
        let fixtures = PathBuf::from("../").join("_fixtures");

        let path = fixtures.join("transflows.yaml");
        let content = fs::read_to_string(path).unwrap();
        let flows: Vec<Transflow> = serde_yaml::from_str(&*content).unwrap();

        let code = JsFlowGen::gen_transform(&flows[0]);

        let except_path = fixtures.join("codegen").join("todos_blogs.js");
        let content = fs::read_to_string(except_path).unwrap();

        assert_eq!(content, code[0]);
    }

    #[cfg(not(windows))]
    #[test]
    fn from_transflow_string() {
        let define = "transflow { from('todo','blog').to(<quake-calendar>); }";
        let flow = QuakeTransflowNode::from_text(define).unwrap();

        let flow = Transflow::from(entry_defines(), flow);

        let code = JsFlowGen::gen_transform(&flow);

        assert_eq!(
            "function from_todo_blog_to_quake_calendar(todos, blogs) {
  let results = [];
  return results;
}
",
            code[0]
        )
    }

    #[cfg(not(windows))]
    #[test]
    fn multiple_transform() {
        let define =
            "transflow { from('todo','blog').to('record'), from('record').to(<quake-calendar>); }";
        let flow = QuakeTransflowNode::from_text(define).unwrap();

        let flow = Transflow::from(entry_defines(), flow);

        let code = JsFlowGen::gen_transform(&flow);

        assert_eq!(
            "function from_todo_blog_to_record(todos, blogs) {
  let results = [];
  return results;
}
",
            code[0]
        );

        assert_eq!(
            "function from_record_to_quake_calendar(records) {
  let results = [];
  return results;
}
",
            code[1]
        )
    }
}
