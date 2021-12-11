import QuakeGen from './quake-gen';

describe('render', () => {
  it('render heading', () => {
    let data = new QuakeGen('# heading+').gen();
    expect(data.length).toEqual(1);
  });

  it('render heading with links', () => {
    let data = new QuakeGen('# [heading+](https://quake.inherd.org)').gen();
    expect(data.length).toEqual(1);
  });
});
