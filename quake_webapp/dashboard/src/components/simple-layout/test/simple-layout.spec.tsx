import { newSpecPage } from '@stencil/core/testing';
import { SimpleLayout } from '../simple-layout';

describe('simple-layout', () => {
  it('renders', async () => {
    const page = await newSpecPage({
      components: [SimpleLayout],
      html: `<simple-layout></simple-layout>`,
    });
    expect(page.root).toEqualHtml(`
      <simple-layout>
        <mock:shadow-root>
          <slot></slot>
        </mock:shadow-root>
      </simple-layout>
    `);
  });
});
