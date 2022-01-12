import * as React from 'react';
import { DiagramEngine } from '@projectstorm/react-diagrams';
import { CanvasWidget } from '@projectstorm/react-canvas-core';
import styled from "styled-components";

export interface BodyWidgetProps {
  engine: DiagramEngine;
}

export class BodyWidget extends React.Component<BodyWidgetProps> {
  render() {
    return <StyledCanvasWidget engine={this.props.engine} />;
  }
}

const StyledCanvasWidget = styled(CanvasWidget)`
  background: #333333;
  width: 100%;
  height: 100%;
`
