import React from 'react';
import Timeline from 'react-calendar-timeline'
import 'react-calendar-timeline/lib/Timeline.css'
import dayjs from "dayjs";

export type Props = {
  entries: {
    items: any[]
  },
  data: any[],
}

function CalendarTimeline(props: Props) {
  let groups: any = [];
  let group_map: any = {};
  let items: any = [];

  if (props.entries && props.entries.items) {
    let index = 1;
    for (let item of props.entries.items) {
      groups.push({
        id: index,
        title: item
      })
      group_map[item] = index;
      index = index + 1;
    }
  }

  if (!!props.data && props.data.length > 0) {
    let index = 1;
    for (let datum of props.data) {
      items.push({
        id: index,
        group: group_map[datum],
        title: datum.title,
        start_time: dayjs(datum.start_time).toDate(),
        end_time: dayjs(datum.end_time).toDate()
      })

      index = index + 1;
    }
  }

  return (
    <div>
      <Timeline
        groups={groups}
        items={items}
        defaultTimeStart={dayjs().add(-7, 'day').toDate()}
        defaultTimeEnd={dayjs().add(7, 'day').toDate()}
      />
    </div>
  );
}

export default CalendarTimeline;
