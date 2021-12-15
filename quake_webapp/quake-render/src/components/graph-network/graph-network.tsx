import {Component, h, Host, Prop, State} from '@stencil/core';
import * as echarts from 'echarts';

@Component({
  tag: 'graph-network',
  styleUrl: 'graph-network.css',
  shadow: true,
})
export class GraphNetwork {
  @State() myChart: any;
  element!: HTMLElement;
  @Prop() config: any = {};
  @Prop() data: any = {
    nodes: [
      {id: 0, name: 'hello', category: 0},
      {id: 1, name: 'world', category: 1}
    ],
    links: [{source: "1", target: "0"}],
    categories: [{name: 'A'}, {name: 'B'}]
  };

  componentDidRender() {
    this.myChart = echarts.init(this.element);
    this.renderGraph();
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
    this.data.nodes.forEach(function (node) {
      node.symbolSize = 10;
    });
    const option: any = {
      title: {
        text: '',
        subtext: 'Default layout',
        top: 'bottom',
        left: 'right'
      },
      tooltip: {},
      legend: [
        {
          data: this.data.categories.map(function (a) {
            return a.name;
          })
        }
      ],
      series: [
        {
          name: '',
          type: 'graph',
          layout: 'force',
          data: this.data.nodes,
          links: this.data.links,
          categories: this.data.categories,
          roam: true,
          label: {
            position: 'right'
          },
          force: {
            repulsion: 100
          }
        }
      ]
    };

    this.myChart.setOption(option);
  }
}
