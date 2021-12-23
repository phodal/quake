use std::collections::{BTreeMap, BTreeSet};

use serde_derive::{Deserialize, Serialize};

use quake_core::entry::EntryDefine;
use quake_core::meta::MetaProperty;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsUpdate {
    pub ranking_rules: Option<Vec<String>>,
    pub distinct_attribute: Option<String>,
    pub searchable_attributes: Option<Vec<String>>,
    pub displayed_attributes: Option<BTreeSet<String>>,
    pub stop_words: Option<BTreeSet<String>>,
    pub synonyms: Option<BTreeMap<String, Vec<String>>>,
    pub filterable_attributes: Option<Vec<String>>,
    pub sortable_attributes: Option<Vec<String>>,
}

pub fn define_to_settings(define: &EntryDefine) -> SettingsUpdate {
    let rules = str_to_string(vec![
        "words",
        "typo",
        "proximity",
        "attribute",
        "sort",
        "exactness",
        "created_date:desc",
    ]);

    let mut searchable = str_to_string(vec!["title", "content"]);
    let mut filterable = str_to_string(vec!["title", "content", "created_date", "updated_date"]);

    for (key, value) in define.to_field_type() {
        match value {
            MetaProperty::Searchable(_) => {
                searchable.push(key);
            }
            MetaProperty::Filterable(_) => {
                filterable.push(key);
            }
            _ => {}
        }
    }

    SettingsUpdate {
        ranking_rules: Some(rules),
        distinct_attribute: None,
        searchable_attributes: Some(searchable),
        displayed_attributes: None,
        stop_words: None,
        synonyms: None,
        filterable_attributes: Some(filterable),
        sortable_attributes: None,
    }
}

fn str_to_string(list: Vec<&str>) -> Vec<String> {
    list.iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use quake_core::entry::EntryDefine;

    use crate::helper::search_config::define_to_settings;

    fn get_define() -> EntryDefine {
        let yaml = "
- type: story
  display: Story
  properties:
    - title: Title
    - author: Filterable
    - description: Searchable
    - content: Text
    - status: Flow
    - priority: Flow
    - created_date: Date
    - updated_date: Date
  actions: ~
  flows:
    - property: status
      items: ['Todo', 'Doing', 'Done']
  states:
    - property: priority
      items: ['Low', 'Medium', 'High']

";
        let entries: Vec<EntryDefine> = serde_yaml::from_str(yaml).unwrap();
        entries[0].clone()
    }

    #[test]
    fn search_config_for_search() {
        let settings = define_to_settings(&get_define());

        assert_eq!(3, settings.searchable_attributes.unwrap().len());
        assert_eq!(5, settings.filterable_attributes.unwrap().len());
    }
}
