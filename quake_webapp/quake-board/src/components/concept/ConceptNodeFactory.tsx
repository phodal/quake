import * as React from "react";
import { ConceptNodeModel } from "./ConceptNodeModel";
import { ConceptNodeWidget } from "./ConceptNodeWidget";
import { AbstractReactFactory } from "@projectstorm/react-canvas-core";
import { DiagramEngine } from "@projectstorm/react-diagrams-core";

export class ConceptNodeFactory extends AbstractReactFactory<ConceptNodeModel, DiagramEngine> {
  constructor() {
    super("concept");
  }

  generateModel(initialConfig: any) {
    return new ConceptNodeModel();
  }

  generateReactWidget(event: any): JSX.Element {
    return (
      <ConceptNodeWidget
        engine={ this.engine as DiagramEngine }
        node={ event.model }
      />
    );
  }
}
