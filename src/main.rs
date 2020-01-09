use std::fs;
use serde_json::{Value};
use reqwest::Error;

#[tokio::main]
async fn fetch_json() -> Result<Value, Error> {
    let response = reqwest::get("https://json-ld.org/contexts/person.jsonld")
        .await?
        .text()
        .await?;
    println!("{:?}", response);

    let data: Value = serde_json::from_str(&response)
        .expect("error parsing json");
    println!("{:?}", data);

    Ok(data)
}

#[derive(Debug)]
struct Graph {
    graph: Vec<Triple>
};

#[derive(Debug)]
struct Triple {
    s: String,
    p: String,
    o: String
}

fn load_graph(data: &Value) -> Graph {

    println!("{}", data);

    let map = data.as_object().unwrap();
    
    for key in map.keys() {

        match key.as_ref() {
            "@context" => { println!("context") },
            "@id" => { println!("id ^^") },
            _ => { println!("key {}", key) }
        };
    }

    return Graph(String::from("abc"))
}

fn main() {
    
    let content = fs::read_to_string("test.jsonld")
        .expect("Error reading file");
    let data: Value = serde_json::from_str(&content)
        .expect("Error parsing json");
    let context_file = &data["@context"];
    println!("{:?}", context_file);

    let g = load_graph(&data);
    println!("{:?}", g);
    
}
