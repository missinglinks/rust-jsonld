use serde_json::{Value, Map};
use std::collections::HashMap;

use crate::triple::Uri;
use crate::helpers::fetch_json;

#[derive(Debug)] 
pub struct Context {
    namespaces: HashMap<String, Namespace>,
    terms: HashMap<String, Term>
}

impl Context {

    pub fn new() -> Context {
        Context {
            namespaces: HashMap::new(),
            terms: HashMap::new()
        }
    }
    
    fn _parse_context(&mut self, context_map: &Map<String, Value>) {

        for (key, value) in context_map {
            match value {
                Value::String(uri) => {
                    let last_char = uri.chars().nth(uri.len()-1).unwrap();
                    
                    match last_char {
                        '#' | '/' => {
                            // add entry to namespaces
                            self.namespaces.insert(
                                String::from(key),
                                Namespace::new(Uri::new(uri), String::from(key))
                            );
                        },
                        _ => { 
                            // entry is term, deal with it
                            self.terms.insert(
                                String::from(key),
                                Term::new(Uri::new(uri), String::from(key))
                            );
                        }
                    }
                },
                Value::Object(obj) => {
                    /*
                    let prop_type = match obj["@type"].as_str().unwrap() {
                        "@id" => PropertyType::Id,
                        "xsd:date" => PropertyType::Date,
                        _ => PropertyType::Undefined
                    };*/
                    self.terms.insert(
                        String::from(key),
                        Term::new(Uri::new(obj["@id"].as_str().unwrap()), String::from(key))
                    );
                }
                _ => ()
            }
        }          
    }
    

    pub fn load(&mut self, context_obj: &Value) {
        //let mut properties: HashMap<String, Property> = HashMap::new();

        match context_obj {
            Value::String(uri) => { 
                // load and parse context file
                let context_file = fetch_json(uri)
                    .expect("Error retrieving context file");
                
                let map = context_file["@context"].as_object().unwrap();
                self._parse_context(&map);
            },
            Value::Object(obj) => { 
                // parse context object
                println!("context object");
                println!("{:?}", obj);

                self._parse_context(&obj);
            },
            _ => ()
        }

    }

    pub fn term(&self, alias: &str) -> Uri {
        self.terms[alias].uri()
    }
}

#[derive(Debug)]
pub struct Namespace {
    uri: Uri,
    alias: String
}

impl Namespace {
    pub fn new(uri: Uri, alias: String) -> Namespace {
        Namespace {
            uri, alias
        }
    }

    pub fn uri(&self) -> Uri {
        self.uri.clone()
    }    
}

#[derive(Debug)]
pub struct Term {
    uri: Uri,
    term: String
}

impl Term {
    pub fn new(uri: Uri, term: String) -> Term {
        Term {
            uri, term
        }
    }

    pub fn uri(&self) -> Uri {
        self.uri.clone()
    }
}