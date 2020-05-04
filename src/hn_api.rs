use reqwest::blocking;
use std::io::{Error, ErrorKind};
use serde_json::{Value, Number};

const URI_PREFIX: &str = "https://hacker-news.firebaseio.com/v0/";
const URI_BEST_STORIES: &str = "beststories.json";


pub fn best_stories() -> Result<Vec<String>, Error> {
    let resp = match blocking::get(format!("{}{}", URI_PREFIX, URI_BEST_STORIES).as_str()) {
        Ok(resp) => resp,
        Err(e) => return Err(Error::new(ErrorKind::NotConnected, "Could not access HackerNews"))
    };
    let content: Vec<Number> = resp.json().expect("Could not get response text.");
    let items = content.into_iter().map(|val: Number| val.to_string()).collect();
    return Ok(items);

}
