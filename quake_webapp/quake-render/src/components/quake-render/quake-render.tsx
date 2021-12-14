import {Component, Event, EventEmitter, h, Prop, State} from '@stencil/core';
import QuakeDown from '../../utils/quake-down';

export interface Link {
  entry_type: String,
  type: String,
  id: number
}

@Component({
  tag: 'quake-render',
  styleUrl: 'quake-render.css',
  shadow: true,
})
export class QuakeRender {
  @Prop() content: string = '';
  @Prop() hasEmbed: boolean = false;

  @State() markdownData: any[] = [];
  el: HTMLElement;

  @Event({
    eventName: 'clickPageLink',
    composed: true,
    cancelable: true,
    bubbles: true,
  }) clickPageLink: EventEmitter<Link>;

  @Event({
    eventName: 'clickEmbedLink',
    composed: true,
    cancelable: true,
    bubbles: true,
  }) clickEmbedLink: EventEmitter<Link>;

  componentWillRender() {
    this.markdownData = new QuakeDown(this.content, this.parseInline).gen();
  }

  componentDidRender() {
    let pageLinks = this.el.querySelectorAll('.quake-page-link');
    // @ts-ignore
    for (let elem of pageLinks) {
      elem.addEventListener('click', _e => {
        let data: Link = {
          type: 'page-link',
          entry_type: elem.dataset.type,
          id: Number(elem.dataset.id)
        };
        this.clickPageLink.emit(data)
      });
    }

    // let embedLinks = this.el.querySelectorAll('.quake-embed-link');
    // // @ts-ignore
    // for (let elem of embedLinks) {
    //   elem.addEventListener('click', _e => {
    //     let data: Link = {
    //       type: 'embed-link',
    //       entry_type: elem.dataset.type,
    //       id: Number(elem.dataset.id)
    //     };
    //
    //     this.clickEmbedLink.emit(data)
    //   });
    // }
  }

  render() {
    return <div ref={(el) => this.el = el as HTMLElement}>
      {this.markdownData.map((item: any) =>
        this.conditionRender(item),
      )}
    </div>;
  }

  private conditionRender(item: any) {
    let out: string;
    switch (item.type) {
      case 'heading':
        out = this.renderHeading(item);
        break;
      case 'blockquote':
        out = <blockquote innerHTML={item.text}/>;
        break;
      case 'hr':
        out = <hr/>;
        break;
      case 'paragraph':
        out = <p innerHTML={item.text}/>;
        break;
      case 'space':
        break;
      case 'table':
        out = this.renderTable(item);
        break;
      case 'list':
        out = this.renderList(item);
        break;
      case 'admonition':
        out = <div class={'admonition is-' + item.display_type}>
          <div class='admonition-header'>{item.title}</div>
          <div class='admonition-body'>{item.data.map((sub) =>
            this.conditionRender(sub)
          )}</div>
        </div>;
        break;
      case 'code':
        out = <pre class={'language-' + item.lang}><code class={'language-' + item.lang} innerHTML={item.text}/></pre>
        break;
      default:
        // console.log(item);
        out = <span/>;
    }

    return out;
  }

  private renderHeading(item: any) {
    let heading: string;
    switch (item.depth) {
      case 1:
        heading = <h1 innerHTML={item.text} class='quake-heading' id={item.anchor}/>;
        break;
      case 2:
        heading = <h2 innerHTML={item.text} class='quake-heading' id={item.anchor}/>;
        break;
      case 3:
        heading = <h3 innerHTML={item.text} class='quake-heading' id={item.anchor}/>;
        break;
      case 4:
        heading = <h4 innerHTML={item.text} class='quake-heading' id={item.anchor}/>;
        break;
      case 5:
        heading = <h5 innerHTML={item.text} class='quake-heading' id={item.anchor}/>;
        break;
      case 6:
        heading = <h6 innerHTML={item.text} class='quake-heading' id={item.anchor}/>;
        break;
      default:
        console.log(item);
    }

    return heading;
  }

  private renderTable(item: any) {
    return <table>
      <thead>
      <tr>
        {item.header.map((head) =>
          <th innerHTML={head}/>,
        )}
      </tr>
      </thead>
      <tbody>
      {item.rows.map((row) =>
        <tr>
          {row.map((cell) =>
            <td innerHTML={cell}/>,
          )}
        </tr>,
      )}
      </tbody>
    </table>;
  }

  private renderList(list: any) {
    if (list.ordered) {
      return <ol start={list.start}>
        {list.children.map((item) =>
          <li>
            {item.task && <input type='checkbox' checked={item.checked}/>}
            <span innerHTML={item.text}/>
            {item.children.length > 0 && item.children[0].type == 'list' &&
              this.renderList(item.children[0])
            }
          </li>,
        )}
      </ol>;
    } else {
      return <ul>
        {list.children.map((item) =>
          <li>
            {item.task && <input type='checkbox' checked={item.checked}/>}
            <span innerHTML={item.text}/>
            {item.children.length > 0 && item.children[0].type == 'list' &&
              this.renderList(item.children[0])
            }
          </li>,
        )}
      </ul>;
    }
  }

  parseInline(tokens, renderer) {
    let out = '',
      i,
      token;

    const l = tokens.length;
    for (i = 0; i < l; i++) {
      token = tokens[i];

      switch (token.type) {
        case 'escape': {
          out += renderer.text(token.text);
          break;
        }
        case 'html': {
          out += renderer.html(token.text);
          break;
        }
        case 'link': {
          out += renderer.link(token.href, token.title, this.parseInline(token.tokens, renderer));
          break;
        }
        case 'image': {
          out += renderer.image(token.href, token.title, token.text);
          break;
        }
        case 'strong': {
          out += renderer.strong(this.parseInline(token.tokens, renderer));
          break;
        }
        case 'em': {
          out += renderer.em(this.parseInline(token.tokens, renderer));
          break;
        }
        case 'codespan': {
          out += renderer.codespan(token.text);
          break;
        }
        case 'br': {
          out += renderer.br();
          break;
        }
        case 'del': {
          out += renderer.del(this.parseInline(token.tokens, renderer));
          break;
        }
        case 'text': {
          out += renderer.text(token.text);
          break;
        }
        case 'page_link': {
          out += `<span class='quake-page-link' data-type=${token.entry_type} data-id=${token.entry_id}>${token.entry_title}</span>`;
          break;
        }
        case 'embed_link': {
          out += `<div class='quake-embed-link' data-type='${token.entry_type}' data-id='${token.entry_id}'>
    <embed-link entry-type='${token.entry_type}' entry-id='${token.entry_id}'></embed-link>
</div>`;
          break;
        }
        default: {
          const errMsg = 'Token with "' + token.type + '" type was not found.';
          console.error(errMsg);
        }
      }
    }
    return out;
  }
}
