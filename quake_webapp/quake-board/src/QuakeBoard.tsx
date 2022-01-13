import React from 'react';
import createEngine, {DagreEngine, DefaultNodeModel, DiagramEngine, DiagramModel} from '@projectstorm/react-diagrams';
import {Item, Menu, useContextMenu} from "react-contexify";
import {CanvasWidget} from "@projectstorm/react-canvas-core";
import styled from "styled-components";
import 'react-contexify/dist/ReactContexify.css';

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

  const engine = React.useMemo(() => createEngine(), []);
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

  function handleContextMenu(event: any) {
    event.preventDefault();
    show(event, {
      props: {
        key: 'value'
      }
    })
  }

  function addNode(event: MouseEvent, engine: DiagramEngine, type: string) {
    let node: DefaultNodeModel;
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
      default:
        node = new DefaultNodeModel('default', 'rgb(0,192,192)');
        node.addInPort('In');
        node.addOutPort('Out');
        node.getOptions().extras = {"type": "inout"};
        break;
    }

    const point = engine.getRelativeMousePoint(event);
    node.setPosition(point);

    return node;
  }

  const addSource = React.useCallback(
    ({event}) => {
      let node = addNode(event, engine, 'out');
      model.addNode(node);

      let serialize = model.serialize();
      props.onChange(serialize);

      engine.repaintCanvas();
    }, [engine, model, props]
  );

  const addFilter = React.useCallback(
    (props: any) => {
      console.log(props)
    }, []
  );

  const addLambda = React.useCallback(
    ({event}) => {
      let node = addNode(event, engine, 'default');
      model.addNode(node);

      let serialize = model.serialize();
      props.onChange(serialize);

      engine.repaintCanvas();
    }, [engine, model, props]
  );

  const addTarget = React.useCallback(
    ({event}) => {
      let node = addNode(event, engine, 'in');
      model.addNode(node);

      let serialize = model.serialize();
      props.onChange(serialize);

      engine.repaintCanvas();
    }, [engine, model, props]
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
