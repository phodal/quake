import {Component, Host, h, State, Prop} from '@stencil/core';
import * as echarts from "echarts";
import {QuakeDownType} from "../../markdown/quake-down.type";

@Component({
  tag: 'graph-bar',
  styleUrl: 'graph-bar.css',
  shadow: true,
})
export class GraphBar {
  @State() myChart: any;
  element!: HTMLElement;
  @Prop() config: any = {};

  @Prop() data: QuakeDownType.Table = null;

  componentDidRender() {
    this.myChart = echarts.init(this.element);
    this.renderGraph();
  }

  transpose(array: any[][]) {
    if (array.length === 0) {
      return;
    }

    return array[0].map((_col, i) => array.map(row => row[i]));
  }

  render() {
    return (
      <Host>
        <div class='chart' ref={(el) => this.element = el as HTMLElement}/>
      </Host>
    );
  }

  private renderGraph() {
    this.myChart.hideLoading();
    let cells = this.transpose(this.data.rows);

    const option: any = {
      xAxis: {
        type: 'category',
        data: cells[0]
      },
      yAxis: {
        type: 'value'
      },
      series: [
        {
          data: cells[1],
          type: 'bar'
        }
      ]
    };

    this.myChart.setOption(option);
  }
}
