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
                //println!("{:?}", context_file);
                
                let map = context_file["@context"].as_object().unwrap();
                for (key, value) in map {
                    match value {
                        Value::String(uri) => {
                            self.properties.insert(
                                String::from(key), 
                                Property::new(Uri::new(uri), PropertyType::Id));
                        },
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
}

#[derive(Debug)]
pub enum PropertyType {
    Id,
    Date,
}