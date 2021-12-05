use crate::entry::EntryDefine;
use crate::quake::QuakeTransflowNode;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Transflow {}

impl Transflow {
    pub fn generate(_defines: Vec<EntryDefine>, _node: QuakeTransflowNode) {
        // filter_define
    }
}

#[cfg(test)]
mod tests {
    use crate::entry::EntryDefine;
    use crate::quake::QuakeTransflowNode;
    use crate::transflow::Transflow;

    fn entry_defines() -> Vec<EntryDefine> {
        let yaml = "
- type: todo
  display: Todo
  fields:
    - title: Title
    - content: Body
    - author: Author

- type: blog
  display: Blog
  fields:
    - title: Title
    - content: Body
    - author: Author
";

        let entries: Vec<EntryDefine> = serde_yaml::from_str(yaml).unwrap();
        entries
    }

    #[test]
    fn stringify_defines() {
        let define = "transflow { from('todo','blog').to(<quake-calendar>); }";
        let flow = QuakeTransflowNode::from_text(define).unwrap();

        Transflow::generate(entry_defines(), flow);
    }
}
