use serde_json::{Value};
use std::collections::HashMap;

use crate::triple::Uri;
use crate::helpers::fetch_json;

#[derive(Debug)] 
pub struct Context {
    properties: HashMap<String, Property>
}

impl Context {

    pub fn new() -> Context {
        Context {
            properties: HashMap::new()
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
                for (key, value) in map {
                    match value {
                        Value::String(uri) => {
                            self.properties.insert(
                                String::from(key), 
                                Property::new(Uri::new(uri), PropertyType::Id));
                        },
                        Value::Object(obj) => {
                            let prop_type = match obj["@type"].as_str().unwrap() {
                                "@id" => PropertyType::Id,
                                "xsd:date" => PropertyType::Date,
                                _ => PropertyType::Undefined
                            };                            

                            self.properties.insert(
                                String::from(key), 
                                Property::new(
                                    Uri::new(obj["@id"].as_str().unwrap()), 
                                    prop_type));
                        }
                        _ => ()
                    }

                }


            },
            Value::Object(obj) => { 
                // parse context object

            },
            _ => ()
        }

    }

    pub fn property(&self, property: &str) -> Uri {
        println!("{}", property);
        self.properties[property].uri()
    }
}

#[derive(Debug)]
pub struct Property {
    uri: Uri,
    prop_type: PropertyType
}

impl Property {
    pub fn new(uri: Uri, prop_type: PropertyType) -> Property {
        Property {
            uri, prop_type
        }
    }

    pub fn uri(&self) -> Uri {
        self.uri.clone()
    }
}


#[derive(Debug)]
pub enum PropertyType {
    Id,
    Date,
    Undefined
}