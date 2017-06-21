extern crate encoding;
extern crate chrono;
#[macro_use(bson,doc)]
extern crate bson;
extern crate mongodb;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;
use bson::Document;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use self::reportserver::Entry;
mod reportserver;

pub fn read_file_and_insert(path: &String, seperator: char) {
    let content = read_latin1_file(path);

    let lines: Vec<&str> = content.split("\r\n").collect();
    let mut entries: Vec<Entry> = vec![];
    let mut documents: Vec<Document> = vec![];

    for line in lines.iter().skip(1) {
        if line.trim() == "" {
            continue;
        }
        let values: Vec<&str> = line.split(seperator).collect();
        let entry = Entry::new(values);

        documents.push(entry.to_bson());
        entries.push(entry);
    }

    println!("entries found {}", entries.len());

    let client = Client::connect("localhost", 27017).expect("could not connect to mongodb");
    let coll = client.db("report").collection("entries");
    for document_chunks in documents.chunks(999){
        let result = coll.insert_many(document_chunks.to_vec(), None);
        match result{
            Err(e) => println! ("error during insert: {}", e),
            Ok(_) => (),
        }
    }
}

fn read_latin1_file(file_name: &String) -> String {

    let mut file = match File::open(&file_name) {
        Err(why) => panic!("couldn't open {}: {}", file_name,
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
