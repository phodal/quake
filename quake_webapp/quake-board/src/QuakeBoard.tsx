import React from 'react';
import createEngine, {DagreEngine, DefaultNodeModel, DiagramModel} from '@projectstorm/react-diagrams';
import {Item, Menu, Separator, Submenu, useContextMenu} from "react-contexify";
import {CanvasWidget} from "@projectstorm/react-canvas-core";
import styled from "styled-components";
import 'react-contexify/dist/ReactContexify.css';

export type Props = {}

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
    let nodes = [];
    let links: any = [];

    const node1 = new DefaultNodeModel('Node 1', 'rgb(0,192,255)');
    const port1 = node1.addOutPort('Out');
    nodes.push(node1);

    const node2 = new DefaultNodeModel('Node 2', 'rgb(192,255,0)');
    const port2 = node2.addInPort('In');
    nodes.push(node2);

    const link1 = port1.link(port2);

    links.push(link1);

    return {
      nodes: nodes,
      links: links
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

  const handleItemClick = React.useCallback(
    (props: any) => console.log(props), []
  );

  return (
    <StyledDiv onDoubleClick={handleContextMenu}>
      <StyledCanvasWidget engine={engine} />
      <Menu id={MENU_ID}>
        <Item onClick={handleItemClick}>Item 1</Item>
        <Item onClick={handleItemClick}>Item 2</Item>
        <Separator/>
        <Item disabled>Disabled</Item>
        <Separator/>
        <Submenu label="Foobar">
          <Item onClick={handleItemClick}>Sub Item 1</Item>
          <Item onClick={handleItemClick}>Sub Item 2</Item>
        </Submenu>
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
  background: #333333;
  width: 100%;
  height: 100%;
`
