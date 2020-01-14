use std::fs;
use serde_json::{Value};

mod triple;
use triple::{Triple, Uri, Entity};

mod graph;
use graph::Graph;

mod context;
use context::Context;

mod helpers;

#[derive(Debug)]
struct JsonLDFile {
    filepath: String,
    context: Context,
    graph: Graph,
    json_data: Value
}

impl JsonLDFile {
    pub fn new(filepath: &str) -> JsonLDFile {

        let content = fs::read_to_string(filepath)
            .expect("Error reading file");
        let data: Value = serde_json::from_str(&content)
            .expect("Error parsing json");

        let mut context = Context::new();
        let mut graph = Graph::new();


        // read needed information (id and properties) from json object
        let map = data.as_object().unwrap();
        let mut id: &str = "";
        let mut properties: Vec<&str> = Vec::new();
        
        for key in map.keys() {
            match key.as_ref() {
                "@context" => { context.load(&map[key]) },
                "@id" => { id = map[key].as_str().unwrap() },
                _ => { properties.push(key) }
            };
        }

        // create triples for each statement
        for property in properties {
            let value = map[property].as_str().unwrap();
            let triple = Triple::new(
                Uri::new(id),
                context.term(property),
                Entity::new(value));
            graph.add(triple);
        }

        JsonLDFile {
            filepath: String::from(filepath), 
            context: context, 
            graph: graph, 
            json_data: data
        }
    }
}


fn main() {
    let jsonld = JsonLDFile::new("test.jsonld");
    //let jsonld = JsonLDFile::new("test.txt.prov");
    println!("{:?}", jsonld);
}
