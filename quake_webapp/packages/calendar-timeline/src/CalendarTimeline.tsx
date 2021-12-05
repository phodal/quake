import React, {useMemo} from 'react';
import Timeline from 'react-calendar-timeline'
import 'react-calendar-timeline/lib/Timeline.css'
import dayjs from "dayjs";

export type Props = {
  entries: any,
  data: any[],
}

function CalendarTimeline(props: Props) {
  let group_map: any = useMemo(() => {
    return {}
  }, []);

  const [entries, setEntries] = React.useState(props.entries);
  const [data, setData] = React.useState(props.data);

  React.useEffect(() => {
    setData(props.data);
  }, [props])

  React.useEffect(() => {
    setEntries(props.entries);
  }, [props])

  const calculateData = useMemo(() => {
    let items: any = [];
    let index = 1;
    for (let item of entries.items) {
      group_map[item] = index;
      index = index + 1;
    }

    if (!!data && data.length > 0) {
      let index = 1;
      for (let datum of data) {
        items.push({
          id: index,
          group: group_map[datum.type],
          title: datum.title,
          start_time: dayjs(datum.start_time).toDate(),
          end_time: dayjs(datum.end_time).toDate()
        })

        index = index + 1;
      }
    }

    return items
  }, [data, entries, group_map])

  const calculateGroup = useMemo(() => {
    let groups: any = [];
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

    return groups;
  }, [entries, group_map]);

  return (
    <div>
      <Timeline
        items={calculateData}
        groups={calculateGroup}
        defaultTimeStart={dayjs().add(-7, 'day').toDate()}
        defaultTimeEnd={dayjs().add(7, 'day').toDate()}
      />
    </div>
  );
}

export default CalendarTimeline;
