use std::error::Error;
use std::io::Read;

use tracing::info;

use quake_core::entry::EntryDefine;

use crate::helper::exec_wrapper::exec_runner;
use crate::helper::search_config::define_to_settings;

pub fn feed_command(index_name: &str, search_url: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{:}/indexes/{:}/documents", search_url, index_name);
    let cmd_line = format!(
        "curl -i -X POST '{:}' \
  --header 'content-type: application/json' \
  --data-binary @dump.json",
        url
    );

    info!("{:?}", cmd_line);
    exec_runner::cmd_runner(cmd_line)?;

    Ok(())
}

pub fn feed_settings(search_url: &str, define: &EntryDefine) -> Result<(), Box<dyn Error>> {
    let config = serde_json::to_string(&define_to_settings(define)).unwrap();
    let url = format!("{:}/indexes/{:}/settings", search_url, &define.entry_type);
    let cmd_line = format!(
        "curl -i -X POST '{:}' \
  --header 'content-type: application/json' \
  -d {:?}",
        url, config
    );

    info!("{:?}", cmd_line);
    exec_runner::cmd_runner(cmd_line)?;

    Ok(())
}

// pub fn feed_settings(search_url: &str, define: &EntryDefine) -> Result<(), Box<dyn Error>> {
//     let value = &define_to_settings(define);
//     let url = format!("{:}/indexes/{:}/settings", search_url, &define.entry_type);
//
//     task::block_on(async {
//         let client = reqwest::Client::new();
//         let req = client.post(url).json(&value).send();
//         let response = req.await.unwrap().text().await.unwrap();
//
//         info!("{:?}", response);
//     });
//
//     Ok(())
// }

#[allow(dead_code)]
pub fn feed_settings_reqwest(search_url: &str, define: &EntryDefine) -> Result<(), Box<dyn Error>> {
    let config = serde_json::to_string(&define_to_settings(define)).unwrap();
    let url = format!("{:}/indexes/{:}/settings", search_url, &define.entry_type);

    let client = reqwest::blocking::Client::new();
    let mut response = client.post(url).body(config).send().expect("");

    let mut buf: Vec<u8> = vec![];
    response.read_to_end(&mut buf).expect("");

    info!("{:?}", response);

    Ok(())
}

pub fn feed_entry(index_name: &str, content: &str, search_url: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{:}/indexes/{:}/documents", search_url, index_name);
    let cmd_line = format!(
        "curl -i -X POST '{:}' \
  --header 'content-type: application/json' \
  --data-binary {:?}",
        url, content
    );

    info!("{:?}", cmd_line);
    exec_runner::cmd_runner(cmd_line)?;

    Ok(())
}

// todo: add sort by date

#[cfg(test)]
mod tests {
    use quake_core::entry::EntryDefine;

    use crate::helper::exec_wrapper::meili_exec::feed_settings_reqwest;

    fn get_define() -> EntryDefine {
        let yaml = "
- type: story
  display: Story
  fields:
    - title: Title
    - author: Filterable
    - description: Searchable
    - content: Text
    - status: Flow
    - priority: Flow
    - created_date: Date
    - updated_date: Date
";
        let entries: Vec<EntryDefine> = serde_yaml::from_str(yaml).unwrap();
        let define = entries[0].clone();
        define
    }

    #[test]
    #[ignore]
    fn test_feed_settings() {
        let define = &get_define();
        let _ = feed_settings_reqwest(&"http://127.0.0.1:7700".to_string(), define);
    }
}
