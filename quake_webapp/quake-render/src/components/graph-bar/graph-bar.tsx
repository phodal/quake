import {Component, h, Host, Prop} from '@stencil/core';
import * as echarts from "echarts";
import {EChartsOption} from "echarts";
import {QuakeDownType} from "../../markdown/quake-down.type";
import {EChartsType} from "echarts/types/dist/echarts";

@Component({
  tag: 'graph-bar',
  styleUrl: 'graph-bar.css',
  shadow: true,
})
export class GraphBar {
  @Prop() config: any = {};
  @Prop() data: QuakeDownType.Table = null;
  @Prop() width = 300;

  myChart: EChartsType;
  element!: HTMLElement;

  componentDidRender() {
    let cells = this.transpose(this.data.rows);

    let width = cells[0].length * 40 + 200 + 'px';
    this.element.style.width = width;
    this.element.style.height = width;

    this.myChart = echarts.init(this.element);
    this.renderGraph(cells);
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

  private renderGraph(cells: any[][]) {
    this.myChart.hideLoading();

    const option: EChartsOption = {
      title: {
        text: ""
      },
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
