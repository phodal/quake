import {Component, Prop, h, Element, State} from '@stencil/core';
import {MeiliSearch} from "meilisearch";

@Component({
  tag: 'quake-dashboard',
  styleUrl: 'quake-dashboard.css',
  shadow: true,
})
export class QuakeDashboard {
  ionInputElement;

  @Prop() indexName: String = "";
  @Element() el: HTMLElement;
  @State() items: Array<object> = [];

  client = new MeiliSearch({
    host: 'http://127.0.0.1:7700'
  })

  handleInput(event) {
    const that = this;
    const query = event.target.value;
    const index = this.client.index('phodal_com')
    requestAnimationFrame(() => {
      index.search(query).then((result) => {
        that.items = result.hits;
      })
    });
  }

  render() {
    console.log(this.items);
    return <ion-app>
      <ion-header translucent>
        <ion-toolbar>
          <ion-title>Quake Action</ion-title>
        </ion-toolbar>
        <ion-toolbar>
          <ion-searchbar ref={(el) => this.ionInputElement = el} onInput={this.handleInput.bind(this)}></ion-searchbar>
        </ion-toolbar>
      </ion-header>
      <ion-content fullscreen>
        <ion-list>
          {this.items.length > 0
            ? this.items.map((item: any) => <ion-item>{item.title}</ion-item>)
            : <div></div>
          }
        </ion-list>
      </ion-content>
    </ion-app>;
  }
}
