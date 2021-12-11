import QuakeDown from './quake-down';

describe('render', () => {
  it('render heading', () => {
    let data = new QuakeDown('# heading+',() => "").gen();
    console.log(data);
    expect(data.length).toEqual(1);
  });

  it('render heading with links', () => {
    // let renderer = new marked.Renderer();
    // let data = new QuakeDown('# [heading+](https://quake.inherd.org)', renderer, this.parseInline).gen();
    // expect(data.length).toEqual(1);
  });
});
