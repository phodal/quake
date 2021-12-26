import {Component, h, Host, Prop, State} from '@stencil/core';

export interface Layout {
  name: string,
  rows: LayoutRow[],
}

export interface LayoutRow {
  columns: LayoutComponent[]
}

export interface LayoutComponent {
  name: string,
  is_empty: boolean,
  is_pure_component: boolean,
  flow: string,
  size: number
}

@Component({
  tag: 'simple-layout',
  styleUrl: 'simple-layout.css',
  shadow: false,
})
export class SimpleLayout {
  @Prop() layout: Layout;
  hostEl: HTMLElement;

  @State() flowMap: any = {};
  @State() slots: any[] = [];

  // build logic
  // 1. collection element of layouts before render.
  // 2. recursive layout and append element to node
  componentWillRender() {
    this.createFlows();
  }

  private createFlows() {
    const that = this;
    if (!(this.layout.rows && this.layout.rows.length > 0)) {
      return;
    }

    let rowId = 0;
    for (let row of this.layout.rows) {
      let colId = 0;
      for (let component of row.columns) {
        let id = that.layoutId(rowId, colId);

        if (component.is_pure_component) {
          this.createPureComponent(component, that, id);
        } else {
          this.createFlowComponent(component, that, id);
        }
        colId = colId + 1;
      }
      rowId = rowId + 1;
    }
  }

  private createPureComponent(component: LayoutComponent, that: this, id: string) {
    let element = document.createElement(component.flow);
    // todo: refactor flow map to add by field;
    if (!that.flowMap[id]) {
      that.flowMap[id] = {};
    }

    // make render after element create
    setTimeout(() => {
      (that.flowMap[id].hostEl as HTMLElement).appendChild(element);
    }, 0);
  }

  private createFlowComponent(component: LayoutComponent, that: this, id: string) {
    let flow = component.flow;
    let func_name = "tl_" + flow;
    try {
      let flow_func = (window as any).Quake.flows[func_name];
      if (flow_func) {
        // todo: remove unused parameters
        flow_func({}, {}).then((flow) => {
          (that.flowMap[id].hostEl as HTMLElement).appendChild(flow);
        });
      }
    } catch (err) {
      console.error(err);
    }
  }

  private layoutId(rowId: number, colId: number) {
    return 'layout-' + rowId + '-' + colId;
  }

  private addElementToMap(hostEl, id: string) {
    if (!this.flowMap[id]) {
      this.flowMap[id] = {};
    }
    this.flowMap[id].hostEl = hostEl;
    this.flowMap[id].id = id;
  }

  render() {
    return (
      <Host ref={(el) => this.hostEl = el}>
        {!!this.layout.rows &&
          <ion-grid>
            {this.layout.rows.map((row, rowId) =>
              <ion-row>
                {row.columns.map((col, colId) =>
                  <ion-col class="quake-component" size={col.size.toString()}
                           ref={(el) => (this.addElementToMap(el, this.layoutId(rowId, colId)))}>
                  </ion-col>
                )}
              </ion-row>
            )}
          </ion-grid>
        }
      </Host>
    );
  }
}
