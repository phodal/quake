import * as React from "react";
import { DiagramEngine, PortWidget } from "@projectstorm/react-diagrams-core";
import { ConceptNodeModel } from "./ConceptNodeModel";
import styled from "styled-components";

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
    this.state = {
      name: "ConceptNodeWidget"
    };
  }

  render() {
    return (
      <StyledConceptNodeWidget>
        <PortWidget
          engine={ this.props.engine }
          port={ this.props.node.getPort("in") as any }
        >
          <StyledCirclePort />
        </PortWidget>

        <StyledNodeColor
          style={ { backgroundColor: this.props.node.color } }
        >
          <p>ConceptNodeWidget</p>
        </StyledNodeColor>

        <PortWidget
          engine={ this.props.engine }
          port={ this.props.node.getPort("out") as any }
        >
          <StyledCirclePort />
        </PortWidget>
      </StyledConceptNodeWidget>
    );
  }
}

const StyledConceptNodeWidget = styled.div`
  border: solid 2px gray;
  border-radius: 5px;
  width: 100%;
  min-height: 50px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  position: relative;
`

const StyledCirclePort = styled.div`
  width: 12px;
  height: 12px;
  margin: 2px;
  border-radius: 4px;
  background: darkgray;
  cursor: pointer;
`

const StyledNodeColor = styled.div`
  top: 15px;
  position: relative;
  width: 100%;
  height: 20px;
  justify-content: center;
  border-radius: 10px;
`
