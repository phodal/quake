import React from 'react';
import {Document, Page, pdfjs} from 'react-pdf';
import 'react-pdf/dist/esm/Page/AnnotationLayer.css';
import styled from "styled-components";

pdfjs.GlobalWorkerOptions.workerSrc = 'pdf.worker.min.js';

export type Props = {
  type: any,
  file: any,
}

function QuakeViewer(props: Props) {
  const [file, setFile] = React.useState(props.file);

  const [numPages, setNumPages] = React.useState(1);
  const [pageNumber, setPageNumber] = React.useState(1);

  React.useEffect(() => {
    setFile(props.file);
  }, [props])

  const onLoadSuccess = React.useCallback(({numPages}) => {
    setNumPages(numPages);
  }, [setNumPages])

  return (
    <div>
      <StyleDocument file={file}
                     renderMode={'canvas'}
                     onLoadSuccess={onLoadSuccess}>
        <Page
          pageNumber={pageNumber}
          renderMode={'canvas'}
          renderAnnotationLayer={true}
          renderInteractiveForms={true}
        />

        <StylePageControls>
          <button
            disabled={pageNumber <= 1}
            onClick={() => setPageNumber(pageNumber - 1)}
            type="button"
            aria-label="Previous page">
            ‹
          </button>
          <span> {pageNumber}{' '}of{' '}{numPages}</span>
          <button
            disabled={pageNumber >= numPages}
            onClick={() => setPageNumber(pageNumber + 1)}
            type="button"
            aria-label="Next page">›
          </button>
        </StylePageControls>
      </StyleDocument>
    </div>
  );
}


export default QuakeViewer;

const StyleDocument = styled(Document)`
  position: relative;

  &:hover {
    .page-controls {
      opacity: 1;
    }
  }
`;

const StylePageControls = styled.div`
  position: absolute;
  bottom: 5%;
  left: 50%;
  background: white;
  //opacity: 0;
  transform: translateX(-50%);
  transition: opacity ease-in-out 0.2s;
  box-shadow: 0 30px 40px 0 rgba(16, 36, 94, 0.2);
  border-radius: 4px;

  span {
    font: inherit;
    font-size: .8em;
    padding: 0 .5em;
  }

  button {
    width: 44px;
    height: 44px;
    background: white;
    border: 0;
    font: inherit;
    font-size: .8em;
    border-radius: 4px;

    &:enabled {
      &:hover {
        cursor: pointer;
      }

      &:hover, &:focus {
        background-color: #e6e6e6;
      }
    }

    &:first-child {
      border-top-right-radius: 0;
      border-bottom-right-radius: 0;
    }

    &:last-child {
      border-top-left-radius: 0;
      border-bottom-left-radius: 0;
    }
  }
`;
