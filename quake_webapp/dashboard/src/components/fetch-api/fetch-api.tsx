import {Component, Host, h, Prop, EventEmitter, Event} from '@stencil/core';

@Component({
  tag: 'fetch-api',
  shadow: true,
})
export class FetchApi {
  @Prop() entryType: String[] = [];
  @Prop() type: String = '';

  // todo: may be not need
  @Prop() data: any = {};

  @Prop() searchEngine: boolean = false;

  @Event({
    eventName: 'fetchNextSuccess',
    composed: true,
    cancelable: true,
    bubbles: true,
  }) fetchNextSuccess: EventEmitter;

  @Event({
    eventName: 'fetchAllSuccess',
    composed: true,
    cancelable: true,
    bubbles: true,
  }) fetchAllSuccess: EventEmitter;

  componentWillLoad() {

  }

  render() {
    return (
      <Host>
        <slot></slot>
      </Host>
    );
  }

}
