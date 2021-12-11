import { Component, h } from '@stencil/core';
import { marked } from 'marked';
import QuakeRenderer from '../../utils/utils';

@Component({
  tag: 'my-component',
  styleUrl: 'quake-render.css',
  shadow: true,
})
export class QuakeRender {

  render() {
    marked.use({ renderer: new QuakeRenderer() });
    let str = marked.parse('# heading+');

    return <div>
      <div innerHTML={str}></div>
    </div>;
  }
}
