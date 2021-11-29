import {Component, Element, h, Prop, State} from '@stencil/core';
import {MeiliSearch} from "meilisearch";
import dayjs from "dayjs";
import axios from "axios";

// only for: IDEA jump
// @ts-ignore
import {IonSearchbar} from "@ionic/core";

interface ActionDefine {
  object: String,
  action: String,
  text: String,
  parameters: String[]
}

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
  @State() inputType: String = "";
  @State() actionDefine: ActionDefine = null;

  client = new MeiliSearch({
    host: 'http://127.0.0.1:7700'
  })

  handleInput(event) {
    const that = this;
    this.query = event.target.value;
    if (this.query.length == 0) {
      that.items = [];
      this.inputType = '';
      return;
    }

    if (this.query.startsWith(".")) {
      this.inputType = 'Action'
      return;
    }

    this.actionDefine = null;
    this.inputType = 'Search';
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

  async handleSubmit(e) {
    e.preventDefault()
    const that = this;
    requestAnimationFrame(() => {
      axios.get('http://127.0.0.1:8000/action/query/', {
        params: {
          input: this.query.substring(1,)
        }
      }).then(response => {
        if (response.data.object) {
          that.actionDefine = response.data
        } else {
          that.actionDefine = null;
          that.presentToast(response.data.msg);
        }
      }).catch(function (error) {
        console.log(error);
      });
    });
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
      <ion-header translucent>
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
          {this.actionDefine ?
            <ion-item>{this.actionDefine.object}, {this.actionDefine.action}, {this.actionDefine.text}, {this.actionDefine.parameters} </ion-item>
            : null
          }
          {this.items.length > 0
            ? this.items.map((item: any) =>
              <ion-item>
                <ion-badge slot="start">{this.formatDate(item.created_date)}</ion-badge>
                {item.title}
              </ion-item>)
            : null
          }
        </ion-list>
      </ion-content>
    </ion-app>;
  }
}
