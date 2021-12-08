import React from 'react';
// @ts-ignore
import { Calendar, dateFnsLocalizer } from 'react-big-calendar'
import format from 'date-fns/format'
import parse from 'date-fns/parse'
import startOfWeek from 'date-fns/startOfWeek'
import getDay from 'date-fns/getDay'
import zhCN from 'date-fns/locale/zh-CN'

export type Props = {
  entries: any,
  data: any[],
}

function Calendar(props: Props) {
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

  return (
    <div>
      <Calendar
        // @ts-ignore
        localizer={localizer}
        events={data}
        startAccessor="start"
        endAccessor="end"
        style={{ height: 500 }}
      />
    </div>
  );
}

export default Calendar;
