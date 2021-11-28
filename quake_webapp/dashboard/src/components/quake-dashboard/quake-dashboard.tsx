import {Component, Prop, h} from '@stencil/core';

@Component({
  tag: 'quake-dashboard',
  styleUrl: 'quake-dashboard.css',
  shadow: true,
})
export class QuakeDashboard {
  @Prop() sets: Array<object> = [];

  render() {
    console.log(this.sets, this.sets.length)
    return <div>
      {this.sets.length > 0 ? this.sets.map((todo: any) =>
          <div>
            <div>{todo.title}</div>
            <div>{todo.content}</div>
            <div>{todo.created_date}</div>
            <div>{todo.updated_date}</div>
          </div>,
        )
        : <div></div>
      }
    </div>;
  }
}
