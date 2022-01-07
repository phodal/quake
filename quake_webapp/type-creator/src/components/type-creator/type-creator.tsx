import {Component, h} from '@stencil/core';
import {FormEditor} from '@bpmn-io/form-js-editor';

@Component({
  tag: 'type-creator',
  styleUrl: 'type-creator.css',
  shadow: false,
})
export class TypeCreator {
  formEl: HTMLElement;

  async componentDidRender() {
    const schema = {
      "schemaVersion": 1,
      "exporter": {
        "name": "form-js",
        "version": "0.1.0"
      },
      "type": "default"
    };

    const formEditor = new FormEditor({
      container: this.formEl
    });

    await formEditor.importSchema(schema);
  }

  render() {
    return <div>
      <div ref={(el) => this.formEl = el} />
    </div>;
  }
}
