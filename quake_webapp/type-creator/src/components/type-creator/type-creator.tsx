import {Component, h} from '@stencil/core';
import {FormEditor} from '@quakeworks/form-js-editor';

@Component({
  tag: 'type-creator',
  styleUrl: 'type-creator.css',
  shadow: false,
})
export class TypeCreator {
  formEl: HTMLElement;
  formEditor: FormEditor
  schema: any

  async componentDidRender() {
    this.schema = {
      "schemaVersion": 1,
      "exporter": {
        "name": "form-js",
        "version": "0.1.0"
      },
      "components": [
        {
          "key": "title",
          "label": "Title",
          "type": "String"
        },
        {
          "key": "content",
          "label": "Content",
          "type": "Content"
        },
        {
          "key": "update_date",
          "label": "Update Date",
          "type": "Date"
        },
        {
          "key": "create_date",
          "label": "Create Date",
          "type": "Date"
        }
      ],
      "type": "default"
    };

    this.formEditor = new FormEditor({
      container: this.formEl
    });

    await this.formEditor.importSchema(this.schema);
  }

  submit() {
    console.log(this.schema);
    console.log(this.formEditor.getSchema());
  }

  render() {
    return <div>
      <button onClick={() => this.submit()}>Submit</button>
      <div ref={(el) => this.formEl = el} />
    </div>;
  }
}
