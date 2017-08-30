use mongodb::{Client, ThreadedClient};
use mongodb::db::{Database, ThreadedDatabase};
use mongodb::coll::Collection;
use mongodb::cursor::Cursor;
use chrono::prelude::*;
use chrono::Duration;

pub fn get_employee_data_newer_than(days: i64) -> usize{
    let coll = get_entries_collection();
    let now = UTC::now();
    let duration = Duration::days(days);
    let date_from = now - duration;
    let mut cursor = coll.find(Some(doc!{"date" => {"$gte" => date_from}}),
                               None).expect("failed to execute find command");
    let count = cursor.count();
    count 
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
