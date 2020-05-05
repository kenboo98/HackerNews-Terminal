use reqwest::blocking;
use std::io::{Error, ErrorKind};
use serde_json::{Value, Number, Map};
use std::borrow::BorrowMut;

const URI_PREFIX: &str = "https://hacker-news.firebaseio.com/v0/";
const URI_BEST_STORIES: &str = "beststories";
const URI_ITEM: &str = "item/";

pub fn best_stories() -> Result<Vec<String>, Error> {
    let resp = match blocking::get(format!("{}{}.json", URI_PREFIX, URI_BEST_STORIES).as_str()) {
        Ok(resp) => resp,
        Err(e) => return Err(Error::new(ErrorKind::NotConnected, "Could not access HackerNews"))
    };
    let content: Vec<Number> = resp.json().expect("Could not get response text.");
    let items = content.into_iter().map(|val: Number| val.to_string()).collect();
    return Ok(items);

}

pub fn items(ids: &[String]) -> Result<Vec<Map<String, Value>>, Error> {
    let client = blocking::Client::new();
    let mut result: Vec<Map<String, Value>> = vec!();
    for id in ids {
        let resp = match client.get(format!("{}{}{}.json", URI_PREFIX, URI_ITEM, id.as_str()).as_str()).send() {
            Ok(resp) => resp,
            Err(e) => return Err(Error::new(ErrorKind::NotConnected, "Could not access HackerNews"))
        };
        let content: Map<String, Value> = resp.json().expect("Could not get response text.");
        result.push(content);
    }
    Ok(result)

}