import { newSpecPage } from '@stencil/core/testing';
import { GraphNetwork } from '../graph-network';

describe('quake-graph', () => {
  it('renders', async () => {
    const page = await newSpecPage({
      components: [GraphNetwork],
      html: `<quake-graph></quake-graph>`,
    });
    expect(page.root).toEqualHtml(`
      <quake-graph>
        <mock:shadow-root>
          <slot></slot>
        </mock:shadow-root>
      </quake-graph>
    `);
  });
});
