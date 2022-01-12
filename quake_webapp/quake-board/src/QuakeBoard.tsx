import React from 'react';
import createEngine, {DefaultNodeModel, DiagramModel} from '@projectstorm/react-diagrams';
import {BodyWidget} from "./BodyWidget";

export type Props = {
}

function QuakeBoard(props: Props) {
  function getDefaultEngine() {
    const engine = createEngine();
    const model = new DiagramModel();
    engine.setModel(model);
    return engine;
  }

  const [engine, setEngine] = React.useState(getDefaultEngine());

  const buildEngine = () => {
    //1) setup the diagram engine
    const engine = createEngine();

    //2) setup the diagram model
    const model = new DiagramModel();

    //3-A) create a default node
    const node1 = new DefaultNodeModel('Node 1', 'rgb(0,192,255)');
    const port1 = node1.addOutPort('Out');
    node1.setPosition(100, 100);

    //3-B) create another default node
    const node2 = new DefaultNodeModel('Node 2', 'rgb(192,255,0)');
    const port2 = node2.addInPort('In');
    node2.setPosition(400, 100);

    //3-C) link the 2 nodes together
    const link1 = port1.link(port2);

    //4) add the models to the root graph
    model.addAll(node1, node2, link1);

    //5) load model into engine
    engine.setModel(model);
    setEngine(engine);

  }

  React.useEffect(() => {
    buildEngine();
  },[]);

  return (
    <BodyWidget engine={engine}/>
  );
}

export default QuakeBoard;
