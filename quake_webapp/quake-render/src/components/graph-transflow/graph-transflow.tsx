import { Component, Host, h } from '@stencil/core';

@Component({
  tag: 'graph-transflow',
  styleUrl: 'graph-transflow.css',
  shadow: true,
})
export class GraphTransflow {

  render() {
    return (
      <Host>
        <slot></slot>
      </Host>
    );
  }

}
