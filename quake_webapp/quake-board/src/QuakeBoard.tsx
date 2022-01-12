import React from 'react';

export type Props = {
  title: string,
  onSave: (content: object) => any
}

function QuakeBoard(props: Props) {
  const [title] = React.useState(props.title);

  return (
    <div>
      {title}
    </div>
  );
}

export default QuakeBoard;
