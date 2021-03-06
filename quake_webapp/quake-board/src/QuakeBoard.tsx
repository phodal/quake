import React from 'react';
import createEngine, {
  DefaultNodeModel,
  DiagramEngine,
  DiagramModel,
  PortModelAlignment
} from '@projectstorm/react-diagrams';
import {Item, Menu, useContextMenu} from "react-contexify";
import {CanvasWidget} from "@projectstorm/react-canvas-core";
import styled from "styled-components";
import {DiamondNodeFactory} from "./components/base-model/DiamondNodeFactory";
import {DiamondNodeModel} from "./components/base-model/DiamondNodeModel";

import 'react-contexify/dist/ReactContexify.css';

import {SimplePortFactory} from "./components/SimplePortFactory";
import {DiamondPortModel} from "./components/base-model/DiamondPortModel";
import { ConceptNodeFactory } from "./components/concept/ConceptNodeFactory";
import { ConceptNodeModel } from "./components/concept/ConceptNodeModel";

export type Props = {
  model: any,
  onChange: (model: any) => any
}

const MENU_ID = 'quake-board';

function QuakeBoard(props: Props) {
  const [clickPosition, setClickPosition] = React.useState({clientX: 0, clientY: 0})
  const engine = React.useMemo(() => {
    let engine = createEngine();

    // if we don't config port factories will cause issue when deserialize
    engine
      .getPortFactories()
      .registerFactory(new SimplePortFactory('diamond', (_config) => new DiamondPortModel(PortModelAlignment.LEFT)));
    engine.getNodeFactories().registerFactory(new DiamondNodeFactory() as any);
    engine.getNodeFactories().registerFactory(new ConceptNodeFactory() as any);

    return engine
  }, []);

  const model = React.useMemo(() => {
    let defaultModel = new DiagramModel();
    if (!!props.model) {
      defaultModel.deserializeModel(props.model, engine);
    }

    defaultModel.registerListener({
      linksUpdated: (e: any) => {
        props.onChange(model.serialize());
      },
    });

    return defaultModel
  }, [props, engine]);
  engine.setModel(model);

  const {show} = useContextMenu({
    id: MENU_ID,
  });

  const handleContextMenu = React.useCallback(
    (event: any) => {
      setClickPosition({
        clientX: event.clientX,
        clientY: event.clientY
      });
      show(event, {
        props: {
          key: 'value'
        }
      })
    }, [setClickPosition, show]);

  function createNode(event: MouseEvent, engine: DiagramEngine, type: string, clickPosition: any) {
    let node: any;
    switch (type) {
      case 'out':
        node = new DefaultNodeModel('Source', 'rgb(0,192,255)');
        node.addOutPort('Out');
        node.getOptions().extras = {"type": "out"};
        break;
      case 'in':
        node = new DefaultNodeModel('Target', 'rgb(192,255,0)');
        node.addInPort('In');
        node.getOptions().extras = {"type": "in"};
        break;
      case 'filter':
        node = new DiamondNodeModel();
        node.getOptions().extras = {"type": "inout"};
        break;
      case 'concept':
        node = new ConceptNodeModel();
        node.getOptions().extras = {"type": "inout"};
        break;
      default:
        node = new DefaultNodeModel('default', 'rgb(0,192,192)');
        node.addInPort('In');
        node.addOutPort('Out');
        node.getOptions().extras = {"type": "inout"};
        break;
    }

    const point = engine.getRelativeMousePoint(clickPosition);
    node.setPosition(point);

    return node;
  }

  const sendChange = React.useCallback(
    () => {
      let serialize = model.serialize();
      props.onChange(serialize);
    }, [props, model]);

  const addSource = React.useCallback(
    ({event}) => {
      let node = createNode(event, engine, 'out', clickPosition);
      model.addNode(node);
      sendChange();
      engine.repaintCanvas();
    }, [engine, model, sendChange, clickPosition]
  );

  const addConcept = React.useCallback(
    ({event}) => {
      let node = createNode(event, engine, 'concept', clickPosition);
      model.addNode(node);
      sendChange();
      engine.repaintCanvas();
    }, [engine, model, sendChange, clickPosition]
  );

  const addFilter = React.useCallback(
    ({event}) => {
      let node = createNode(event, engine, 'filter', clickPosition);
      model.addNode(node);
      sendChange();
      engine.repaintCanvas();
    }, [engine, model, sendChange, clickPosition]
  );

  const addLambda = React.useCallback(
    ({event}) => {
      let node = createNode(event, engine, 'default', clickPosition);
      model.addNode(node);
      sendChange();
      engine.repaintCanvas();
    }, [engine, model, sendChange, clickPosition]
  );

  const addTarget = React.useCallback(
    ({event}) => {
      let node = createNode(event, engine, 'in', clickPosition);
      model.addNode(node);
      sendChange();
      engine.repaintCanvas();
    }, [engine, model, sendChange, clickPosition]
  );

  const onDrag = React.useCallback(({event}) => {
    console.log(event);
  }, []);

  return (
    <StyledDiv onContextMenu={handleContextMenu} onDrag={onDrag}>
      <StyledCanvasWidget engine={engine}/>
      <StyledMenu id={MENU_ID}>
        <Item onClick={addConcept}>add Concept</Item>
        <Item onClick={addSource}>add Source</Item>
        <Item onClick={addFilter}>add Filter rule</Item>
        <Item onClick={addLambda}>add Lambda</Item>
        <Item onClick={addTarget}>add Target</Item>
      </StyledMenu>
    </StyledDiv>
  );
}

export default QuakeBoard;

const StyledDiv = styled.div`
  width: 100%;
  height: 100%;
`

const StyledMenu = styled(Menu)`
  position: absolute;
`

// @ts-ignore
const StyledCanvasWidget = styled(CanvasWidget)`
  width: 100%;
  height: 100%;

  background-size: 50px 50px;
  background-color: rgb(60, 60, 60);
  background-image: linear-gradient(0deg, transparent 24%, rgba(255, 255, 255, 0.05) 25%, rgba(255, 255, 255, 0.05) 26%, transparent 27%, transparent 74%, rgba(255, 255, 255, 0.05) 75%, rgba(255, 255, 255, 0.05) 76%, transparent 77%, transparent), linear-gradient(90deg, transparent 24%, rgba(255, 255, 255, 0.05) 25%, rgba(255, 255, 255, 0.05) 26%, transparent 27%, transparent 74%, rgba(255, 255, 255, 0.05) 75%, rgba(255, 255, 255, 0.05) 76%, transparent 77%, transparent);

`
