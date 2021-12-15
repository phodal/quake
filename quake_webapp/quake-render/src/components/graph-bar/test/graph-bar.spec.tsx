import { newSpecPage } from '@stencil/core/testing';
import { GraphBar } from '../graph-bar';

describe('graph-bar', () => {
  it('renders', async () => {
    const page = await newSpecPage({
      components: [GraphBar],
      html: `<graph-bar></graph-bar>`,
    });
    expect(page.root).toEqualHtml(`
      <graph-bar>
        <mock:shadow-root>
          <slot></slot>
        </mock:shadow-root>
      </graph-bar>
    `);
  });
});
