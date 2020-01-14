use serde_json::{Value, Map};
use std::collections::HashMap;

use crate::triple::Uri;
use crate::helpers::fetch_json;

#[derive(Debug)] 
pub struct Context {
    namespaces: HashMap<String, Uri>,
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

        let mut tmp_terms = Vec::new();

        for (key, value) in context_map {
            match value {
                Value::String(uri) => {
                    let last_char = uri.chars().nth(uri.len()-1).unwrap();
                    
                    match last_char {
                        '#' | '/' => {
                            // add entry to namespaces
                            self.namespaces.insert(
                                String::from(key),
                                Uri::new(uri)
                            );
                        },
                        _ => { 
                            tmp_terms.push((String::from(key), String::from(uri), String::from("@id")));
                        }
                    }
                },
                Value::Object(obj) => {
                    tmp_terms.push((
                        String::from(key), 
                        String::from(obj["@id"].as_str().unwrap()),
                        String::from(obj["@type"].as_str().unwrap())
                    ));
                }
                _ => ()
            }
        }          

        for (alias, uri, data_type) in tmp_terms {
            self.terms.insert(
                alias,
                Term::new(Uri::new(&uri), DataType::Id)
            );
        }

    }
    

    pub fn load(&mut self, context_obj: &Value) {
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
pub struct Term {
    uri: Uri,
    data_type: DataType
}

impl Term {
    pub fn new(uri: Uri, data_type: DataType) -> Term {
        Term {
            uri, data_type
        }
    }

    pub fn uri(&self) -> Uri {
        self.uri.clone()
    }
}

#[derive(Debug)]
pub enum DataType {
    Id, 
    Literal(Uri)
}