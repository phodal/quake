import { NodeModel, DefaultPortModel } from "@projectstorm/react-diagrams";
import { BaseModelOptions } from "@projectstorm/react-canvas-core";

export interface ConceptNodeModelOptions extends BaseModelOptions {
  color?: string;
}

export class ConceptNodeModel extends NodeModel {
  color: string;

  constructor(options: ConceptNodeModelOptions = {}) {
    super({
      ...options,
      type: "concept"
    });
    this.color = options.color || "rgb(0,192,255)";

    // setup an in and out port
    this.addPort(
      new DefaultPortModel({
        in: true,
        name: "in"
      })
    );
    this.addPort(
      new DefaultPortModel({
        in: false,
        name: "out"
      })
    );
  }

  serialize() {
    return {
      ...super.serialize(),
      color: this.color
    };
  }

  deserialize(event: any): void {
    super.deserialize(event);
    this.color = event.data.color;
  }
}
