import {Component, Host, h, State} from '@stencil/core';

@Component({
  tag: 'flow-view',
  styleUrl: 'flow-view.css',
  shadow: true,
})
export class FlowView {
  @State() items: any[] = [];

  render() {
    return (
      <Host>
        <slot>
          <h1>dsfa</h1>
          <h1>dsfa</h1>
          <h1>dsfa</h1>
        </slot>
      </Host>
    );
  }

}
