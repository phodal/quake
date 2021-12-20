import {Build} from "@stencil/core";
import init, {flow_to_code, parse_action} from "@quakeworks/quake_wasm";
import axios from "axios";

export async function init_wasm() {
  await init()
}

export async function parseAction(str: string) {
  if (!Build.isDev) {
    return JSON.parse(parse_action(str));
  } else {
    return axios.get('/action/query/', {
      params: {
        input: this.query.substring(1,)
      }
    }).then((res) => {return res.data});
  }
}

// demo: from('blog').to(<ion-button>)
export async function createTransflow(flow_name: string, flow: string) {
  if (!Build.isDev) {
    let transflow = `transflow ${flow_name} { ${flow} } `
    let defines = await axios.get('/entry/defines').then((res) => {
      return res.data;
    });

    return flow_to_code(transflow, JSON.stringify(defines));
  } else {
    return axios.post(`/transflow/translate/${flow_name}`, {
      flow: flow
    }).then((res) => {return res.data});
  }
}
