import {Component, Event, EventEmitter, h} from '@stencil/core';
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

  @Event({
    eventName: 'saveProps',
    composed: true,
    cancelable: true,
    bubbles: true,
  }) saveProps: EventEmitter;

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
    let props = [];
    let schema = this.formEditor.getSchema();
    for (let component of schema.components) {
      props.push({
        property: component.key,
        type: component.type
      })
    }

    console.log(props);
    this.saveProps.emit(props);
  }

  render() {
    return <div>
      <div class="type-creator_form" ref={(el) => this.formEl = el} />
      <button class="type-creator_submit" onClick={() => this.submit()}>Submit</button>
    </div>;
  }
}
