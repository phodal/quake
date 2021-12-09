use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use lazy_static::lazy_static;
use regex::Regex;

const DATETIME_ZONE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S %z";
const DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
const DATE_FORMAT: &'static str = "%Y-%m-%d";
const SIMPLE_DATE_FORMAT: &'static str = "%Y.%m.%d";

lazy_static! {
    static ref SIMPLE_DATE_REGEX: Regex = Regex::new(r"(?P<time>\d{4}.\d{2}.\d{2})").unwrap();
    static ref ISO8601_DATE_REGEX: Regex = Regex::new(r"(?P<time>\d{4}-\d{2}-\d{2})").unwrap();
    static ref ISO8601_DATE_TIME_REGEX: Regex =
        Regex::new(r"(?P<time>\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2})").unwrap();
    static ref ISO8601_DATE_TIME_ZONE_REGEX: Regex =
        Regex::new(r"(?P<time>\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2}\s\+\d{2}:\d{2})").unwrap();
}

pub fn date_now() -> String {
    let local: DateTime<Local> = Local::now();
    local.format(DATETIME_FORMAT).to_string()
}

/// see in: https://stackoverflow.com/questions/61179070/rust-chrono-parse-date-string-parseerrornotenough-and-parseerrortooshort
/// "2020-04-12" => Date = NaiveDate
/// "22:10" => Time = NaiveTime
/// "2020-04-12 22:10:57" => Date + Time = NaiveDateTime
/// "2020-04-12 22:10:57+02:00" => Date + Time + TimeZone = DateTime<Tz>
///
/// RFC2822 = Date + Time + TimeZone
/// RFC3339 = Date + Time + TimeZone
/// Date + Time + Timezone (other or non-standard)
/// Date + Time
/// Date
pub fn text_date_to_unix(text: &str) -> String {
    if let Some(caps) = ISO8601_DATE_TIME_ZONE_REGEX.captures(text) {
        let time = &caps["time"];
        let naive_date_time = DateTime::parse_from_str(time, DATETIME_ZONE_FORMAT).unwrap();
        let timestamp = naive_date_time.timestamp();
        return text.replace(time, timestamp.to_string().as_str());
    };

    if let Some(caps) = ISO8601_DATE_TIME_REGEX.captures(text) {
        let time = &caps["time"];
        let naive_date_time = NaiveDateTime::parse_from_str(time, DATETIME_FORMAT).unwrap();
        let timestamp = naive_date_time.timestamp();
        return text.replace(time, timestamp.to_string().as_str());
    };

    if let Some(caps) = ISO8601_DATE_REGEX.captures(text) {
        let time = &caps["time"];
        let naive_date = NaiveDate::parse_from_str(time, DATE_FORMAT).unwrap();
        let naive_datetime = naive_date.and_hms(0, 0, 0);
        return text.replace(time, naive_datetime.timestamp().to_string().as_str());
    };

    if let Some(caps) = SIMPLE_DATE_REGEX.captures(text) {
        let time = &caps["time"];
        let naive_date = NaiveDate::parse_from_str(time, SIMPLE_DATE_FORMAT).unwrap();
        let naive_datetime = naive_date.and_hms(0, 0, 0);
        return text.replace(time, naive_datetime.timestamp().to_string().as_str());
    };

    text.to_string()
}

#[cfg(test)]
mod tests {
    use crate::helper::quake_time::text_date_to_unix;

    #[test]
    fn time_replace() {
        let filter1 = "created_date > 2020-04-12 22:10:57 +08:00";
        assert_eq!(text_date_to_unix(filter1), "created_date > 1586700657");

        let filter2 = "created_date > 2020-04-12 22:10:57";
        assert_eq!(text_date_to_unix(filter2), "created_date > 1586729457");

        let filter3 = "created_date > 2021-12-09";
        assert_eq!(text_date_to_unix(filter3), "created_date > 1639008000");

        let filter4 = "created_date > 2021.12.09";
        assert_eq!(text_date_to_unix(filter4), "created_date > 1639008000");
    }
}
