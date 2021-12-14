import { newSpecPage } from '@stencil/core/testing';
import { EmbedLink } from '../embed-link';

describe('embed-link', () => {
  it('renders', async () => {
    const page = await newSpecPage({
      components: [EmbedLink],
      html: `<embed-link></embed-link>`,
    });
    expect(page.root).toEqualHtml(`
      <embed-link>
        <mock:shadow-root>
          <slot></slot>
        </mock:shadow-root>
      </embed-link>
    `);
  });
});
