use crate::entry::entry_paths::EntryPaths;
use crate::transflow::js_flow_codegen::JsFlowCodegen;
use crate::transflow::Transflow;
use std::error::Error;
use std::fs;
use std::option::Option::None;
use std::path::PathBuf;

pub fn dump_flows(path: PathBuf) -> Result<String, Box<dyn Error>> {
    let flow_path = path.join(EntryPaths::quake()).join(EntryPaths::transflow());
    let content = fs::read_to_string(flow_path)?;
    let flows: Vec<Transflow> = serde_yaml::from_str(&*content)?;

    let mut scripts = vec![];
    for flow in flows {
        let script = flow_to_script(&flow);

        scripts.push(script);
    }

    Ok(scripts.join("\n"))
}

pub fn flow_to_script(flow: &Transflow) -> String {
    let trans = JsFlowCodegen::gen_transform(flow, None);
    let els = JsFlowCodegen::gen_element(flow, None);

    let route = format!(
        "Quake.transflow.add({{name:'{:}',action:tl_{:}}})",
        &flow.name, &flow.name
    );
    let bind = format!("Quake.flows['tl_{:}'] = tl_{:}", &flow.name, &flow.name);

    let script = format!(
        "{:}\n{:}\n{:}\n{:}\n",
        trans.join("\n"),
        els.join("\n"),
        route,
        bind
    );
    script
}
