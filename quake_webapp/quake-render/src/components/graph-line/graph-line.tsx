import {Component, Host, h, Prop} from '@stencil/core';
import {EChartsType} from "echarts/types/dist/echarts";
import {QuakeDownType} from "../../markdown/quake-down.type";
import * as echarts from "echarts";
import {EChartsOption} from "echarts";

@Component({
  tag: 'graph-line',
  styleUrl: 'graph-line.css',
  shadow: true,
})
export class GraphLine {
  @Prop() config: any = {};
  @Prop() data: QuakeDownType.Table = null;
  @Prop() width = 300;

  myChart: EChartsType;
  element!: HTMLElement;

  componentDidRender() {
    let cells = this.transpose(this.data.rows);
    let legend = this.data.header;

    let width = cells[0].length * 40 + 200 + 'px';
    this.element.style.width = width;
    this.element.style.height = width;

    this.myChart = echarts.init(this.element);
    this.renderGraph(cells, legend);
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

  private renderGraph(cells: any[][], legend: any[]) {
    this.myChart.hideLoading();

    let series = [];
    for (let i = 1; i < cells.length; i++) {
      series.push({
        name: legend[i],
        data: cells[i],
        type: 'line'
      })
    }

    const option: EChartsOption = {
      title: {
        text: ""
      },
      tooltip: {
        trigger: 'axis'
      },
      xAxis: {
        type: 'category',
        data: cells[0]
      },
      legend: {
        data: legend
      },
      yAxis: {
        type: 'value'
      },
      series: series
    };

    this.myChart.setOption(option);
  }
}
