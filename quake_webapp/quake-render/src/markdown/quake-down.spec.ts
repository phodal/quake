import QuakeDown from './quake-down';

describe('render', () => {
  it('build heading', () => {
    let data = new QuakeDown('# heading+', () => "").build();

    expect(data.length).toEqual(1);
    expect(data).toEqual([{type: 'heading', depth: 1, text: '', anchor: ''}])
  });

  it('build heading with links', () => {
    let quakeDown = new QuakeDown('# [heading+](https://quake.inherd.org)\n', () => "");
    let data = quakeDown.build();

    expect(data).toEqual([{type: 'heading', depth: 1, text: '', anchor: ''}])
  });

  it('build code type', () => {
    let quakeDown = new QuakeDown('```@graph(\'bar\')\n' +
      'demo\n' +
      '```', () => "");
    let data = quakeDown.build();

    expect(data).toEqual([{
      type: 'code',
      code_type: 'Graph',
      code_param: 'bar',
      lang: "@graph('bar')",
      text: 'demo'
    }])
  });
});
