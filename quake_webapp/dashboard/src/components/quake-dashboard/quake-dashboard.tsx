import {Component, Element, h, Prop, State} from '@stencil/core';
import {MeiliSearch} from "meilisearch";
import dayjs from "dayjs";

// only for: IDEA jump
// @ts-ignore
import {IonSearchbar} from "@ionic/core";
import axios from "axios";


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
  @State() isAction = false;

  @State() query: String = "";
  @State() actionType: String = "";

  client = new MeiliSearch({
    host: 'http://127.0.0.1:7700'
  })

  handleInput(event) {
    console.log(event);
    const that = this;
    this.query = event.target.value;
    if (this.query.length == 0) {
      that.items = [];
      this.actionType = '';
      return;
    }

    if (this.query.startsWith(".")) {
      this.actionType = 'Action'
      return;
    }

    this.actionType = 'Search';
    this.createSearch(that);
  }

  private createSearch(that: this) {
    let query = that.query.substr(0, 1);
    const index = this.client.index('phodal_com')
    requestAnimationFrame(() => {
      index.search(query).then((result) => {
        that.items = result.hits;
      })
    });
  }

  formatDate(str) {
    return dayjs(str).format('YYYY-MM-DD HH:mm:ss');
  }

  clearInput(_event) {
    this.items = [];
  }

  private async handleSubmit(e) {
    e.preventDefault()

    axios.get('http://127.0.0.1:8000/action/query/', {
      params: {
        input: this.query.substring(1,)
      }
    }).then(function (response) {
      console.log(response.data);
    }).catch(function (error) {
      console.log(error);
    });
  }

  render() {
    return <ion-app>
      <ion-header translucent>
        <ion-toolbar>
          <ion-title>Quake</ion-title>
        </ion-toolbar>
        <ion-toolbar>
          <ion-item>
            {this.actionType.length > 0 ? <ion-chip>
              <ion-label>{this.actionType}</ion-label>
            </ion-chip> : null}
            <form onSubmit={(e) => this.handleSubmit(e)}>
              <ion-input
                placeholder="`.todo:add` for create `todo`"
                autofocus={true}
                ref={(el) => this.ionInputElement = el}
                onIonInput={this.handleInput.bind(this)}
              />
              <input type="submit" value="Submit" id="submit-button"/>
            </form>
            {/*{ this.actionType.length > 0 ? <ion-chip><ion-label>{this.actionType}</ion-label></ion-chip> : null }*/}
          </ion-item>
        </ion-toolbar>
      </ion-header>
      <ion-content fullscreen>
        <ion-list>
          {this.items.length > 0
            ? this.items.map((item: any) =>
              <ion-item>
                <ion-badge slot="start">{this.formatDate(item.created_date)}</ion-badge>
                {item.title}
              </ion-item>)
            : <div></div>
          }
        </ion-list>
      </ion-content>
    </ion-app>;
  }
}
