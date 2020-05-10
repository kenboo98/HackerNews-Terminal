use std::io::{Error, ErrorKind};

use futures::future::join_all;
use reqwest::blocking;
use reqwest::Client;
use serde_json::{Map, Number, Value};
use tokio::runtime::Runtime;

const URI_PREFIX: &str = "https://hacker-news.firebaseio.com/v0/";

const URI_TOP_STORIES: &str = "topstories";
const URI_NEW_STORIES: &str = "newstories";
const URI_BEST_STORIES: &str = "beststories";
const URI_ASK_STORIES: &str = "askstories";
const URI_SHOW_STORIES: &str = "showstories";
const URI_JOB_STORIES: &str = "jobstories";

const URI_ITEM: &str = "item/";

pub enum ListType {
    TopStories,
    NewStories,
    BestStories,
    AskStories,
    ShowStories,
    JobStories,
}

pub fn get_stories(story_type: &ListType) -> Result<Vec<String>, Error> {
    let endpoint = match story_type {
        ListType::TopStories => URI_TOP_STORIES,
        ListType::NewStories => URI_NEW_STORIES,
        ListType::BestStories => URI_BEST_STORIES,
        ListType::AskStories => URI_ASK_STORIES,
        ListType::ShowStories => URI_SHOW_STORIES,
        ListType::JobStories => URI_JOB_STORIES
    };
    let resp = match blocking::get(format!("{}{}.json", URI_PREFIX, endpoint).as_str()) {
        Ok(resp) => resp,
        Err(_) => return Err(Error::new(ErrorKind::NotConnected, "Could not access HackerNews"))
    };
    let content: Vec<Number> = resp.json().expect("Could not get response text.");
    let items = content.into_iter().map(|val: Number| val.to_string()).collect();
    return Ok(items);
}

pub fn get_items(ids: &[String]) -> Result<Vec<Map<String, Value>>, Error> {
    let mut rt = Runtime::new()?;
    let client = Client::new();
    let results = rt.block_on(
        join_all(
            ids
                .iter()
                .map(|id| {
                    client.get(format!("{}{}{}.json", URI_PREFIX, URI_ITEM, id.as_str()).as_str()).send()
                })
        )
    );

    let items: Vec<Map<String, Value>> = results
        .into_iter()
        .map(|result| {
            let resp = result.expect("Could not get response");
            match rt.block_on(resp.json()) {
                Ok(json) => json,
                Err(_) => Map::new()
            }
        }).collect();
    Ok(items)
}

