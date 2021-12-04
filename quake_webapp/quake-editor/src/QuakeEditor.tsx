import {lighten} from "polished";
import React from 'react';
import Editor from "rich-markdown-editor";
import styled from "styled-components";
import './QuakeEditor.css';

export type Props = {
  id: number,
  title: string,
  value: string,
  onSave: (content: object) => any
}

function QuakeEditor(props: Props) {
  const [title, setTitle] = React.useState(props.title);
  const [value, setValue] = React.useState(props.value);

  React.useEffect(() => {
    setTitle(props.title);
  }, [props.title])


  React.useEffect(() => {
    setValue(props.value);
  }, [props.value])


  const onSave = React.useCallback(() => {
    props.onSave({
      title: props.title,
      value: props.value,
    });
  }, [props])

  const onChange = React.useCallback((getValue) => {
    setValue(getValue())
    console.log(value);
  }, [props]);

  return (
    <div className="App">
      <StyleLabel># {props.id}</StyleLabel>
      <StyleInput type="text" value={title} onChange={(e) => { setTitle(e.target.value)}}/>
      <StyledEditor
        autoFocus={true}
        value={props.value}
        onChange={onChange}
        onSave={onSave}
      />
    </div>
  );
}

export default QuakeEditor;

const StyleInput = styled.input`
  color: palevioletred;
  font-size: 1em;
  border: 2px solid palevioletred;
  border-radius: 4px;
  margin: 1em;
  padding: 1em;
  width: 80%;
`;

const StyleLabel = styled.label`

`;

const StyledEditor = styled(Editor)`
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
