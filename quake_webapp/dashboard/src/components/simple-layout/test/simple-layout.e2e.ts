import { newE2EPage } from '@stencil/core/testing';

describe('simple-layout', () => {
  it('renders', async () => {
    const page = await newE2EPage();
    await page.setContent('<simple-layout></simple-layout>');

    const element = await page.find('simple-layout');
    expect(element).toHaveClass('hydrated');
  });
});
