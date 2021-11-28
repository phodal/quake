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
          <ion-card>
            <ion-card-header>
              <ion-card-title>{todo.title}</ion-card-title>
            </ion-card-header>
            <ion-card-content>
              {todo.content}

              <span>{todo.created_date}</span>
              <span>{todo.updated_date}</span>
            </ion-card-content>
          </ion-card>,
        )
        : <div></div>
      }
    </div>;
  }
}
