import * as marked from 'marked';

// @ts-ignore
const QuakeRenderer: marked.Renderer = {
  options: undefined, blockquote(quote: string): string {
    return '';
  }, br(): string {
    return '';
  }, checkbox(checked: boolean): string {
    return '';
  }, code(code: string, language: string | undefined, isEscaped: boolean): string {
    return '';
  }, codespan(code: string): string {
    return '';
  }, del(text: string): string {
    return '';
  }, em(text: string): string {
    return '';
  }, heading(text: string, level: 1 | 2 | 3 | 4 | 5 | 6, raw: string, slugger: marked.Slugger): string {
    return '';
  }, hr(): string {
    return '';
  }, html(html: string): string {
    return '';
  }, image(href: string | null, title: string | null, text: string): string {
    return '';
  }, link(href: string | null, title: string | null, text: string): string {
    return '';
  }, list(body: string, ordered: boolean, start: number): string {
    return '';
  }, listitem(text: string, task: boolean, checked: boolean): string {
    return '';
  }, paragraph(text: string): string {
    return '';
  }, strong(text: string): string {
    return '';
  }, table(header: string, body: string): string {
    return '';
  }, tablecell(content: string, flags: { header: boolean; align: 'center' | 'left' | 'right' | null }): string {
    return '';
  }, tablerow(content: string): string {
    return '';
  }, text(text: string): string {
    return '';
  },

};

// @ts-ignore
class Sample extends marked.Renderer {

}

export default QuakeRenderer;
