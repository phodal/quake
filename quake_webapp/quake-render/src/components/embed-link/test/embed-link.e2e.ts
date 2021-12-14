import { newE2EPage } from '@stencil/core/testing';

describe('embed-link', () => {
  it('renders', async () => {
    const page = await newE2EPage();
    await page.setContent('<embed-link></embed-link>');

    const element = await page.find('embed-link');
    expect(element).toHaveClass('hydrated');
  });
});
