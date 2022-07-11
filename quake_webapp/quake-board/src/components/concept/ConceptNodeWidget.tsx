import * as React from "react";
import { DiagramEngine, PortWidget } from "@projectstorm/react-diagrams-core";
import { ConceptNodeModel } from "./ConceptNodeModel";
import styled from "styled-components";

export interface ConceptNodeWidgetProps {
  node: ConceptNodeModel;
  engine: DiagramEngine;
}

export interface ConceptNodeWidgetState {
  name: string,
  toggle: boolean
}

export class ConceptNodeWidget extends React.Component<ConceptNodeWidgetProps,
  ConceptNodeWidgetState> {
  constructor(props: ConceptNodeWidgetProps) {
    super(props);
    this.state = {
      toggle: false,
      name: ""
    };
  }

  setName(name: string) {
    // Changing state
    this.setState({ name })
  }

  setToggle(toggle: boolean) {
    this.setState({ toggle })
  }

  render() {
    return (
      <StyledConceptNodeWidget  style={ { backgroundColor: this.props.node.color } }>
        <PortWidget
          engine={ this.props.engine }
          port={ this.props.node.getPort("in") as any }
        >
          <StyledCirclePort/>
        </PortWidget>

        <StyledNodeColor>
          <StyleInputBox type="text" value={ this.state.name }
                 onChange={ (event) => {
                   this.setName(event.target.value)
                 } }
                 onKeyDown={ (event) => {
                   if (event.key === 'Enter' || event.key === 'Escape') {
                     this.setToggle(true)
                     event.preventDefault()
                     event.stopPropagation()
                   }
                 } }
          />
        </StyledNodeColor>

        <PortWidget
          engine={ this.props.engine }
          port={ this.props.node.getPort("out") as any }
        >
          <StyledCirclePort/>
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
  background: greenyellow;
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

const StyleInputBox = styled.input`
  background: transparent;
  border: none;


  :hover {
    border: none;
  }
`
