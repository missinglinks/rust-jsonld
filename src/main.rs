use std::fs;
use serde_json::{Value};
use reqwest::Error;

mod triple;
use triple::{Triple, Uri, Entity};

mod graph;
use graph::Graph;

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

fn load_graph(data: &Value) -> Result<Graph, Error> {

    println!("{}", data);

    // read needed information (id and properties) from json object
    let map = data.as_object().unwrap();
    let mut id: String = String::from("");
    let mut properties: Vec<String> = Vec::new();
    
    for key in map.keys() {
        match key.as_ref() {
            "@context" => { println!("context") },
            "@id" => { id = map[&String::from(key)].to_string() },
            _ => { properties.push(String::from(key)) }
        };
    }

    // create triples for each statement
    let mut graph = Graph::new();
    for property in properties {
        let value = map[&property].to_string();
        let triple = Triple::new(
            Uri::new(&id),
            Uri::new(&property),
            Entity::Uri(Uri::new(&value))
        );
        graph.add(triple);
    }

    return Ok(graph)
}

fn main() {
    
    let content = fs::read_to_string("test.jsonld")
        .expect("Error reading file");
    let data: Value = serde_json::from_str(&content)
        .expect("Error parsing json");
    let context_file = &data["@context"];
    println!("{:?}", context_file);

    let g = load_graph(&data).expect("error loading graph");
    println!("{:?}", g);
    
}
