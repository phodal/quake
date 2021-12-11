import { newSpecPage } from '@stencil/core/testing';
import { QuakeRender } from './quake-render';

describe('quake-render', () => {
  it('renders', async () => {
    // @ts-ignore
    const { root } = await newSpecPage({
      components: [QuakeRender],
      html: '<quake-render></quake-render>',
    });
    // expect(root).toEqualHtml(`
    //   <quake-render>
    //     <mock:shadow-root>
    //       <div>
    //         Hello, World! I'm
    //       </div>
    //     </mock:shadow-root>
    //   </quake-render>
    // `);
  });

  it('renders with values', async () => {

  });
});
