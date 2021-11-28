import { Component, Prop, h, State, Watch } from '@stencil/core';

@Component({
  tag: 'quake-dashboard',
  styleUrl: 'quake-dashboard.css',
  shadow: true,
})
export class QuakeDashboard {
  @Prop() sets: string;
  @State() internalEntrySets: Array<object> = [];

  componentWillLoad() {
    this.parseEntrySets();
  }

  @Watch('sets')
  parseEntrySets() {
    if (this.sets) {
      this.internalEntrySets = JSON.parse(this.sets);
    }
  }
  render() {
    console.log(this.sets);
    console.log(this.internalEntrySets);
    return <div>
      {this.internalEntrySets.map((todo: any) =>
        <div>
          <div>{todo.title}</div>
          <div>{todo.content}</div>
          <div>{todo.created_date}</div>
          <div>{todo.updated_date}</div>
        </div>,
      )}
    </div>;
  }
}
