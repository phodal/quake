import { Component, Host, h } from '@stencil/core';

@Component({
  tag: 'quake-graph',
  styleUrl: 'quake-graph.css',
  shadow: true,
})
export class QuakeGraph {

  render() {
    return (
      <Host>
        <slot></slot>
      </Host>
    );
  }

}
