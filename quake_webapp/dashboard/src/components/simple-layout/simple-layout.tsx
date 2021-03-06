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
  width: number,
  height: number,
  component_type: string
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

  componentWillRender() {
    this.createFlows();
  }

  // 1. collection element of layouts before render.
  // 2. recursive layout and append element to node.
  private createFlows() {
    const that = this;
    if (!(this.layout.rows && this.layout.rows.length > 0)) {
      return;
    }

    let rowId = 0;
    for (let row of this.layout.rows) {
      let colId = 0;
      for (let component of row.columns) {
        let id = SimpleLayout.layoutId(rowId, colId);
        switch (component.component_type) {
          case "native":
            this.createPureComponent(component, that, id);
            break;
          case "flow":
            this.createFlowComponent(component, that, id);
            break;
          default:
            console.log("not supported type: " + component.component_type);
        }

        colId = colId + 1;
      }
      rowId = rowId + 1;
    }
  }

  private createPureComponent(component: LayoutComponent, that: this, id: string) {
    let element = document.createElement(component.flow);
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

  private static layoutId(rowId: number, colId: number) {
    return 'layout-' + rowId + '-' + colId;
  }

  private addElementToMap(hostEl, id: string) {
    if (!this.flowMap[id]) {
      this.flowMap[id] = {};
    }
    this.flowMap[id].hostEl = hostEl;
    this.flowMap[id].id = id;
  }

  private static countHeight(height: number) {
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
                  <ion-col
                    class="quake-component" size={col.width.toString()}
                    style={SimpleLayout.countHeight(col.height)}
                    ref={(el) => (this.addElementToMap(el, SimpleLayout.layoutId(rowId, colId)))}>
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
