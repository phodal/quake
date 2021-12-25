use indexmap::IndexMap;

use crate::transflow::element_define::{ElementDefine, EventListener};
use crate::transflow::flow::{Flow, Mapping};
use crate::transflow::Transflow;

/// Javascript Transflow Code generate
pub struct JsFlowCodegen {}

impl JsFlowCodegen {
    /// generate from typescript interface
    ///
    /// - a transflow function nme will start with `tl_{:}` prefix
    ///
    /// ```javascript
    ///   el.setAttribute('entries', JSON.stringify({
    //     items: ['blog', 'todo']
    //   }));
    //   el.setAttribute('data', JSON.stringify(data));
    /// ```
    ///
    pub fn gen_element(trans: &Transflow, element: Option<ElementDefine>) -> Vec<String> {
        let mut vec = vec![];
        for flow in &trans.flows {
            let mut func = String::new();
            let start = format!(
                "const tl_{:} = async (context, commands) => {{\n",
                trans.name
            );
            func.push_str(start.as_str());
            func.push_str(
                format!("  const el = document.createElement('{:}');\n", flow.to).as_str(),
            );
            func.push('\n');

            for item in &flow.from {
                let fetch = Self::gen_fetch(flow, item);
                func.push_str(fetch.as_str());
                func.push('\n');
            }

            let params = Self::gen_params(flow);
            Self::gen_data_attribute(&flow, &mut func, params);

            if let Some(el) = &element {
                Self::gen_events(&mut func, &el.events)
            }

            func.push_str("  return el;\n");
            func.push_str("}\n");
            vec.push(func)
        }

        vec
    }

    /// generate transform
    pub fn gen_transform(trans: &Transflow, element: Option<ElementDefine>) -> Vec<String> {
        let mut vec = vec![];
        for flow in &trans.flows {
            let mut func = String::new();

            let params = Self::gen_params(flow);

            func.push_str(format!("function {:}({:}) {{\n", &flow.name, params).as_str());
            func.push_str("  let results = [];\n");

            // from normal map
            if flow.map.is_some() {
                let string = Self::gen_flow_map(&flow, &element);
                func.push_str(&*string);
            }

            // for from yaml mapping
            if flow.mapping.is_some() {
                let mappings = JsFlowCodegen::gen_obj_mapping(flow.mapping.as_ref().unwrap());
                func.push_str(mappings.join("\n").as_str());
            } else {
                let results = JsFlowCodegen::gen_obj_concat(&flow.from);
                func.push_str(results.join("").as_str());
            }

            func.push_str("  return results;\n");
            func.push_str("}\n");
            vec.push(func)
        }

        vec
    }

    fn gen_flow_map(flow: &&Flow, element: &Option<ElementDefine>) -> String {
        let mut output = String::new();
        let flow_map = Self::build_flow_map_prop(flow, element);
        for from in &flow.from {
            let mut string = format!(
                "  for (let {:} of {:}s) {{\n    results.push({{",
                from, from
            );

            let str = match flow_map.get(from) {
                None => "".to_string(),
                Some(params) => {
                    let mut out: Vec<String> = vec![];
                    for param in params {
                        out.push(format!("\n      {:}: {:}", param.0, param.1));
                    }

                    out.join(",")
                }
            };

            string.push_str(&*str);

            string.push_str("\n    })\n  }\n");
            output.push_str(&*string);
        }
        output
    }

    fn build_flow_map_prop(
        flow: &Flow,
        element: &Option<ElementDefine>,
    ) -> IndexMap<String, IndexMap<String, String>> {
        let maps = flow.map.as_ref().unwrap();
        let mut mapping = Self::create_default_prop_map(&flow, element);

        for stream in maps {
            let mut prop_map: IndexMap<String, String> = IndexMap::new();
            if !stream.operators.is_empty() {
                let mut str = stream.source_prop.to_string();
                for operator in &stream.operators {
                    str = format!(
                        "{:}.{:}({:})",
                        str,
                        operator.operator,
                        operator.params.join(",")
                    );
                }

                prop_map.insert(stream.target_prop.clone(), str);
            } else {
                prop_map.insert(stream.target_prop.clone(), stream.source_prop.clone());
            }

            match mapping.get_mut(&*stream.source_type) {
                None => {
                    mapping.insert(stream.source_type.clone(), prop_map);
                }
                Some(origin) => {
                    origin.extend(prop_map);
                }
            }
        }

        println!("{:?}", mapping);
        mapping
    }

    fn create_default_prop_map(
        flow: &&Flow,
        element: &Option<ElementDefine>,
    ) -> IndexMap<String, IndexMap<String, String>> {
        let mut mapping: IndexMap<String, IndexMap<String, String>> = IndexMap::new();
        if element.is_some() {
            for from in &flow.from {
                let mut from_map: IndexMap<String, String> = IndexMap::new();
                for (key, _) in element.as_ref().unwrap().data_map() {
                    from_map.insert(key.clone(), format!("{:}.{:}", from, key));
                }
                mapping.insert(from.clone(), from_map);
            }
        }
        mapping
    }

    fn gen_events(func: &mut String, events: &[EventListener]) {
        for event in events {
            let event_code = format!(
                "  el.addEventListener('{:}', function (event) {{
    let data = event.detail;
    console.log(data);
  }});\n\n",
                event.event_name
            );
            func.push_str(event_code.as_str())
        }
    }

    fn gen_data_attribute(flow: &&Flow, func: &mut String, params: String) {
        let data = format!("  let data = {:}({:});\n", &flow.name, params);
        func.push_str(data.as_str());
        func.push_str("  el.setAttribute('data', JSON.stringify(data));\n\n");
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

    fn gen_obj_concat(entries: &[String]) -> Vec<String> {
        let mut vec = vec![];
        for entry in entries {
            let string = format!("  results = results.concat({:}s);\n", &entry);
            vec.push(string);
        }

        vec
    }

    fn gen_obj_mapping(mappings: &[Mapping]) -> Vec<String> {
        let mut vec = vec![];
        for mapping in mappings {
            let mut loop_expr = String::new();
            let string = format!(
                "  for (let {:} of {:}s) {{\n    results.push(",
                &mapping.entry, &mapping.entry
            );

            loop_expr.push_str(string.as_str());

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

    fn gen_fetch(flow: &Flow, item: &str) -> String {
        let mut filter = "".to_string();
        if flow.filter.is_some() {
            let filter_str = flow.filter.as_ref().unwrap();
            filter = format!(", '', {{\n    filter: '{:}'\n  }}", &filter_str);
        }

        let fetch = format!(
            "  let {:}s = await Quake.query('{:}'{:});\n",
            item, item, filter
        );

        fetch
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::option::Option::None;
    use std::path::PathBuf;

    use crate::entry::EntryDefine;
    use crate::quake::QuakeTransflowNode;
    use crate::transflow::element_define::ElementDefine;
    use crate::transflow::js_flow_codegen::JsFlowCodegen;
    use crate::transflow::Transflow;

    fn entry_defines() -> Vec<EntryDefine> {
        let yaml = "
- type: todo
  display: Todo
  properties:
    - title: Title
    - content: Body
    - author: Author

- type: blog
  display: Blog
  properties:
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

        let path = fixtures.join("transflows").join("transflows.yaml");
        let content = fs::read_to_string(path).unwrap();
        let flows: Vec<Transflow> = serde_yaml::from_str(&*content).unwrap();

        let code = JsFlowCodegen::gen_transform(&flows[0], None);

        let except_path = fixtures.join("codegen").join("todos_blogs.js");
        let content = fs::read_to_string(except_path).unwrap();

        assert_eq!(content, code[0]);
    }

    #[cfg(not(windows))]
    #[test]
    fn from_transflow_string() {
        let define = "transflow show_calendar { from('todo','blog').to(<quake-calendar>); }";
        let node = QuakeTransflowNode::from_text(define).unwrap();
        let flow = Transflow::from(entry_defines(), node);
        let code = JsFlowCodegen::gen_transform(&flow, None);

        assert_eq!(
            "function from_todo_blog_to_quake_calendar(todos, blogs) {
  let results = [];
  results = results.concat(todos);
  results = results.concat(blogs);
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
            "transflow show_calendar { from('todo','blog').to('record'), from('record').to(<quake-calendar>); }";
        let flow = QuakeTransflowNode::from_text(define).unwrap();

        let flow = Transflow::from(entry_defines(), flow);

        let code = JsFlowCodegen::gen_transform(&flow, None);

        assert_eq!(
            "function from_todo_blog_to_record(todos, blogs) {
  let results = [];
  results = results.concat(todos);
  results = results.concat(blogs);
  return results;
}
",
            code[0]
        );

        assert_eq!(
            "function from_record_to_quake_calendar(records) {
  let results = [];
  results = results.concat(records);
  return results;
}
",
            code[1]
        )
    }

    #[cfg(not(windows))]
    #[test]
    fn gen_element() {
        let define = "transflow show_calendar { from('todo','blog').to(<quake-calendar>); }";
        let node = QuakeTransflowNode::from_text(define).unwrap();
        let flow = Transflow::from(entry_defines(), node);
        let code = JsFlowCodegen::gen_element(&flow, None);

        let except_path = PathBuf::from("_fixtures")
            .join("transflow")
            .join("show_calendar.code");

        let except = fs::read_to_string(except_path).unwrap();

        assert_eq!(except, code[0])
    }

    #[cfg(not(windows))]
    #[test]
    fn gen_element_with_wc() {
        let define = "transflow show_calendar { from('todo','blog').to(<quake-calendar>); }";
        let node = QuakeTransflowNode::from_text(define).unwrap();
        let flow = Transflow::from(entry_defines(), node);

        let mut element = ElementDefine::default();
        element.add_event("onSave");
        element.add_event("onChange");

        let code = JsFlowCodegen::gen_element(&flow, Some(element));

        let except_path = PathBuf::from("_fixtures")
            .join("transflow")
            .join("event_with_calendar.code");

        let except = fs::read_to_string(except_path).unwrap();

        assert_eq!(except, code[0])
    }

    #[cfg(not(windows))]
    #[test]
    fn gen_element_with_filter() {
        let define = "transflow show_calendar {
         from('todo','blog').to(<quake-calendar>).filter('created_date > 2021.01.01 and created_date < 2021.12.31');
}";

        let node = QuakeTransflowNode::from_text(define).unwrap();
        let flow = Transflow::from(entry_defines(), node);
        let code = JsFlowCodegen::gen_element(&flow, None);

        let except_path = PathBuf::from("_fixtures")
            .join("transflow")
            .join("get_todos_blogs_with_filter.code");

        let except = fs::read_to_string(except_path).unwrap();

        assert_eq!(except, code[0])
    }

    fn create_blog_data_prop() -> Vec<HashMap<String, String>> {
        let mut properties = vec![];
        let mut maps: HashMap<String, String> = HashMap::new();
        maps.insert("title".to_string(), "String".to_string());
        properties.push(maps);
        let mut maps: HashMap<String, String> = HashMap::new();
        maps.insert("content".to_string(), "String".to_string());
        properties.push(maps);

        properties
    }

    #[cfg(not(windows))]
    #[test]
    fn gen_transform_with_filter_map() {
        let define = "transflow show_calendar {
         from('todo','blog')
            .to(<quake-calendar>)
            .filter('created_date > 2021.01.01 and created_date < 2021.12.31')
            .map('blog.content => content | uppercase | substring(1, 150), blog.created_date => created_date');
}";

        let mut element = ElementDefine::new("quake-calendar".to_string());
        element.data_properties = create_blog_data_prop();

        let node = QuakeTransflowNode::from_text(define).unwrap();
        let flow = Transflow::from(entry_defines(), node);
        let code = JsFlowCodegen::gen_transform(&flow, Some(element));

        let except_path = PathBuf::from("_fixtures")
            .join("transflow")
            .join("get_todos_blogs_with_filter_map.code");

        let except = fs::read_to_string(except_path).unwrap();

        assert_eq!(except, code[0])
    }
}
