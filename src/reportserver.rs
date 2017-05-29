use chrono::prelude::*;
use bson::Document;

pub fn get_utc_from_naive(date: &NaiveDate) -> DateTime<UTC> {
    let from_utc = DateTime::<UTC>::from_utc;
    from_utc(date.and_hms(0, 0, 0), UTC)
}

pub struct Entry {
    pub date: NaiveDate,
    pub employee: String,
    pub duration: u32,
    pub customer: String,
    pub project: String,
    pub event: String,
    pub comment: String,
}

impl Entry {
    pub fn new(input: Vec<&str>) -> Entry {
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

    pub fn to_bson(&self) -> Document {
        let date_time = get_utc_from_naive(&self.date);
        doc! {"date" => date_time,
              "employee" => (&self.employee),
              "customer" => (&self.customer),
              "project" => (&self.project),
              "event" => (&self.event),
              "duration" => (self.duration),
              "comment" => (&self.comment)}
    }
}

#[test]
fn test_create_entry(){
    let input = vec!["03.04.17", "NASA", "Alfons, Hans", "Blaster2000", "/AA/BB/CC",
                     "Hochspannung erzeugen", "02:30"];
    let entry = Entry::new(input);
    assert_eq!(entry.date, NaiveDate::from_ymd(2017, 4, 3));
    assert_eq!(entry.customer, "NASA");
    assert_eq!(entry.duration, 2 * 60 * 60 + 30 * 60);
}
