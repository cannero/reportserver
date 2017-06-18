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
    pub division: String,
}

impl Entry {
    pub fn new(input: Vec<&str>) -> Entry {
        let parse_from_str = NaiveDate::parse_from_str;
        //last entry is the empty string, that's why check for 8
        assert!(input.len() == 8, "does not contain 7 entries {:?}", input);
        let employes_arad = vec!["Tanasie, Cosmin", "Teudan, Emanuel", "Nicoara, Brigitta", "Sokol, Peter", "Homone, Adrian", "Ivan, Marius"];
        let (hours, minutes) = match get_hours_and_minutes_from_string(input[6].to_string()) {
            Some((h, m)) => (h, m),
            None => {
                println!("no number found {}", input.join("-"));
                (0, 0)
            }
        };

        let employee = input[2];
        let division =
            if employes_arad.contains(&employee) {
                "Arad"
            } else {
                "Krailling"
            };

        Entry {
            date: parse_from_str(input[0], "%d.%m.%y").unwrap(),
            customer: input[1].to_string(),
            employee: employee.to_string(),
            project: input[3].to_string(),
            event: input[4].to_string(),
            comment: input[5].to_string(),
            duration: (hours * 3600) + (minutes * 60),
            division: division.to_string(),
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
              "comment" => (&self.comment),
              "division" => (&self.division) }
    }
}

fn get_hours_and_minutes_from_string(input: String) -> Option<(u32, u32)> {
    if input == "" {
        return None;
    }
    let hours_and_minutes: Vec<&str> = input.split(":").collect();
    if hours_and_minutes.len() != 2 {
        return None;
    }
    let hours: u32 = match hours_and_minutes[0].parse() {
        Ok(i) => i,
        Err(_e) => return None,
    };
    let minutes: u32 = match hours_and_minutes[1].parse() {
        Ok(i) => i,
        Err(_e) => return None,
    };
    Some((hours, minutes))
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
