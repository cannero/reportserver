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
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;

fn main() {

    let content = read_latin1_file("c:/dat/planning/Leistungsnachweis_17April.txt".to_string());

    let lines: Vec<&str> = content.split("\r\n").collect();
    let mut entries: Vec<Entry> = vec![];
    //todo check for header line

    for line in lines.iter().skip(1) {
        if line.trim() == "" {
            continue;
        }
        let values: Vec<&str> = line.split(";").collect();
        let entry = create_entry(values);
        entries.push(entry);
    }

    if entries.len() == 0 {
        return;
    }
    println!("entries found {}", entries.len());

    let client = Client::connect("localhost", 27017).expect("could not connect to mongodb");
    let coll = client.db("report").collection("data");

    let mut current_date = entries[0].date;
    for entry in entries {
        if entry.date != current_date {
            current_date = entry.date;
            println!("changed to {}", current_date);
        }
    }
}

fn test_mongo() {
    let client = Client::connect("localhost", 27017).expect("could not connect to mongodb");
    let coll = client.db("test").collection("test");
    coll.insert_one(doc! {"a" => "800"}, None).unwrap();
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

fn create_entry(input: Vec<&str>) -> Entry {
    let parse_from_str = NaiveDate::parse_from_str;
    assert!(input.len() >= 6, "does not contain at least 6 entries {:?}", input);

    let hours_and_minutes: Vec<&str> = input[6].split(":").collect();
    let hours: u32 = hours_and_minutes[0].parse().expect("hours not a number");
    let minutes: u32 = hours_and_minutes[1].parse().expect("minutes not a number");

    Entry {
        date: parse_from_str(input[0], "%d.%m.%y").unwrap(),
        customer: input[1].to_string(),
        employee: input[2].to_string(),
        project: input[3].to_string(),
        event: input[4].to_string(),
        comment: input[5].to_string(),
        duration: (hours * 3600) + (minutes * 60),
    }
}

#[test]
fn test_create_entry(){
    let input = vec!["03.04.17", "NASA", "Alfons, Hans", "Blaster2000", "/AA/BB/CC",
                     "Hochspannung erzeugen", "02:30"];
    let entry = create_entry(input);
    assert_eq!(entry.date, NaiveDate::from_ymd(2017, 4, 3));
    assert_eq!(entry.customer, "NASA");
    assert_eq!(entry.duration, 2 * 60 * 60 + 30 * 60);
}

struct Entry {
    date: NaiveDate,
    employee: String,
    duration: u32,
    customer: String,
    project: String,
    event: String,
    comment: String,
}

fn insert_date(date: NaiveDate, collection: Collection) {
    let from_utc = DateTime::<UTC>::from_utc;
    let date_time = from_utc(date.and_hms(0, 0, 0), UTC);
    collection.insert_one(doc! {"date" => date_time}, None).unwrap();
}


