extern crate serde;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate serde_derive;

pub mod concept_parser;
pub mod model;
pub mod quake_config;
pub mod parser;
pub mod ast;

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryBlog {

}
