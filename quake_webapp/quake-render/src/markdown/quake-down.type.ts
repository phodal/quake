export enum CodeType {
  Unknown = "Unknown",
  Normal = "Normal",
  Graph = "Graph",
  Chart = "Chart",
  Example = "Example",
  Repl = "Repl",
  Transflow = "Transflow",
  Run = "Run",
}

export function CodeTypeFromStr(text: String) {
  switch (text.toLocaleLowerCase()) {
    case "graph":
      return CodeType.Graph;
    case "chart":
      return CodeType.Chart;
    case "example":
      return CodeType.Example;
    case "repl":
      return CodeType.Repl;
    case "run":
      return CodeType.Run;
    case "transflow":
      return CodeType.Transflow;
    default:
      return CodeType.Unknown;
  }
}

export namespace QuakeDownType {
  export interface Table {
    type: 'table',
    align: any[],
    rows: any[],
    header: any[],
  }
  export interface Code {
    type: 'code',
    code_type: CodeType,
    code_param: string,
    lang: string,
    text: string,
  }
}
