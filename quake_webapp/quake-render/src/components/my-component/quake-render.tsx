import { Component, h, State } from '@stencil/core';
import QuakeGen from '../../utils/quake-gen';

@Component({
  tag: 'quake-render',
  styleUrl: 'quake-render.css',
  shadow: true,
})
export class QuakeRender {
  @State() markdownData: any[] = [];

  componentWillLoad() {
    let content = `# [heading+](https://quake.inherd.org)
> blockquote

# h1
## h2
### h3
#### h4
##### h5
###### h6

---

123

456

`;
    this.markdownData = new QuakeGen(content).gen();
  }

  render() {
    return <div>
      {this.markdownData.map((item: any) =>
        QuakeRender.conditionRender(item),
      )}
    </div>;
  }

  private static conditionRender(item: any) {
    let temp: string;
    switch (item.type) {
      case 'heading':
        console.log(item);
        temp = this.create_heading(item);
        break;
      case 'blockquote':
        temp = <blockquote innerHTML={item.text} />;
        break;
      case 'hr':
        temp = <hr />;
        break;
      case 'paragraph':
        temp = <p innerHTML={item.data} />;
        break;
      case 'space':
        break;
      default:
        console.log(item);
        temp = <span/>;
    }

    return temp;
  }

  private static create_heading(item: any) {
    let heading: string;
    switch (item.depth) {
      case 1:
        heading = <h1 innerHTML={item.text} class='quake-heading' id={item.anchor} />
        break;
      case 2:
        heading = <h2 innerHTML={item.text} class='quake-heading' id={item.anchor} />
        break;
      case 3:
        heading = <h3 innerHTML={item.text} class='quake-heading' id={item.anchor} />
        break;
      case 4:
        heading = <h4 innerHTML={item.text} class='quake-heading' id={item.anchor} />
        break;
      case 5:
        heading = <h5 innerHTML={item.text} class='quake-heading' id={item.anchor} />
        break;
      case 6:
        heading = <h6 innerHTML={item.text} class='quake-heading' id={item.anchor} />
        break;
      default:
        console.log(item);
    }

    return heading
  }
}
