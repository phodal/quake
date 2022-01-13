import {NodeModel, NodeModelGenerics, PortModelAlignment} from '@projectstorm/react-diagrams';
import {DiamondPortModel} from './DiamondPortModel';

export interface DiamondNodeModelGenerics extends NodeModelGenerics {
  PORT: DiamondPortModel;
}

export class DiamondNodeModel extends NodeModel<DiamondNodeModelGenerics> {
  constructor() {
    super({
      type: 'diamond'
    });
    this.addPort(new DiamondPortModel(PortModelAlignment.TOP));
    this.addPort(new DiamondPortModel(PortModelAlignment.LEFT));
    this.addPort(new DiamondPortModel(PortModelAlignment.BOTTOM));
    this.addPort(new DiamondPortModel(PortModelAlignment.RIGHT));
  }
}
