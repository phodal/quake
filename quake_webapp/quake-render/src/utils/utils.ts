import * as marked from 'marked';

// // @ts-ignore
// const QuakeRenderer: marked.Renderer = {
//   options: undefined,
// };

class QuakeRenderer extends marked.Renderer {
  markdownData: any[] = [];

}

export default QuakeRenderer;
