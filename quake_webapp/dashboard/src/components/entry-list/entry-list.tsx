import { Component, Host, h } from '@stencil/core';

@Component({
  tag: 'entry-list',
  styleUrl: 'entry-list.css',
  shadow: true,
})
export class EntryList {

  render() {
    return (
      <Host>
        <slot></slot>
      </Host>
    );
  }

}
