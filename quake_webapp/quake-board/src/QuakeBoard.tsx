import React from 'react';
import createEngine, {DagreEngine, DefaultNodeModel, DiagramEngine, DiagramModel} from '@projectstorm/react-diagrams';
import {Item, Menu, useContextMenu} from "react-contexify";
import {CanvasWidget} from "@projectstorm/react-canvas-core";
import styled from "styled-components";
import 'react-contexify/dist/ReactContexify.css';
import {DiamondNodeFactory} from "./components/base-model/DiamondNodeFactory";
import {DiamondNodeModel} from "./components/base-model/DiamondNodeModel";

export type Props = {
  model: any,
  onChange: (model: any) => any
}

const MENU_ID = 'blahblah';

function QuakeBoard(props: Props) {
  const dagreEngine = React.useMemo(() => new DagreEngine({
    graph: {
      rankdir: 'LR',
      ranker: 'network-simplex',
      marginx: 0,
      marginy: 0,
      nodesep: 35,
      edgesep: 10,
      ranksep: 75,
    },
    includeLinks: false,
  }), []);

  const [clickPosition, setClickPosition] = React.useState({clientX: 0, clientY: 0})

  const engine = React.useMemo(() => {
    let engine = createEngine();
    engine.getNodeFactories().registerFactory(new DiamondNodeFactory() as any);

    return engine
  }, []);

  const model = React.useMemo(() => new DiagramModel(), []);

  const data = React.useMemo(() => {
    return {
      nodes: [],
      links: []
    };
  }, []);


  React.useLayoutEffect(() => {
    model.addAll(...data.nodes.concat(data.links));
    dagreEngine.redistribute(model);
    engine.repaintCanvas();
  }, [data, model, engine, dagreEngine]);

  engine.setModel(model);

  const {show} = useContextMenu({
    id: MENU_ID,
  });

  const handleContextMenu = React.useCallback(
    (event: any) => {
      event.preventDefault();
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
      console.log(JSON.stringify(serialize));
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

  return (
    <StyledDiv onContextMenu={handleContextMenu}>
      <StyledCanvasWidget engine={engine}/>
      <Menu id={MENU_ID}>
        <Item onClick={addSource}>add Source</Item>
        <Item onClick={addFilter}>add Filter rule</Item>
        <Item onClick={addLambda}>add Lambda</Item>
        <Item onClick={addTarget}>add Target</Item>
      </Menu>
    </StyledDiv>
  );
}

export default QuakeBoard;

const StyledDiv = styled.div`
  width: 100%;
  height: 100%;
`

const StyledCanvasWidget = styled(CanvasWidget)`
  width: 100%;
  height: 100%;

  background-size: 50px 50px;
  background-color: rgb(60, 60, 60);
  background-image: linear-gradient(0deg, transparent 24%, rgba(255, 255, 255, 0.05) 25%, rgba(255, 255, 255, 0.05) 26%, transparent 27%, transparent 74%, rgba(255, 255, 255, 0.05) 75%, rgba(255, 255, 255, 0.05) 76%, transparent 77%, transparent), linear-gradient(90deg, transparent 24%, rgba(255, 255, 255, 0.05) 25%, rgba(255, 255, 255, 0.05) 26%, transparent 27%, transparent 74%, rgba(255, 255, 255, 0.05) 75%, rgba(255, 255, 255, 0.05) 76%, transparent 77%, transparent);

`
