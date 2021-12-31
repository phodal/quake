import {Component, Host, h, Prop, State} from '@stencil/core';

@Component({
  tag: 'embed-link',
  styleUrl: 'embed-link.css',
  shadow: true,
})
export class EmbedLink {
  @Prop() entryType: string = '';
  @Prop() entryId: number = 0;

  @State() entry: {
    id: number,
    title: string,
    content: string
  } = {
    id: 0,
    title: '',
    content: ''
  }

  async componentWillLoad() {
    try {
      let response = await fetch(`/entry/${this.entryType}/${this.entryId}`)
      this.entry = await response.json();
    } catch (error) {

    }
  }

  render() {
    return (
      <Host>
        # {this.entry.title}
        <quake-render content={this.entry.content} has-embed="true"/>
      </Host>
    );
  }
}
