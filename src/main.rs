use std::fs;
use serde_json::{Value};
use reqwest::Error;

mod triple;
use triple::{Triple, Uri, Entity};

mod graph;
use graph::Graph;

mod context;
use context::Context;

mod helpers;


fn load_graph(data: &Value) -> Result<Graph, Error> {

    println!("{}", data);

    // read needed information (id and properties) from json object
    let map = data.as_object().unwrap();
    let mut id: &str = "";
    let mut properties: Vec<&str> = Vec::new();
    let mut context = Context::new();
    
    for key in map.keys() {
        match key.as_ref() {
            "@context" => { context.load(&map[key]) },
            "@id" => { id = map[key].as_str().unwrap() },
            _ => { properties.push(key) }
        };
    }

    println!("CONTEXT {:?}", context);

    // create triples for each statement
    let mut graph = Graph::new();
    for property in properties {
        let value = map[property].as_str().unwrap();
        let triple = Triple::new(
            Uri::new(id),
            Uri::new(property),
            Entity::new(value));
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
