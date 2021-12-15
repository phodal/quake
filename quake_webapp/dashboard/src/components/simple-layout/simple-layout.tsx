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
  flow: string,
  size: number
}

@Component({
  tag: 'simple-layout',
  styleUrl: 'simple-layout.css',
  shadow: true,
})
export class SimpleLayout {
  @Prop() layout: Layout;
  hostEl: HTMLElement;

  @State() flowMap: any =  {};
  @State() slots: any[] = [];

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
        let flow = component.flow;
        let func_name = "tl_" + flow;
        try {
          let flow_func = (window as any).Quake.flows[func_name];
          if (flow_func) {
            console.log("calling: " + func_name);
            let id = that.layoutId(rowId, colId);

            flow_func({}, {}).then((flow) => {
              (that.flowMap[id].hostEl as HTMLElement).appendChild(flow);
            });
          }
        } catch (err) {
          console.error(err);
        }
        colId = colId + 1;
      }
      rowId = rowId + 1;
    }
  }

  private layoutId(rowId: number, colId: number) {
    return 'layout-' + rowId + '-' + colId;
  }

  private addElementToMap(hostEl, id: string) {
    this.flowMap[id] = {
      hostEl: hostEl,
      id
    }
  }

  render() {
    console.log(this.flowMap);
    return (
      <Host ref={(el) => this.hostEl = el}>
        {!!this.layout.rows &&
          <ion-grid>
            {this.layout.rows.map((row, rowId) =>
              <ion-row>
                {row.columns.map((col, colId) =>
                  <ion-col size={col.size.toString()} ref={(el) => (this.addElementToMap(el, this.layoutId(rowId, colId)))}>
                    <p slot={this.layoutId(rowId, colId)}/>
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
