import { marked, Slugger } from 'marked';
import Token = marked.Token;
import TokensList = marked.TokensList;

class QuakeGen {
  content = '';
  token = null;
  tokens: TokensList | any = [];
  markdownData: any[] = [];
  slugger = new Slugger();

  headingIndex = 0;
  headingMap = {};
  indexHeadingMap = {};

  constructor(content: string) {
    this.content = content;
  }

  private next(): Token {
    this.token = this.tokens.pop();
    return this.token;
  }

  // @ts-ignore
  private peek() {
    return this.tokens[this.token.length - 1] || 0;
  }

  gen() {
    const tokens = marked.lexer(this.content);
    this.tokens = tokens.reverse();

    while (this.next()) {
      this.tok();
    }

    return this.markdownData;
  }

  private tok() {
    const token: Token = this.token;
    switch (token.type) {
      case 'heading':
        this.markdownData.push({
          type: 'heading',
          depth: token.depth,
          text: this.renderInline(token.text),
          headingIndex: this.headingIndex,
          anchor: this.slugger.slug(this.unescape(this.renderInline(token.text))),
        });
        break;
      case 'blockquote':
        this.markdownData.push({ type: 'blockquote', text: token.text, raw: token.raw });
        break;
      case 'hr':
        this.markdownData.push({ type: 'hr', raw: token.raw });
        break;
      case 'space':
        this.markdownData.push({ type: 'space', raw: token.raw });
        break;
      case 'paragraph':
        this.markdownData.push({
          type: 'paragraph',
          data: this.renderInline(token.text),
        });
        break;
      default:
        console.log(token);
    }
  }

  private renderInline(tokenText: string) {
    const renderer = new marked.Renderer();
    const linkRenderer = renderer.link;
    renderer.link = (href, title, text) => {
      const html = linkRenderer.call(renderer, href, title, text);
      return html.replace(/^<a /, '<a target="_blank" ');
    };

    return marked.parseInline(tokenText, { renderer });
  }


  private unescape(html) {
    const unescapeTest = /&(#(?:\d+)|(?:#x[0-9A-Fa-f]+)|(?:\w+));?/gi;
    return html.replace(unescapeTest, (_, n) => {
      n = n.toLowerCase();
      if (n === 'colon') {
        return ':';
      }
      if (n.charAt(0) === '#') {
        return n.charAt(1) === 'x'
          ? String.fromCharCode(parseInt(n.substring(2), 16))
          : String.fromCharCode(+n.substring(1));
      }
      return '';
    });
  }

}

export default QuakeGen;
