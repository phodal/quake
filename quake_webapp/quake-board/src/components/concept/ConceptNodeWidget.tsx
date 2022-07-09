import * as React from "react";
import { DiagramEngine, PortWidget } from "@projectstorm/react-diagrams-core";
import { ConceptNodeModel } from "./ConceptNodeModel";

export interface ConceptNodeWidgetProps {
  node: ConceptNodeModel;
  engine: DiagramEngine;
}

export interface ConceptNodeWidgetState {
}

export class ConceptNodeWidget extends React.Component<ConceptNodeWidgetProps,
  ConceptNodeWidgetState> {
  constructor(props: ConceptNodeWidgetProps) {
    super(props);
    this.state = {};
  }

  render() {
    return (
      <div className="custom-node">
        <PortWidget
          engine={ this.props.engine }
          port={ this.props.node.getPort("in") as any }
        >
          <div className="circle-port"/>
        </PortWidget>
        <PortWidget
          engine={ this.props.engine }
          port={ this.props.node.getPort("out") as any }
        >
          <div className="circle-port"/>
        </PortWidget>
        <div
          className="custom-node-color"
          style={ { backgroundColor: this.props.node.color } }
        />
      </div>
    );
  }
}
