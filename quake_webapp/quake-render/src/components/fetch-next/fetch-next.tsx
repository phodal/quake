import {Component, Host, h, Prop, EventEmitter, Event} from '@stencil/core';

@Component({
  tag: 'fetch-next',
  shadow: true,
})
export class FetchNext {
  @Prop() url: String = '';
  @Prop() method: String = 'get';

  // todo: may be not need
  @Prop() data: any = {};

  @Prop() searchEngine: boolean = false;

  @Event({
    eventName: 'fetchSuccess',
    composed: true,
    cancelable: true,
    bubbles: true,
  }) fetchSuccess: EventEmitter;

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
