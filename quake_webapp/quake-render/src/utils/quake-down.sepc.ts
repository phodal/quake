import QuakeDown from './quake-down';

describe('render', () => {
  it('render heading', () => {
    let data = new QuakeDown('# heading+').gen();
    expect(data.length).toEqual(1);
  });

  it('render heading with links', () => {
    let data = new QuakeDown('# [heading+](https://quake.inherd.org)').gen();
    expect(data.length).toEqual(1);
  });
});
