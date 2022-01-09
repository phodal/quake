import React from 'react';
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
        <Document file={file}
                  onLoadSuccess={onLoadSuccess}>
          {
            Array.from(
              new Array(numPages),
              (el, index) => (
                <Page
                  key={`page_${index + 1}`}
                  pageNumber={index + 1}
                  renderAnnotationLayer={true}
                  renderInteractiveForms={true}
                />
              ),
            )
          }
        </Document>
      }
    </div>
  );
}


export default QuakeViewer;
