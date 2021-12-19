use crate::entry::entry_paths::EntryPaths;
use crate::transflow::js_flow_codegen::JsFlowCodegen;
use crate::transflow::Transflow;
use std::fs;
use std::path::PathBuf;

pub fn dump_flows(path: PathBuf) -> String {
    let flow_path = path.join(EntryPaths::quake()).join(EntryPaths::transflow());
    let content = fs::read_to_string(flow_path).unwrap();
    let flows: Vec<Transflow> = serde_yaml::from_str(&*content).unwrap();

    let mut scripts = vec![];
    for flow in flows {
        let script = flow_to_script(&flow);

        scripts.push(script);
    }

    scripts.join("\n")
}

pub fn flow_to_script(flow: &Transflow) -> String {
    let trans = JsFlowCodegen::gen_transform(flow);
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
