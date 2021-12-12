import React from 'react';
import { VerticalTimeline, VerticalTimelineElement } from 'react-vertical-timeline-component';
import 'react-vertical-timeline-component/style.min.css';

export type Props = {
  entries: any,
  data: any[],
}

function QuakeTimeline(props: Props) {
  const [data, setData] = React.useState(props.data);

  React.useEffect(() => {
    setData(props.data);
  }, [props])

  return (
    <div style= {{backgroundColor: "#eee"}}>
      <VerticalTimeline>
        { data && data.map((item) =>
          <VerticalTimelineElement
            date={item.date}
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
