use indexmap::IndexMap;

pub use entry_define::EntryDefine;
pub use entry_defines::EntryDefines;
pub use entry_node_info::EntryNodeInfo;

pub mod entry_by_path;
pub mod entry_define;
pub mod entry_defines;
pub mod entry_file;
pub mod entry_node_info;
pub mod entry_paths;
pub mod slug;

pub type PropMap = IndexMap<String, String>;
