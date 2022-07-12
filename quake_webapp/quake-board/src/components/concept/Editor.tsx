import * as React from "react";
import styled from "styled-components";
import { useCallback, useState } from "react";

export interface EditorProps {
  onChange: (value: string) => void;
}

const Editor = (props: EditorProps) => {
  const [value, setValue] = useState("text");
  const [editable, setEditable] = useState(false);

  const onChange = useCallback((event) => {
    setValue(event.target.value)
  }, [setValue]);

  const toggleEditable = useCallback((enable: boolean) => {
    setEditable(enable)
  }, [setEditable]);

  return <StyleInputBox
    value={ value }
    onChange={ onChange }
    onKeyDown={ (event) => {
      if (event.key === 'Enter' || event.key === 'Escape') {
        toggleEditable(true)
        event.preventDefault()
        event.stopPropagation()
      }
    } }
  />;
}

const StyleInputBox = styled.textarea`
  background: transparent;
  width: 100%;
  border: none;
`

export default Editor;
