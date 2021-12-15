import { newE2EPage } from '@stencil/core/testing';

describe('graph-bar', () => {
  it('renders', async () => {
    const page = await newE2EPage();
    await page.setContent('<graph-bar></graph-bar>');

    const element = await page.find('graph-bar');
    expect(element).toHaveClass('hydrated');
  });
});
