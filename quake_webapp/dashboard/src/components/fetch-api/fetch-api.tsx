import {Component, Event, EventEmitter, h, Host, Prop, State} from '@stencil/core';
import {MeiliSearch} from "meilisearch";
import axios from "axios";

@Component({
  tag: 'fetch-api',
  shadow: true,
})
export class FetchApi {
  @Prop() entryType: string[] = [];
  @Prop() type: string = '';
  @State() url: string = '';

  @Prop() searchEngine: boolean = false;

  client = new MeiliSearch({
    host: 'http://127.0.0.1:7700'
  })

  @Event({
    eventName: 'fetchSuccess',
    composed: true,
    cancelable: true,
    bubbles: true,
  }) fetchSuccess: EventEmitter;

  @Event({
    eventName: 'fetchAllSuccess',
    composed: true,
    cancelable: true,
    bubbles: true,
  }) fetchAllSuccess: EventEmitter;

  getRequest(url: string): Promise<any> {
    return axios.get(url).then((data) => this.fetchSuccess.emit(data));
  }

  postRequest(url: string, data: any): Promise<any> {
    return axios.post(url, data).then((data) => this.fetchSuccess.emit(data));
  }

  componentWillLoad() {
    switch (this.type) {
      case 'suggest': {
        this.url = "/action/suggest";
        break;
      }
      case 'layout': {
        this.url = "/layout/dashboard";
        break;
      }
      case 'actionQuery' : {
        // todo: use wasm
        break;
      }
      default:
        console.log(this.type);
    }
    this.getRequest(this.url).then(() => {
    });
  }

  render() {
    return (
      <Host></Host>
    );
  }

}
