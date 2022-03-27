use std::error::Error;

use async_std::task;
use rocket::tokio;
use tracing::{error, info};

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::EntryDefine;

use crate::helper::search_config::define_to_settings;

pub fn feed_documents(
    server: &str,
    index_name: &str,
    content: &[EntryFile],
) -> Result<(), Box<dyn Error>> {
    let url = format!("{:}/indexes/{:}/documents", server, index_name);

    info!("try feed to {:?} with {:?} len", url, content.len());
    task::block_on(async {
        let client = reqwest::Client::new();
        let req = client.post(url).json(content).send();
        let response = req.await.unwrap().text().await.unwrap();

        info!("{:?}", response);
    });

    Ok(())
}

pub fn feed_settings(server: &str, define: &EntryDefine) -> Result<(), Box<dyn Error>> {
    let value = &define_to_settings(define);
    let url = format!("{:}/indexes/{:}/settings", server, &define.entry_type);

    task::block_on(async {
        let client = reqwest::Client::new();
        let req = client.post(url).json(&value).send();
        let response = req.await.unwrap().text().await.unwrap();

        info!("{:?}", response);
    });

    Ok(())
}

pub fn feed_document(
    server: &str,
    index_name: &str,
    file: &EntryFile,
) -> Result<(), Box<dyn Error>> {
    let format_url = format!("{:}/indexes/{:}/documents", server, index_name);

    task::block_on(async {
        let client = reqwest::Client::new();
        let req = client.post(format_url).json(&file).send();
        let response = req.await.unwrap().text().await.unwrap();

        info!("{:?}", response);
    });

    Ok(())
}

pub fn feed_document_async(
    server: &str,
    index_name: &str,
    file: &EntryFile,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{:}/indexes/{:}/documents", server, index_name);

    let content = file.clone();

    tokio::spawn(async move {
        info!("feed async: {:}", &url);
        let cloned_url = url.clone();
        let client = reqwest::Client::new();
        match async move {
            let res = client.post(url).json(&content).send().await?.text().await?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res)
        }
        .await
        {
            Ok(res) => info!("feed to {:?} with response: {:?}", &cloned_url, res),
            Err(er) => error!("feed error to {:?} with error: {}", &cloned_url, er),
        };
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use quake_core::entry::EntryDefine;

    use crate::helper::exec_wrapper::meili_exec::feed_settings;

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
        entries[0].clone()
    }

    #[test]
    #[ignore]
    fn test_feed_settings() {
        let define = &get_define();
        let _ = feed_settings("http://127.0.0.1:7700", define);
    }
}
