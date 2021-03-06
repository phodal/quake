import * as React from "react";
import { DiagramEngine, PortWidget } from "@projectstorm/react-diagrams-core";
import { ConceptNodeModel } from "./ConceptNodeModel";
import styled from "styled-components";
import Editor from "./Editor";

export interface ConceptNodeWidgetProps {
  node: ConceptNodeModel;
  engine: DiagramEngine;
}

export interface ConceptNodeWidgetState {
  value: string,
  toggle: boolean,
  width: number,
  height: number
}

export class ConceptNodeWidget extends React.Component<ConceptNodeWidgetProps,
  ConceptNodeWidgetState> {
  constructor(props: ConceptNodeWidgetProps) {
    super(props);
    this.state = {
      toggle: false,
      value: "text",
      width: 100,
      height: 100,
    };
  }

  render() {
    return (
      <StyledConceptNodeWidget style={ {
        backgroundColor: this.props.node.color
      } }>
        <StyledPorts>
          <PortWidget
            engine={ this.props.engine }
            port={ this.props.node.getPort("in") as any }
          >
            <StyledCirclePort/>
          </PortWidget>

          <PortWidget
            engine={ this.props.engine }
            port={ this.props.node.getPort("out") as any }
          >
            <StyledCirclePort/>
          </PortWidget>
        </StyledPorts>

        <StyledNodeColor>
          <Editor  onChange={this.onChange}/>
        </StyledNodeColor>
      </StyledConceptNodeWidget>
    );
  }

  private onChange() {
    this.setState({
      value: "text"
    });
  }
}

const StyledPorts = styled.div`
  display: flex;
  justify-content: space-between;
`

const StyledConceptNodeWidget = styled.div`
  border: solid 2px gray;
  border-radius: 5px;
  width: 100%;
  display: flex;
  flex-direction: column;
  position: relative;
`

const StyledCirclePort = styled.div`
  width: 12px;
  height: 12px;
  margin: 2px;
  border-radius: 4px;
  background: greenyellow;
  cursor: pointer;
`

const StyledNodeColor = styled.div`
  width: 100%;
  justify-content: center;
  padding: 2px;
`
