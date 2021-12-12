import { Component, Host, h, Prop } from '@stencil/core';
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

  componentDidRender() {
    this.renderGraph(this.element);
  }

  render() {
    return (
      <Host>
        <p>
          <label htmlFor="nRadius">
            radius = <span id="nRadius-value">…</span>
          </label>
          <input type="range" min="1" max="150" id="nRadius" value={this.value} />
        </p>
        <div class='chart' ref={(el) => this.element = el as HTMLElement} />
      </Host>
    );
  }

  private renderGraph(chartElement: any) {
    const width = 600;
    const height = 300;
    const that = this;

    const holder = d3.select(chartElement)
      .append('svg')
      .attr('width', width)
      .attr('height', height);

// draw the circle
    holder.append('circle')
      .attr('cx', 300)
      .attr('cy', 150)
      .style('fill', 'none')
      .style('stroke', 'blue')
      .attr('r', 120);

    d3.select('#nRadius').on('input', function() {
      update(+that.value);
    });

    update(120);

    function update(nRadius) {

      // adjust the text on the range slider
      d3.select('#nRadius-value').text(nRadius);
      d3.select('#nRadius').property('value', nRadius);

      holder.selectAll('circle')
        .attr('r', nRadius);
    }

  }
}
