import React from 'react';
import {Document, Page, pdfjs} from "react-pdf";
import styled from "styled-components";
// issues from: https://github.com/wojtekmaj/react-pdf/issues/97
pdfjs.GlobalWorkerOptions.workerSrc = `//cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjs.version}/pdf.worker.js`;

export type Props = {
  type: any,
  file: any,
}

function QuakeViewer(props: Props) {
  const [type, setType] = React.useState(props.type);
  const [file, setFile] = React.useState(props.file);

  const [numPages, setNumPages] = React.useState(null);
  const [pageNumber, setPageNumber] = React.useState(1);

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
          <Page pageNumber={pageNumber} />
        </StyleDocument>
      }
    </div>
  );
}

const StyleDocument = styled(Document)`
  height: 100%;
  width: 100%;
  min-height: 500px;
`;

export default QuakeViewer;
