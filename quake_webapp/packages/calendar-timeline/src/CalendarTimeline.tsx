import React from 'react';
import Timeline from 'react-calendar-timeline'
import 'react-calendar-timeline/lib/Timeline.css'
import dayjs from "dayjs";

export type Props = {
  data: any,
}

function CalendarTimeline(_props: Props) {
  const groups = [{ id: 1, title: 'group 1' }, { id: 2, title: 'group 2' }]
  const items = [
    {
      id: 1,
      group: 1,
      title: 'item 1',
      start_time: dayjs().toDate(),
      end_time: dayjs().add(1, 'hour').toDate()
    },
    {
      id: 2,
      group: 2,
      title: 'item 2',
      start_time: dayjs().add(-0.5, 'hour').toDate(),
      end_time: dayjs().add(0.5, 'hour').toDate()
    },
    {
      id: 3,
      group: 1,
      title: 'item 3',
      start_time: dayjs().add(2, 'hour').toDate(),
      end_time: dayjs().add(3, 'hour').toDate()
    }
  ]

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
