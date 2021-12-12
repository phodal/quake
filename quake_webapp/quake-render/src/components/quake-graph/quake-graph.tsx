import { Component, h, Host, Prop } from '@stencil/core';
import * as d3 from 'd3';

@Component({
  tag: 'quake-graph',
  styleUrl: 'quake-graph.css',
  shadow: true,
})
export class QuakeGraph {
  element!: HTMLElement;
  @Prop() config: any = {};
  @Prop() data: any = [{
    children: [{
      name: 'Hello',
      value: 2,
    }, {
      name: 'World',
      value: 2,
    }],
    name: 'Demo',
    checked: true,
  }];
  // @State() value: number = 0;
  private value: number = 0;
  private holder: any;

  componentDidRender() {
    this.renderGraph(this.element);
  }

  private inputChanged = (event: Event) => {
    this.value = Number((event.target as HTMLInputElement).value);
    this.update(this.value);
  };

  private update(nRadius) {
    this.holder.selectAll('circle')
      .attr('r', nRadius);
  }

  render() {
    return (
      <Host>
        <p>
          <label htmlFor='nRadius'>
            {this.value}
          </label>
          <input type='range' min='1' max='150' id='nRadius' value={this.value} onChange={this.inputChanged} />
        </p>
        <div class='chart' ref={(el) => this.element = el as HTMLElement} />
      </Host>
    );
  }

  private renderGraph(chartElement: any) {
    const width = 600;
    const height = 300;

    this.holder = d3.select(chartElement)
      .append('svg')
      .attr('width', width)
      .attr('height', height);

    this.holder.append('circle')
      .attr('cx', 300)
      .attr('cy', 150)
      .style('fill', 'none')
      .style('stroke', 'blue')
      .attr('r', 120);

    this.update(120);
  }
}
