use std::fs::File;
use std::io::Error;
use std::io::{Read, Write};

use serde_json::Result as SerdeResult;
use wasm_blog::services::types::Post;
use std::path::PathBuf;

/// Step 1: Get all post paths.
fn get_and_parse_paths() -> Result<Vec<Post>, Error> {
    let paths = std::fs::read_dir("./posts")?
        .into_iter()
        .map(|x| x.expect("bad path").path())
        .filter(|path| path.display().to_string().contains(".md"))
        .filter_map(|path| create_metadata(path).ok())
        .collect::<Vec<Post>>();

    Ok(paths)
}

fn parse_yaml_key(str: &str, key: &str) -> Option<String> {
    let line = str
        .lines()
        .filter(|line| line.starts_with(key))
        .collect::<String>();
    if line.is_empty() {
        return None;
    }
    Some(String::from(line.split(key).last().unwrap().trim()))
}

/// Step 2: Parse YAML as Post
fn create_metadata(path: PathBuf) -> Result<Post, Error> {
    let mut file = File::open(path.clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut header = String::new();
    // Just parse the first 6 lines of metadata
    for line in contents.lines().take(6) {
        if line != "---" {
            header.push_str(&(line.to_owned() + "\n"));
        }
    }
    Ok(Post {
        title: parse_yaml_key(&header, "title:").unwrap_or_default(),
        author: parse_yaml_key(&header, "author:").unwrap_or_default(),
        date: parse_yaml_key(&header, "date:").unwrap_or_default(),
        peek: parse_yaml_key(&header, "desc:").unwrap_or_default(),
        id: parse_yaml_key(&header, "id:").unwrap_or_default(),
        md_name: path.display().to_string(),
    })
}

/// Step 3: JSON and save.
fn save(posts: Vec<Post>) -> SerdeResult<()> {
    let j = serde_json::to_string(&posts)?;
    let mut file = File::create("./posts/metadata.json").expect("failed");
    file.write_all(j.as_bytes()).expect("cannot write");
    Ok(())
}

fn main() -> Result<(), Error> {
    let posts = get_and_parse_paths()?;
    save(posts)?;
    Ok(())
}
