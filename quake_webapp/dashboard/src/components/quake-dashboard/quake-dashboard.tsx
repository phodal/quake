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

export interface FlowDefine {
  field: string,
  items: string[]
}

export interface EntryInfo {
  type: string,
  display: string,
  fields: any[],
  action: any[],
  flows: FlowDefine[]
}

export interface SearchResult {
  [id: string]: Array<any>
}

enum InputType {
  Empty,
  Search = 'Search',
  Action = 'Action'
}

@Component({
  tag: 'quake-dashboard',
  styleUrl: 'quake-dashboard.css',
  shadow: true,
})
export class QuakeDashboard {
  ionInputElement;
  infiniteScroll: HTMLIonInfiniteScrollElement;

  @Element() el: HTMLElement;

  @Prop() indexName: string = "";

  @State() items: Array<object> = [];
  @State() list: SearchResult = {};

  @State() isAction = false;

  @State() query: string = "";
  @State() inputType: InputType = InputType.Empty;
  @State() actionDefine: ActionDefine = null;
  @State() entries_info: EntryInfo[] = [];
  @State() actions_list: string[] = [];

  @State() selected_entry: EntryInfo = null;
  @State() selected_flow_result: Map<string, object[]> = new Map();
  @State() selected_result: any[] = [];

  @State() is_flow: boolean = false;
  @State() offset: object = {};

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
        that.actions_list = response.data.actions;
      });
  }

  handleInput(event) {
    const that = this;
    this.query = event.target.value;
    this.handleQuery(that);
  }

  private handleQuery(that: this) {
    if (this.query.length == 0) {
      this.items = [];
      this.inputType = InputType.Empty;
      this.list = {};
      return;
    }

    if (this.query.startsWith("/")) {
      this.inputType = InputType.Action
      return;
    }

    this.actionDefine = null;
    this.inputType = InputType.Search;
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
      index.search(that.query, {
        attributesToHighlight: ['overview']
      }).then((result) => {
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

  async selectType(_e: Event, info: EntryInfo) {
    this.reset_input();
    this.selected_entry = info;
    this.query = '/' + info.type + '.';
    this.handleQuery(this);
  }

  async addAction(_e: Event, action: string) {
    this.reset_input();
    if (!this.selected_entry) {
      return;
    }
    if (action == 'show') {
      let type = this.selected_entry.type;
      this.offset[type] = 0;
      this.queryItems(this.offset[type]).then((result) => {
        this.offset[type] = this.offset[type] + 40;

        let parsed = result.hits;
        this.is_flow = !!this.selected_entry.flows
        if (this.is_flow) {
          this.process_flow(parsed);
        } else {
          this.selected_result = parsed;
        }
      })
    }

    if (this.query.startsWith("/") && this.query.endsWith(".")) {
      this.query = this.query + action;
      this.handleQuery(this);
    }
  }

  private queryItems(offset: number) {
    const index = this.client.index(this.selected_entry.type)
    return index.search('', {
      attributesToHighlight: ['overview'],
      limit: 40,
      offset
    })
  }

  private process_flow(parsed) {
    this.is_flow = true;
    let results_map = new Map();
    let flow = this.selected_entry.flows[0];
    for (let item of flow.items) {
      results_map.set(item, []);
    }

    for (let el of parsed) {
      results_map.get(el.status)?.push(el);
    }

    this.selected_flow_result = results_map;
  }

  private reset_input() {
    this.selected_result = [];
    this.selected_flow_result = new Map();
  }

  async handleSubmit(e) {
    e.preventDefault()

    if (this.query.startsWith("/")) {
      this.inputType = InputType.Action
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

  loadData(_ev) {
    if (this.infiniteScroll && this.infiniteScroll.disabled) {
      return "no more data";
    }

    console.log('Loaded data');
    let type = this.selected_entry.type;
    this.queryItems(this.offset[type]).then((result) => {
      this.offset[type] = this.offset[type] + 40;
      this.infiniteScroll.complete().then(_r => {});

      let parsed = result.hits;
      if (parsed.length == 0) {
        this.infiniteScroll.disabled = true;
        return;
      }

      this.selected_result = this.selected_result.concat(parsed);
      console.log(this.selected_result.length);
    })

  }

  render() {
    return <ion-app>
      <ion-header translucent>
        <ion-toolbar>
          <ion-item>
            {this.inputType !== InputType.Empty ? <ion-chip>
              <ion-label>{this.inputType}</ion-label>
            </ion-chip> : null}
            <form id="search-form" onSubmit={this.handleSubmit.bind(this)}>
              <ion-input
                placeholder="`/todo.add: hello, world` for create `todo`"
                autofocus={true}
                value={this.query}
                ref={(el) => this.ionInputElement = el}
                onIonInput={this.handleInput.bind(this)}
              />
              <input type="submit" value="Submit" id="submit-button"/>
            </form>
          </ion-item>
          <ion-item>
            {this.entries_info.map((info) =>
              <ion-button onClick={(e) => this.selectType(e, info)}>{info.type}</ion-button>
            )}
            {this.actions_list.map((action) =>
              <ion-button class="dark-button" onClick={(e) => this.addAction(e, action)}>{action}</ion-button>
            )}
          </ion-item>
        </ion-toolbar>
      </ion-header>
      <ion-content fullscreen>
        <ion-grid>
          <ion-row>
            {this.entries_info.map((info) =>
              this.list[info.type] && this.list[info.type].length > 0 ? this.renderSearchCol(info) : null
            )}
            {this.is_flow && Array.from(this.selected_flow_result.keys()).map((key) =>
              this.renderFlowByKey(key)
            )}
            {!this.is_flow && this.selected_result && this.selected_result.map((item: any) =>
              this.renderCards(item, this.selected_entry.type)
            )}
            {!this.is_flow && this.selected_result.length > 0 &&
              <ion-infinite-scroll ref={(el) => (this.infiniteScroll = el)} onIonInfinite={(ev) => this.loadData(ev)}>
                <ion-infinite-scroll-content
                  loadingSpinner="bubbles"
                  loadingText="Loading more data...">
                </ion-infinite-scroll-content>
              </ion-infinite-scroll>
            }
          </ion-row>
        </ion-grid>
      </ion-content>
    </ion-app>;
  }

  private renderFlowByKey(key: string) {
    return <ion-col>
      <ion-text color="secondary">{key}</ion-text>
      <ion-list>
        {this.selected_flow_result.get(key).map((item: any) =>
          <ion-card onClick={() => this.clickEntry(item.id, this.selected_entry.type)}>
            <ion-card-header>
              <ion-card-subtitle># {this.padLeft(item.id, 4, '')}</ion-card-subtitle>
              <ion-card-title>{item.title}</ion-card-title>
            </ion-card-header>
            <ion-card-content>
              {item.description && <p>{item.description}</p>}
              <ion-badge slot="start">{this.formatDate(item.created_date)}</ion-badge>
            </ion-card-content>
          </ion-card>
        )}
      </ion-list>
    </ion-col>;
  }

  private renderSearchCol(info: EntryInfo) {
    return <ion-col>
      <ion-text color="secondary">{info.type}</ion-text>
      {this.list[info.type] ? this.list[info.type].map((item: any) =>
        <ion-list>{this.renderCards(item, info.type)}</ion-list>
      ) : null
      }
    </ion-col>;
  }

  private renderCards(item: any, type: string) {
    return <div class="entry-show-list">
      <ion-card  size-xs="3" size-md="3" onClick={() => this.clickEntry(item.id, type)}>
        <ion-card-header>
          <ion-card-subtitle># {this.padLeft(item.id, 4, '')}</ion-card-subtitle>
          <ion-card-title>{item.title}</ion-card-title>
        </ion-card-header>
        <ion-card-content>
          {item.description && <p>{item.description}</p>}
          <ion-badge slot="start">{this.formatDate(item.created_date)}</ion-badge>
        </ion-card-content>
      </ion-card>
    </div>;
  }

  // @ts-ignore
  private renderConditionSearch() {
    return <ion-item>
      <select>
        <option value="last_week">Last Week</option>
        <option value="last_month">Last Month</option>
        <option value="last_quarter">Last Quarter</option>
        <option value="last_year">Last Year</option>
      </select>
      <ion-text>Created Date</ion-text>
      <input type="date" id="created_date" name="trip-start" />
      <ion-text>Updated Date</ion-text>
      <input type="date" id="end_date" name="trip-start" />
    </ion-item>;
  }
}
