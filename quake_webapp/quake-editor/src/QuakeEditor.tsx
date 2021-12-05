import {lighten} from "polished";
import React from 'react';
import Editor from "rich-markdown-editor";
import styled from "styled-components";

export type Props = {
  id: number,
  title: string,
  value: string,
  onSave: (content: object) => any
}

function QuakeEditor(props: Props) {
  const [title, setTitle] = React.useState(props.title);
  const [value, setValue] = React.useState(props.value);

  const pattern = /[a-zA-Z0-9_\u00A0-\u02AF\u0392-\u03c9\u0410-\u04F9]+|[\u4E00-\u9FFF\u3400-\u4dbf\uf900-\ufaff\u3040-\u309f\uac00-\ud7af]+/g;
  const wordCount = (data: string) => {
    if(!data  || (!!data && data.length < 0)) {
      return 0;
    }

    const m = data.match(pattern);
    let count = 0;
    if (m === null) return count;
    for (let i = 0; i < m.length; i++) {
      if (m[i].charCodeAt(0) >= 0x4E00) {
        count += m[i].length;
      } else {
        count += 1;
      }
    }
    return count;
  }

  React.useEffect(() => {
    setTitle(props.title);
  }, [props])


  React.useEffect(() => {
    setValue(props.value);
  }, [props])


  const onSave = React.useCallback(() => {
    props.onSave({
      title: title,
      value: value,
    });
  }, [props, title, value])

  const onChange = React.useCallback((getValue) => {
    setValue(getValue())
  }, [setValue]);

  const saveEntry = () => {
    onSave();
  }

  return (
    <div>
      <StyledTitle>
        <StyleLabel># {props.id}</StyleLabel>
        <StyleInput type="text" value={title} onChange={(e) => { setTitle(e.target.value)}}/>
        <StyleButton onClick={saveEntry}>Save</StyleButton>
        <StyleCount>words：{wordCount(value)}</StyleCount>
      </StyledTitle>
      <StyledEditor
        autoFocus={true}
        defaultValue={props.value}
        onChange={onChange}
        onSave={onSave}
      />
    </div>
  );
}

export default QuakeEditor;

const StyleCount = styled.span`
  margin-left: 1em;
`

const StyleButton = styled.button`
  border: 2px solid royalblue;
  background: royalblue;
  border-radius: 4px;
  padding: 1em;
  color: #fff;
`;

const StyledTitle = styled.div`
  padding: 0 40px;
`;

const StyleInput = styled.input`
  color: royalblue;
  font-size: 1em;
  border: 2px solid royalblue;
  border-radius: 4px;
  margin: 1em;
  padding: 1em;
  width: 60%;
`;

const StyleLabel = styled.label`

`;

const StyledEditor = styled(Editor)`
  padding: 0 60px;
  flex-grow: 1;
  justify-content: start;

  > div {
    background: transparent;
  }

  & * {
    box-sizing: content-box;
  }

  .notice-block.tip,
  .notice-block.warning {
    font-weight: 500;
  }

  .heading-anchor {
    box-sizing: border-box;
  }

  .heading-name {
    pointer-events: none;
    display: block;
    position: relative;
    top: -60px;
    visibility: hidden;
  }

  .heading-name:first-child,
  .heading-name:first-child + .ProseMirror-yjs-cursor {
    & + h1,
    & + h2,
    & + h3,
    & + h4 {
      margin-top: 0;
    }
  }

  p {
    a {
      color: #111319;
      border-bottom: 1px solid ${lighten(0.5, "#111319")};
      text-decoration: none !important;
      font-weight: 500;

      &:hover {
        border-bottom: 1px solid #111319;
        text-decoration: none;
      }
    }
  }

  .ProseMirror {
    & > .ProseMirror-yjs-cursor {
      display: none;
    }

    .ProseMirror-yjs-cursor {
      position: relative;
      margin-left: -1px;
      margin-right: -1px;
      border-left: 1px solid black;
      border-right: 1px solid black;
      height: 1em;
      word-break: normal;

      &:after {
        content: "";
        display: block;
        position: absolute;
        left: -8px;
        right: -8px;
        top: 0;
        bottom: 0;
      }

      > div {
        opacity: 0;
        transition: opacity 100ms ease-in-out;
        position: absolute;
        top: -1.8em;
        font-size: 13px;
        background-color: rgb(250, 129, 0);
        font-style: normal;
        line-height: normal;
        user-select: none;
        white-space: nowrap;
        color: white;
        padding: 2px 6px;
        font-weight: 500;
        border-radius: 4px;
        pointer-events: none;
        left: -1px;
      }

      &:hover {
        > div {
          opacity: 1;
        }
      }
    }
  }

  &.show-cursor-names .ProseMirror-yjs-cursor > div {
    opacity: 1;
  }
`;
