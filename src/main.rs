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
    g: Vec<Triple>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            g: Vec::new()
        }
    }

    pub fn add(&mut self, triple: Triple) {
        self.g.push(triple);
    }
}


#[derive(Debug)]
struct Triple {
    s: Uri,
    p: Uri,
    o: Entity
}

impl Triple {
    pub fn new(s: Uri, p: Uri, o: Entity) -> Triple {
        Triple {
            s, p, o
        }
    }
}

#[derive(Debug)]
struct Uri(String);

#[derive(Debug)]
enum Entity {
    Uri(Uri),
    Literal(String)
}


fn load_graph(data: &Value) -> Result<Graph, Error> {

    println!("{}", data);

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

    println!("{}", id);
    println!("{:?}", properties);

    let mut graph = Graph::new();

    let triple = Triple::new(
        Uri(String::from("abv")),
        Uri(String::from("pred")),
        Entity::Literal(String::from("adsd")));

    graph.add(triple);

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
