import QuakeRenderer from './utils';
import { marked } from 'marked';

describe('render', () => {
  it('render', () => {
    marked.use({ renderer: new QuakeRenderer() });
    console.log(marked.parse('# heading+'));
  });
});
