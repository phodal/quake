import { Component, h, State } from '@stencil/core';
import QuakeDown from '../../utils/quake-down';

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

| 工具          | 项目地址                          |
| ------------- | ------------------------------- |
| wrk           | [https://github.com/wg/wrk](https://github.com/wg/wrk)     |
| Apache JMeter | [https://jmeter.apache.org/](https://jmeter.apache.org/)   |

`;
    this.markdownData = new QuakeDown(content).gen();
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
      case 'table':
        temp = this.create_table(item);
        break;
      default:
        console.log(item);
        temp = <span />;
    }

    return temp;
  }

  private static create_heading(item: any) {
    let heading: string;
    switch (item.depth) {
      case 1:
        heading = <h1 innerHTML={item.text} class='quake-heading' id={item.anchor} />;
        break;
      case 2:
        heading = <h2 innerHTML={item.text} class='quake-heading' id={item.anchor} />;
        break;
      case 3:
        heading = <h3 innerHTML={item.text} class='quake-heading' id={item.anchor} />;
        break;
      case 4:
        heading = <h4 innerHTML={item.text} class='quake-heading' id={item.anchor} />;
        break;
      case 5:
        heading = <h5 innerHTML={item.text} class='quake-heading' id={item.anchor} />;
        break;
      case 6:
        heading = <h6 innerHTML={item.text} class='quake-heading' id={item.anchor} />;
        break;
      default:
        console.log(item);
    }

    return heading;
  }

  private static create_table(item: any) {
    return <table>
      <thead>
      <tr>
        {item.header.map((head) =>
          <th innerHTML={head} />,
        )}
      </tr>
      </thead>
      <tbody>

      {item.rows.map((row) =>
        <tr>
          {row.map((cell) =>
            <td innerHTML={cell} />,
          )}
        </tr>,
      )}
      </tbody>
    </table>;
  }
}
