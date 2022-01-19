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
  flow: string,
  size: number,
  height: number
  is_empty: boolean,
  is_pure_component: boolean,
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
    let funcName = "tl_" + flow;
    try {
      let flowFunc = (window as any).Quake.flows[funcName];
      if (flowFunc) {
        // todo: remove unused parameters
        flowFunc({}, {}).then((flow) => {
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

  private countHeight(height: number) {
    let styles = {};
    let height_style = "100%";

    if (height > 0 && height <= 12) {
      height_style = height * (screen.height / 12) + "px"
    }

    styles['height'] = height_style;
    styles['max-height'] = height_style;
    return styles;
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
                           style={this.countHeight(col.height)}
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
