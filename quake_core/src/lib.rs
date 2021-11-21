extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod concept_parser;
pub mod model;
pub mod quake_config;


#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryBlog {

}
