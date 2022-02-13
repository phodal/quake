import {Component, Element, Event, EventEmitter, h, Prop, State} from '@stencil/core';
import {MeiliSearch} from "meilisearch";
import dayjs from "dayjs";

// only for: IDEA jump
// @ts-ignore
// import {IonSearchbar} from "@ionic/core";
import {createTransflow, init_wasm, parseAction} from "./quake-core-wrapper";

export interface ActionDefine {
  entry: String,
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
  properties: object[],
  action: any[],
  flows: FlowDefine[]
}

export interface SearchResult {
  [id: string]: Array<any>
}

enum InputType {
  Empty,
  Search = 'Search',
  Action = 'Action',
  Transflow = 'Transflow'
}

@Component({
  tag: 'quake-dashboard',
  styleUrl: 'quake-dashboard.css',
  shadow: false,
})
export class QuakeDashboard {
  ionInputElement;
  infiniteScroll: any;

  @Element() el: HTMLElement;

  @Prop() indexName: string = "";

  @State() items: Array<object> = [];
  @State() list: SearchResult = {};

  @State() isAction = false;

  @State() query: string = "";
  @State() inputType: InputType = InputType.Empty;
  @State() actionDefine: ActionDefine = null;
  @State() entriesInfo: EntryInfo[] = [];
  @State() actionsList: string[] = [];

  @State() selectedEntry: EntryInfo = null;
  @State() selectedFlowResult: Map<string, object[]> = new Map();
  @State() selectedResult: any[] = [];
  @State() selectedEntryFileProp: string = "";

  @State() isFlow: boolean = false;
  @State() offset: object = {};

  @State() flow_index: number = 1;
  @State() generated_code: string = "";
  @State() layout: any = {};
  @State() showSimpleLayout: boolean = true;


  @Event({
    eventName: 'dispatchAction',
    composed: true,
    cancelable: true,
    bubbles: true,
  }) dispatchAction: EventEmitter<ActionDefine>;

  client = (window as any).Quake?.client ? (window as any).Quake.client : new MeiliSearch({
    host: 'http://127.0.0.1:7700'
  });

  componentWillLoad() {
    init_wasm().then(() => {
    });

    const that = this;
    this.loadingElement("suggest", {}, (res: any) => {
      that.entriesInfo = res.data.entries;
      that.actionsList = res.data.actions;
    }).then(() => {
    });

    this.loadingElement("layout", {}, (res: any) => {
      that.layout = res.data;
    }).then(() => {
    });
  }

  async loadingElement(type: string, params: object = {}, callback: any) {
    const loading: any = document.createElement('ion-loading');
    loading.cssClass = 'my-custom-class';
    loading.message = 'Please wait...';
    loading.duration = 500;

    const fetchEl = document.createElement('fetch-api');
    fetchEl.setAttribute("type", type);
    fetchEl.setAttribute("params", JSON.stringify(params));
    fetchEl.addEventListener("fetchSuccess", (res: any) => {
      let response = res.detail;
      loading.onDidDismiss().then(() => {
      });
      callback(response);
    })

    loading.appendChild(fetchEl);
    document.body.appendChild(loading);
    await loading.present();
  }

  handleInput(event) {
    const that = this;
    this.query = event.target.value;
    this.handleQuery(that);
    this.showSimpleLayout = false;
  }

  private handleQuery(that: this) {
    if (this.query.length == 0) {
      this.items = [];
      this.inputType = InputType.Empty;
      this.list = {};
      return;
    }

    let first_char = this.query[0];
    switch (first_char) {
      case "/":
        this.inputType = InputType.Action
        return
      case ":":
        this.inputType = InputType.Transflow;
        return
    }

    this.actionDefine = null;
    this.inputType = InputType.Search;
    this.createSearch(that);
  }

  private createSearch(that: this) {
    for (let info of this.entriesInfo) {
      this.search_item(that, info.type);
    }
  }

  private search_item(that: this, doc: string) {
    const index = this.client.index(doc)
    index.search(that.query, {
      attributesToHighlight: ['overview']
    }).then((result) => {
      that.list[doc] = result.hits;
      that.items = result.hits;
    })
  }

  private queryItems(offset: number) {
    const index = this.client.index(this.selectedEntry.type)
    return index.search('', {
      attributesToHighlight: ['overview'],
      limit: 40,
      offset
    })
  }

  formatDate(str) {
    return dayjs(str * 1000).format('YYYY-MM-DD');
  }

  padLeft(nr, n, str) {
    return Array(n - String(nr).length + 1).join(str || '0') + nr;
  }

  async selectType(_e: Event, info: EntryInfo) {
    this.resetInput();
    this.selectedEntry = info;
    this.selectedEntryFileProp = null;
    for (let property of this.selectedEntry.properties) {
      for (let key in property) {
        if(property[key] == "File") {
          this.selectedEntryFileProp = key;
        }
      }
    }

    this.query = '/' + info.type + '.';
    this.handleQuery(this);
  }

  async addAction(_e: Event, action: string) {
    this.resetInput();
    this.showSimpleLayout = false;
    if (!this.selectedEntry) {
      return;
    }
    if (action == 'show') {
      let type = this.selectedEntry.type;
      this.offset[type] = 0;
      this.queryItems(this.offset[type]).then((result) => {
        this.offset[type] = this.offset[type] + 40;

        let parsed = result.hits;
        this.isFlow = !!this.selectedEntry.flows
        if (this.isFlow) {
          this.processFlow(parsed);
        } else {
          this.selectedResult = parsed;
        }
      })
    }

    if (this.query.startsWith("/") && this.query.endsWith(".")) {
      this.query = this.query + action;
      this.handleQuery(this);
    }
  }

  private processFlow(parsed) {
    this.isFlow = true;
    let results_map = new Map();
    let flow = this.selectedEntry.flows[0];
    for (let item of flow.items) {
      results_map.set(item, []);
    }

    for (let el of parsed) {
      results_map.get(el.status)?.push(el);
    }

    this.selectedFlowResult = results_map;
  }

  private resetInput() {
    this.selectedResult = [];
    this.selectedFlowResult = new Map();
  }

  async createTransflow() {
    let flow_name = `temp_${this.flow_index}`;
    let flow = this.query.substring(1,);
    createTransflow(flow_name, flow).then(src => {
      src = src + "\n " + `Quake.router.addRoutes({path: '/transflow/show_${flow_name}', action: tl_${flow_name} },)`;

      // todo: remove old id??
      const script = document.createElement('script');
      script.setAttribute('id', `temp-${this.flow_index}`);

      this.generated_code = src;

      script.innerHTML = src;
      document.body.appendChild(script);

      const nav = document.createElement('a');
      nav.setAttribute("href", `/transflow/show_${flow_name}/`);
      nav.innerText = flow_name;

      let navNode = document.getElementById("transflow-nav");
      navNode.appendChild(nav);

      this.flow_index = this.flow_index + 1;
    })
  }

  async handleSubmit(e) {
    e.preventDefault()

    if (this.query.startsWith(":")) {
      this.inputType = InputType.Transflow;
      await this.createTransflow();
      return;
    }

    if (!this.query.startsWith("/")) {
      this.inputType = InputType.Search;
      return;
    }

    const that = this;
    parseAction(this.query.substring(1,)).then(data => {
      if (data.entry) {
        that.actionDefine = data
        that.dispatchAction.emit(data);
      } else {
        that.actionDefine = null;
        that.presentToast(data.msg);
      }
    }).catch(function (error) {
      console.log(error);
    });
  }

  async showPdf(entry: string, file_prop: string) {
    let url = `/processor/${entry}?file_prop=${file_prop}`;
    const modal: any = document.createElement('ion-modal');

    const viewer: any = document.createElement('quake-viewer');
    viewer.setAttribute('url', url);

    modal.appendChild(viewer);
    document.body.appendChild(modal);

    await modal.present();
  }

  editEntry(id: string, entry: string) {
    this.dispatchAction.emit({
      parameters: [id],
      action: 'update',
      entry: entry,
    } as any);
  }

  showEntry(id: string, entry: string) {
    this.dispatchAction.emit({
      parameters: [id],
      action: 'show',
      entry: entry,
    } as any);
  }

  async presentToast(msg: string) {
    const toast: any = document.createElement('ion-toast');
    toast.message = msg;
    toast.duration = 2000;

    document.body.appendChild(toast);
    return toast.present();
  }

  loadData(_ev) {
    if (this.infiniteScroll && this.infiniteScroll.disabled) {
      return "no more data";
    }

    let type = this.selectedEntry.type;
    this.queryItems(this.offset[type]).then((result) => {
      this.offset[type] = this.offset[type] + 40;
      this.infiniteScroll.complete().then(_r => {
      });

      let parsed = result.hits;
      if (parsed.length == 0) {
        this.infiniteScroll.disabled = true;
        return;
      }

      this.selectedResult = this.selectedResult.concat(parsed);
    })

  }

  render() {
    return <div>
      <ion-header translucent>
        <ion-toolbar>
          <ion-item>
            {this.inputType !== InputType.Empty ? <ion-chip>
              <ion-icon name="airplane-outline"></ion-icon>
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
            {this.entriesInfo.map((info) =>
              <ion-button onClick={(e) => this.selectType(e, info)}>{info.type}</ion-button>
            )}
            {this.actionsList.map((action) =>
              <ion-button class="dark-button" onClick={(e) => this.addAction(e, action)}>{action}</ion-button>
            )}
          </ion-item>
        </ion-toolbar>
      </ion-header>
      <ion-content fullscreen>
        <ion-grid>
          <ion-row>
            {this.generated_code && <pre><code class="javascript">{this.generated_code}</code></pre>}
            {this.entriesInfo.map((info) =>
              this.list[info.type] && this.list[info.type].length > 0 ? this.renderSearchCol(info) : null
            )}
            {this.isFlow && Array.from(this.selectedFlowResult.keys()).map((key) =>
              this.renderFlowByKey(key)
            )}
            {!this.isFlow && this.selectedResult && this.selectedResult.map((item: any) =>
              this.renderCard(item, this.selectedEntry.type)
            )}
            {!this.isFlow && this.selectedResult.length > 0 &&
              <ion-infinite-scroll ref={(el) => (this.infiniteScroll = el)} onIonInfinite={(ev) => this.loadData(ev)}>
                <ion-infinite-scroll-content
                  loadingSpinner="bubbles"
                  loadingText="Loading more data...">
                </ion-infinite-scroll-content>
              </ion-infinite-scroll>
            }
          </ion-row>
        </ion-grid>
        {this.showSimpleLayout && this.layout && <simple-layout layout={this.layout}/>}
      </ion-content>
    </div>;
  }

  private renderFlowByKey(key: string) {
    return <ion-col>
      <ion-text color="secondary">{key}</ion-text>
      <ion-list>
        {this.selectedFlowResult.get(key).map((item: any) =>
          <ion-card>
            <ion-card-header>
              <ion-card-subtitle># {this.padLeft(item.id, 4, '')}
                <ion-icon name="book-outline" onClick={() => this.showEntry(item.id, this.selectedEntry.type)}/>
                <ion-icon name="create-outline" onClick={() => this.editEntry(item.id, this.selectedEntry.type)}/>
              </ion-card-subtitle>
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
        <ion-list>{this.renderCard(item, info.type)}</ion-list>
      ) : null
      }
    </ion-col>;
  }

  private renderCard(item: any, type: string) {
    let entryFileProp = this.selectedEntryFileProp;

    return <div class="entry-show-list">
      <ion-card>
        <ion-card-header>
          <ion-card-subtitle># {this.padLeft(item.id, 4, '')}
            {entryFileProp &&
              <ion-icon name="document-outline" onClick={() => this.showPdf(type, item[entryFileProp])}/>
            }
            <ion-icon name="book-outline" onClick={() => this.showEntry(item.id, type)}/>
            <ion-icon name="create-outline" onClick={() => this.editEntry(item.id, type)}/>
          </ion-card-subtitle>
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
      <input type="date" id="created_date" name="trip-start"/>
      <ion-text>Updated Date</ion-text>
      <input type="date" id="end_date" name="trip-start"/>
    </ion-item>;
  }
}
