import QuakeDown from './quake-down';

describe('render', () => {
  it('render heading', () => {
    let data = new QuakeDown('# heading+',() => "").gen();

    expect(data.length).toEqual(1);
    expect(data).toEqual([{ type: 'heading', depth: 1, text: '',  anchor: '' }])
  });

  it('render heading with links', () => {
    let quakeDown = new QuakeDown('# [heading+](https://quake.inherd.org)\n', () => "");
    let data = quakeDown.gen();

    expect(data).toEqual([{ type: 'heading', depth: 1, text: '', anchor: '' }])
  });
});
