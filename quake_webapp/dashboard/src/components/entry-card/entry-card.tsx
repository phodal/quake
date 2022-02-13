import {Component, Event, h, Prop, EventEmitter} from '@stencil/core';
import PadLeft from "../utils/PadLeft";
import DateFormat from "../utils/DateFormat";

@Component({
  tag: 'entry-card',
  styleUrl: 'entry-card.css',
  shadow: false,
})
export class EntryCard {
  @Prop() item: any;
  @Prop() type: string;
  @Prop() fileProp: string;

  @Event() triggerShow: EventEmitter<string>;
  @Event() triggerEdit: EventEmitter<string>;

  showEntry(id: string) {
    this.triggerShow.emit(id);
  }

  editEntry(id: string) {
    this.triggerEdit.emit(id);
  }

  async showPdf(entry: string, fileProp: string) {
    let url = `/processor/${entry}?file_prop=${fileProp}`;
    const modal: any = document.createElement('ion-modal');

    const viewer: any = document.createElement('quake-viewer');
    viewer.setAttribute('url', url);

    modal.appendChild(viewer);
    document.body.appendChild(modal);

    await modal.present();
  }

  render() {
    return (
      <div class="entry-show-list">
        <ion-card>
          <ion-card-header>
            <ion-card-subtitle># {PadLeft(this.item.id, 4, '')}
              {this.fileProp &&
                <ion-icon name="document-outline" onClick={() => this.showPdf(this.type, this.item[this.fileProp])}/>
              }
              <ion-icon name="book-outline" onClick={() => this.showEntry(this.item.id)}/>
              <ion-icon name="create-outline" onClick={() => this.editEntry(this.item.id)}/>
            </ion-card-subtitle>
            <ion-card-title>{this.item.title}</ion-card-title>
          </ion-card-header>
          <ion-card-content>
            {this.item.description && <p>{this.item.description}</p>}
            <ion-badge slot="start">{DateFormat(this.item.created_date)}</ion-badge>
          </ion-card-content>
        </ion-card>
      </div>
    );
  }

}
