use std::error::Error;
use std::fs;
use std::option::Option::None;
use std::path::PathBuf;

use crate::entry::entry_paths::EntryPaths;
use crate::transflow::element_define::{filter_element_define, ElementDefine};
use crate::transflow::js_flow_codegen::JsFlowCodegen;
use crate::transflow::Transflow;

pub fn dump_flows(path: PathBuf) -> Result<String, Box<dyn Error>> {
    let flow_path = path.join(EntryPaths::quake()).join(EntryPaths::transflow());
    let flows: Vec<Transflow> = serde_yaml::from_str(&*fs::read_to_string(flow_path)?)?;

    let elements_define = path
        .join(EntryPaths::quake())
        .join(EntryPaths::element_define());
    let element_defines: Vec<ElementDefine> =
        serde_yaml::from_str(&*fs::read_to_string(elements_define)?)?;

    let mut scripts = vec![];
    for flow in flows {
        let script = flow_to_script(&flow, &element_defines);
        scripts.push(script);
    }

    Ok(scripts.join("\n"))
}

pub fn flow_to_script(flow: &Transflow, element_defines: &[ElementDefine]) -> String {
    let wc = filter_element_define(element_defines, flow.target.as_str());

    let trans = JsFlowCodegen::gen_transform(flow, &None);
    let els = JsFlowCodegen::gen_element(flow, &wc);

    let route = format!(
        "Quake.transflow.add({{name: '{:}', action: tl_{:}, display: {:?}}})",
        &flow.name, &flow.name, &flow.display
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
