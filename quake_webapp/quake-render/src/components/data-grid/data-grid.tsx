import {Component, Host, h, Prop} from '@stencil/core';
import {Grid, GridOptions} from 'ag-grid-community';
import {QuakeDownType} from "../../markdown/quake-down.type";
import {ColDef} from "ag-grid-community/dist/lib/entities/colDef";

@Component({
  tag: 'data-grid',
  styleUrl: 'data-grid.css',
  shadow: true,
})
export class DataGrid {
  @Prop() data: QuakeDownType.Table = null;
  element!: HTMLElement;

  componentDidRender() {
    if (this.data) {

      let columnDefs: ColDef[] = [];
      for (let item of this.data?.header) {
        columnDefs.push({
          field: item,
          sortable: true,
          filter: true,
          rowDrag: true
        })
      }

      let rowData = [];
      for (let column of this.data?.rows) {
        let row = {}
        for (let i = 0; i < column.length; i++) {
          row[columnDefs[i].field] = column[i];
        }
        rowData.push(row);
      }

      const gridOptions: GridOptions = {
        domLayout: 'autoHeight',
        accentedSort: true,
        suppressMultiSort: true,
        columnDefs: columnDefs,
        rowData: rowData
      };

      new Grid(this.element, gridOptions);
    }
  }

  render() {
    return (
      <Host>
        <div class='data-grid ag-theme-alpine' ref={(el) => this.element = el as HTMLElement}/>
      </Host>
    );
  }
}
