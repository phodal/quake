import React, {useMemo} from 'react';
// @ts-ignore
import { Calendar, dateFnsLocalizer } from 'react-big-calendar'
import format from 'date-fns/format'
import parse from 'date-fns/parse'
import startOfWeek from 'date-fns/startOfWeek'
import getDay from 'date-fns/getDay'
import zhCN from 'date-fns/locale/zh-CN'
import 'react-big-calendar/lib/css/react-big-calendar.css'

export type Props = {
  entries: any,
  data: any[],
}

function QuakeCalendar(props: Props) {
  const [data, setData] = React.useState(props.data);

  const locales = {
    'zhCN': zhCN,
  }

  const localizer = dateFnsLocalizer({
    format,
    parse,
    startOfWeek,
    getDay,
    locales,
  })

  React.useEffect(() => {
    setData(props.data);
  }, [props])


  const calculateData = useMemo(() => {
    let items: any = [];

    if (!!data && data.length > 0) {
      for (let datum of data) {
        items.push({
          id: datum.id,
          title: datum.title,
          start: new Date(datum.start),
          end: new Date(datum.end)
        })
      }
    }

    return items
  }, [data])

  return (
    <div>
      <Calendar
        // @ts-ignore
        localizer={localizer}
        events={calculateData}
        startAccessor="start"
        endAccessor="end"
        style={{ height: 500 }}
      />
    </div>
  );
}

export default QuakeCalendar;
