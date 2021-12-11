import QuakeDown from './quake-down';
import { marked } from 'marked';

describe('render', () => {
  it('render heading', () => {
    let renderer = new marked.Renderer();
    let data = new QuakeDown('# heading+', renderer).gen();
    expect(data.length).toEqual(1);
  });

  it('render heading with links', () => {
    let renderer = new marked.Renderer();
    let data = new QuakeDown('# [heading+](https://quake.inherd.org)', renderer).gen();
    expect(data.length).toEqual(1);
  });
});
