use mongodb::{Client, ThreadedClient};
use mongodb::db::{Database, ThreadedDatabase};
use mongodb::coll::Collection;
use mongodb::coll::options::FindOptions;
use chrono::prelude::*;
use chrono::Duration;
use bson::{Bson, Document};
use std::collections::HashMap;

pub fn get_employee_data_newer_than(days: i64) -> String{
    let coll = get_entries_collection();
    let now = UTC::now();
    let duration = Duration::days(days);
    let date_from = now - duration;

    let mut options = FindOptions::new();
    options.projection = Some(doc! {
        "_id" => 0,
        "employee" => 1,
        "customer" => 1,
//        "project" => 1,
        "comment" => 1,
        "date" => 1,
        "duration" => 1,
        "event" => 1
    });

    options.sort = Some(doc! {
        "division" => 1,
        "employee" => 1,
        "date" => 1,
        "customer" => 1,
        "event" => 1
    });
    
    let cursor = coll.find(Some(doc!{"date" => {"$gte" => date_from}}),
                               Some(options))
        .expect("failed to execute find command");

    let mut result_string = "{\"result\":[".to_string();
    
    for (i, doc_result) in cursor.enumerate() {
        match doc_result{
            Ok(doc) => {
                let json_string = doc_to_json_string(doc);
                let next_block = if i == 0 {
                    format!("{}", json_string)
                } else {
                    format!(",{}", json_string)
                };
                result_string.push_str(&next_block);
            },
            Err(e) => panic!("err {}", e),
        };
    }

    result_string.push_str("]}");
    
    result_string 
}

const FORMAT: &'static str = "%Y-%m-%d";

fn doc_to_json_string(doc: Document) -> String {

    let mut key_and_values = HashMap::new();
    for elem in doc {
        let key = elem.0;
        match elem.1 {
            Bson::String(v) => key_and_values.insert(key, v.to_string()),
            Bson::UtcDatetime(v) => {
                let s = format!("{}", v.format(FORMAT));
                key_and_values.insert(key, s)
            },
            Bson::I32(v) => key_and_values.insert(key, v.to_string()),
            Bson::I64(v) => key_and_values.insert(key, v.to_string()),
            _ => panic!("doc_to_json_string not defined for {}", elem.1),
        };
    }

    format!("{}", json!(key_and_values))
}

fn get_entries_collection() -> Collection {
    let db = get_report_db();
    db.collection("entries")
}

fn get_report_db() -> Database {
    let client = connect();
    client.db("report")
}

fn connect() -> Client {
   
    let client = Client::connect("localhost", 27017).expect("could not connect to mongodb");
    client
}
