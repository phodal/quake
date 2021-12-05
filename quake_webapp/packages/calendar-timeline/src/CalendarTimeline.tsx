import React, {useMemo} from 'react';
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
  let group_map: any = {};

  const [entries, setEntries] = React.useState(props.entries);
  const [data, setData] = React.useState(props.data);

  React.useEffect(() => {
    setData(props.data);
  }, [props])

  React.useEffect(() => {
    setEntries(props.entries);
  }, [props])

  const buildData =  useMemo(() => {
    let items: any = [];
    if (!!data && data.length > 0) {
      let index = 1;
      for (let datum of data) {
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

    return items
  }, [data, group_map])

  const buildGroups = useMemo(() => {
    let groups: any = [];
    console.log(entries);
    if (entries && entries.items) {
      let index = 1;
      for (let item of entries.items) {
        groups.push({
          id: index,
          title: item
        })
        group_map[item] = index;
        index = index + 1;
      }
    }

    console.log(groups);
    return groups;
  }, [entries, group_map]);

  return (
    <div>
      <Timeline
        groups={buildGroups}
        items={buildData}
        defaultTimeStart={dayjs().add(-7, 'day').toDate()}
        defaultTimeEnd={dayjs().add(7, 'day').toDate()}
      />
    </div>
  );
}

export default CalendarTimeline;
