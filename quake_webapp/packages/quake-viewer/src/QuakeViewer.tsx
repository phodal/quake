import React from 'react';

export type Props = {
  type: any,
  data: any,
}

function QuakeViewer(props: Props) {
  const [type] = React.useState(props.type);
  const [data, setData] = React.useState(props.data);

  React.useEffect(() => {
    setData(props.data);
  }, [props])

  return (
    <div >
      <h1>hello, world!</h1>
      <h2>{data}</h2>
      { type === "pdf" &&
        <h2>PDF</h2>
      }
    </div>
  );
}

export default QuakeViewer;
