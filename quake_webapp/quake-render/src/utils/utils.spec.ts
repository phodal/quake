import QuakeRenderer from './utils';
import { marked } from 'marked';

describe('render', () => {
  it('render', () => {
    marked.use({ renderer: QuakeRenderer});
    console.log(marked.parse('# heading+'));
  });
});
