import React from 'react';
import createEngine, {DagreEngine, DefaultNodeModel, DiagramModel} from '@projectstorm/react-diagrams';
import {BodyWidget} from "./BodyWidget";

export type Props = {}

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

  React.useLayoutEffect(() => {
    // dagreEngine.redistribute(model);
  });

  const data = React.useMemo(() => {
    let nodes = [];
    let links: any = [];

    const node1 = new DefaultNodeModel('Node 1', 'rgb(0,192,255)');
    node1.setPosition(100, 100);
    const port1 = node1.addOutPort('Out');
    nodes.push(node1);

    const node2 = new DefaultNodeModel('Node 2', 'rgb(192,255,0)');
    node2.setPosition(400, 100);
    const port2 = node2.addInPort('In');
    nodes.push(node2);

    const link1 = port1.link(port2);

    links.push(link1);

    return {
      nodes: nodes,
      links: links
    };
  }, []);

  model.addAll(...data.nodes.concat(data.links));

  // console.log(model);
  engine.setModel(model);

  return (
    <>
      <BodyWidget engine={engine}/>
    </>
  );
}

export default QuakeBoard;
