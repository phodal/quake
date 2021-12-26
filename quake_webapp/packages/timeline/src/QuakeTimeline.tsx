import React from 'react';
import {VerticalTimeline, VerticalTimelineElement} from 'react-vertical-timeline-component';
import 'react-vertical-timeline-component/style.min.css';
import { format } from 'date-fns'

export type Props = {
  entries: any,
  data: any[],
}

function QuakeTimeline(props: Props) {
  const [data, setData] = React.useState(props.data);

  React.useEffect(() => {
    setData(props.data);
  }, [props])

  function formatDate(date: number) {
    let result = '';
    try {
      result = format(date * 1000, 'yyyy-MM-dd');
    } catch (_err) {
    }

    return result;
  }

  return (
    <div style= {{backgroundColor: "#eee"}}>
      <VerticalTimeline>
        { data && data.map((item) =>
          <VerticalTimelineElement
            date={formatDate(item.date)}
            key={item.id}
            iconStyle={{ background: 'rgb(33, 150, 243)', color: '#fff' }}
          >
            <h3 className="vertical-timeline-element-title">{item.title}</h3>
            { item.subtitle && <h4 className="vertical-timeline-element-subtitle">{item.subtitle}</h4> }
            <p>
              {item.content}
            </p>
          </VerticalTimelineElement>
        )}
      </VerticalTimeline>
    </div>
  );
}

export default QuakeTimeline;
