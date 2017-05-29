extern crate encoding;
extern crate chrono;
#[macro_use(bson,doc)]
extern crate bson;
extern crate mongodb;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;
use chrono::prelude::*;
use bson::Document;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use mongodb::coll::options::UpdateOptions;

use self::reportserver::Entry;
mod reportserver;

fn main() {

    let content = read_latin1_file("c:/dat/planning/Leistungsnachweis_17April.txt".to_string());

    let lines: Vec<&str> = content.split("\r\n").collect();
    let mut entries: Vec<Entry> = vec![];
    let mut documents: Vec<Document> = vec![];

    for line in lines.iter().skip(1) {
        if line.trim() == "" {
            continue;
        }
        let values: Vec<&str> = line.split(";").collect();
        let entry = Entry::new(values);

        documents.push(entry.to_bson());
        entries.push(entry);
    }

    if entries.len() == 0 {
        return;
    }
    println!("entries found {}", entries.len());

    let client = Client::connect("localhost", 27017).expect("could not connect to mongodb");
    let coll = client.db("report").collection("data");

    let mut current_date = entries[0].date;
    insert_date(&current_date, &coll);
    for entry in entries {
        if entry.date != current_date {
            current_date = entry.date;
            println!("changed to {}", current_date);
            insert_date(&entry.date, &coll);
        }
        add_entry(&entry, &coll);
    }
}

fn read_latin1_file(file_name: String) -> String {
    let path = Path::new(&file_name);
    let display = path.display();
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    let mut content_as_byte = Vec::new();
    file.read_to_end(&mut content_as_byte).expect("Unable to read");

    let content  = match ISO_8859_1.decode(&content_as_byte, DecoderTrap::Strict) {
        Err(why) => panic!("decode failed {}", why),
        Ok(content) => content,
    };

    content
}

fn insert_date(date: &NaiveDate, collection: &Collection) {
    let date_time = reportserver::get_utc_from_naive(date);
    collection.insert_one(doc! {"date" => date_time}, None).unwrap();
}

fn add_entry(entry: &Entry, collection: &Collection) {
    let date_time = reportserver::get_utc_from_naive(&entry.date);
    let update = doc! {"$push" =>
                       {"entries" =>
                        {"employee" => (&entry.employee),
                         "customer" => (&entry.customer),
                         "project" => (&entry.project),
                         "event" => (&entry.event),
                         "duration" => (entry.duration),
                         "comment" => (&entry.comment)}}};
    let mut update_options = UpdateOptions::new();
    update_options.upsert = Some(true);
    let result = collection.update_one(doc! {"date" => date_time},
                                       update,
                                       Some(update_options));
    match result{
        Err(e) => println! ("some error {}", e),
        Ok(_) => ()
    }
}



