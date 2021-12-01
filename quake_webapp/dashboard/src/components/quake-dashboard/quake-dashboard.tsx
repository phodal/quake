import {Component, Element, Event, EventEmitter, h, Prop, State} from '@stencil/core';
import {MeiliSearch} from "meilisearch";
import dayjs from "dayjs";
import axios from "axios";

// only for: IDEA jump
// @ts-ignore
import {IonSearchbar} from "@ionic/core";

export interface ActionDefine {
  object: String,
  action: String,
  text: String,
  parameters: String[]
}

export interface EntryInfo {
  type: string,
  display: string,
  fields: any[],
  action: any[]
}

export interface SearchResult {
  [id: string]: Array<any>
}

@Component({
  tag: 'quake-dashboard',
  styleUrl: 'quake-dashboard.css',
  shadow: true,
})
export class QuakeDashboard {
  ionInputElement;

  @Prop() indexName: string = "";
  @Element() el: HTMLElement;
  @State() items: Array<object> = [];
  @State() list: SearchResult = {};

  @State() isAction = false;

  @State() query: string = "";
  @State() inputType: string = "";
  @State() actionDefine: ActionDefine = null;
  @State() entries_info: EntryInfo[] = [];

  @Event({
    eventName: 'dispatchAction',
    composed: true,
    cancelable: true,
    bubbles: true,
  }) dispatchAction: EventEmitter<ActionDefine>;

  client = new MeiliSearch({
    host: 'http://127.0.0.1:7700'
  })

  componentWillLoad() {
    const that = this;
    axios.get('/action/suggest')
      .then((response: any) => {
        that.entries_info = response.data.entries;
        console.log(that.entries_info);
      });
  }

  handleInput(event) {
    const that = this;
    this.query = event.target.value;
    if (this.query.length == 0) {
      this.items = [];
      this.inputType = '';
      this.list = {};
      return;
    }

    if (this.query.startsWith("/")) {
      this.inputType = 'Action'
      return;
    }

    this.actionDefine = null;
    this.inputType = 'Search';
    this.createSearch(that);
  }

  private createSearch(that: this) {
    for (let info of this.entries_info) {
      this.search_item(that, info.type);
    }
  }

  private search_item(that: this, doc: string) {
    const index = this.client.index(doc)
    requestAnimationFrame(() => {
      index.search(that.query).then((result) => {
        that.list[doc] = result.hits;
        that.items = result.hits;
      })
    });
  }

  formatDate(str) {
    return dayjs(str).format('YYYY-MM-DD');
  }

  padLeft(nr, n, str) {
    return Array(n - String(nr).length + 1).join(str || '0') + nr;
  }

  async handleSubmit(e) {
    e.preventDefault()

    if (this.query.startsWith("/")) {
      this.inputType = 'Action'
    } else {
      return;
    }

    const that = this;
    requestAnimationFrame(() => {
      axios.get('/action/query/', {
        params: {
          input: this.query.substring(1,)
        }
      }).then(response => {
        if (response.data.object) {
          that.actionDefine = response.data
          that.dispatchAction.emit(response.data);
        } else {
          that.actionDefine = null;
          that.presentToast(response.data.msg);
        }
      }).catch(function (error) {
        console.log(error);
      });
    });
  }

  clickEntry(id: string, object: string) {
    this.dispatchAction.emit({
      parameters: [id],
      action: 'update',
      object: object,
    } as any);
  }

  async presentToast(msg: string) {
    const toast = document.createElement('ion-toast');
    toast.message = msg;
    toast.duration = 2000;

    document.body.appendChild(toast);
    return toast.present();
  }

  render() {
    return <ion-app>
      <ion-header>
        <ion-toolbar>
          <ion-item>
            {this.inputType.length > 0
              ? <ion-chip>
                <ion-label>{this.inputType}</ion-label>
              </ion-chip>
              : null
            }
            <form id="search-form" onSubmit={this.handleSubmit.bind(this)}>
              <ion-input
                placeholder="`/todo.add: hello, world` for create `todo`"
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
      <ion-grid>
        <ion-row>
          {this.entries_info.map((info) =>
            this.list[info.type] && this.list[info.type].length > 0 ? this.renderCol(info) : null
          )}
        </ion-row>
      </ion-grid>
    </ion-app>;
  }

  private renderCol(info: EntryInfo) {
    return <ion-col>
      <ion-text color="secondary">{info.type}</ion-text>
      {this.list[info.type] ? this.list[info.type].map((item: any) =>
        <ion-item onClick={() => this.clickEntry(item.id, info.type)}>
          <ion-badge slot="start"># {this.padLeft(item.id, 4, '')}</ion-badge>
          <ion-badge slot="start">{this.formatDate(item.created_date)}</ion-badge>
          {item.title}
        </ion-item>
      ) : null
      }
    </ion-col>;
  }
}
