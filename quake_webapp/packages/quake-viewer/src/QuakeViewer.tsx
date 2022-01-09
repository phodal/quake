import React from 'react';
import styled from "styled-components";
import {Document, Page} from 'react-pdf/dist/esm/entry.webpack';
import 'react-pdf/dist/esm/Page/AnnotationLayer.css';

export type Props = {
  type: any,
  file: any,
}

function QuakeViewer(props: Props) {
  const [type, setType] = React.useState(props.type);
  const [file, setFile] = React.useState(props.file);

  const [numPages, setNumPages] = React.useState(null);

  React.useEffect(() => {
    setType(props.type);
  }, [props])

  React.useEffect(() => {
    setFile(props.file);
  }, [props])

  // @ts-ignore
  function onLoadSuccess({ numPages }) {
    setNumPages(numPages);
  }

  return (
    <div>
      {type === "pdf" &&
        <StyleDocument file={file} onLoadSuccess={onLoadSuccess}>
          {
            Array.from(
              new Array(numPages),
              (el, index) => (
                <Page
                  key={`page_${index + 1}`}
                  pageNumber={index + 1}
                />
              ),
            )
          }
        </StyleDocument>
      }
    </div>
  );
}

const StyleDocument = styled(Document)`
  height: 100%;
  width: 100%;
`;

export default QuakeViewer;
