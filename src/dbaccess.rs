use mongodb::{Client, ThreadedClient};
use mongodb::db::{Database, ThreadedDatabase};
use mongodb::coll::Collection;
use mongodb::coll::options::FindOptions;
use chrono::prelude::*;
use chrono::Duration;
use bson::{Bson, Document};
use std::collections::HashMap;

pub fn get_employee_data_newer_than(client: &Client, days: u32) -> String{
    let coll = get_entries_collection(client);
    let now = UTC::now();
    let duration = Duration::days(days as i64);
    let date_from = now - duration;

    let options = get_standard_find_options();
    
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

pub fn get_entries_containing_comment(client: &Client, part: &String) -> String {

    let coll = get_entries_collection(client);
    let options = get_standard_find_options();

    let cursor = coll.find(Some(doc!{"comment" => {"$regex" => part,
                                                   "$options" => "i"}}),
                           Some(options))
        .expect("failed to execute find command");

    let mut result_string = "{\"result\":[".to_string();
    let mut total_duration = 0;
    
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

pub fn time_per_customer(client: &Client) -> String {
    let coll = get_entries_collection(client);

    let group = doc!{"$group" => {"_id" => {"customer" => "$customer"},"duration" => {"$sum" => "$duration"}}};
    let sort = doc!{"$sort" => {"_id.customer" => 1}};

    let cursor = coll.aggregate(vec![group],
                                None)
        .expect("failed to execute find command");

    let mut result_string = "{\"result\":[".to_string();
    let mut total_duration = 0;
    
    for (i, doc_result) in cursor.enumerate() {
        match doc_result{
            Ok(doc) => {
                let mut doc_customer_and_duration = match doc.get_document("_id") {
                    Ok(d) => d.clone(),
                    Err(e) => panic!("no document with _id, {}", e),
                };
                match doc.get("duration").expect("time per customer no duration defined") {
                    &Bson::I32(v) => {
                        let duration_hours = v / 3600;
                        total_duration = total_duration + duration_hours;
                        doc_customer_and_duration.insert("duration", duration_hours);
                    },
                    _ => panic!("time per customer duration not an i32"),
                };

                let json_string = doc_to_json_string(doc_customer_and_duration);
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

    result_string.push_str("], \"total_duration\": ");
    result_string.push_str(&format!("{}", total_duration));
    result_string.push_str("}");

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

fn get_standard_find_options() -> FindOptions {

    let mut options = FindOptions::new();
    options.projection = Some(doc! {
        "_id" => 0,
        "employee" => 1,
        "customer" => 1,
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
    options
}

fn get_entries_collection(client: &Client) -> Collection {
    let db = get_report_db(client);
    db.collection("entries")
}

fn get_report_db(client: &Client) -> Database {
    client.db("report")
}

pub fn get_connection() -> Client {
    let client = Client::connect("localhost", 27017).expect("could not connect to mongodb");
    client
}
