use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use lazy_static::lazy_static;
use regex::Regex;

const DATETIME_ZONE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S %z";
const DATETIME_NANO_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S.%f";
const UTC_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";
const UTC_MS_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S.%fZ";
const DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
const DATE_FORMAT: &'static str = "%Y-%m-%d";
const SIMPLE_DATE_FORMAT: &'static str = "%Y.%m.%d";

lazy_static! {
    static ref SIMPLE_DATE_REGEX: Regex = Regex::new(r"(?P<time>\d{4}\.\d{2}\.\d{2})").unwrap();
    static ref ISO8601_DATE_REGEX: Regex = Regex::new(r"(?P<time>\d{4}-\d{2}-\d{2})").unwrap();
    static ref ISO8601_DATE_TIME_REGEX: Regex =
        Regex::new(r"(?P<time>\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2})").unwrap();
    static ref RFC3339_NANO_REGEX: Regex =
        Regex::new(r"(?P<time>\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2}\.\d{1,8})").unwrap();
    static ref UTC_TIME_REGEX: Regex =
        Regex::new(r"(?P<time>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z)").unwrap();
    static ref UTC_TIME_MS_REGEX: Regex =
        Regex::new(r"(?P<time>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{1,8}Z)").unwrap();
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
///
/// long time should be in first
pub fn replace_to_unix(text: &str) -> String {
    let mut result = text.to_string();
    // for: 2021-08-20T06:32:28.214Z
    for caps in UTC_TIME_MS_REGEX.captures_iter(text) {
        let time = &caps["time"];
        let naive_date_time = NaiveDateTime::parse_from_str(time, UTC_MS_FORMAT).unwrap();
        result = result.replace(time, naive_date_time.timestamp().to_string().as_str());
    }

    // for: 2021-08-20T06:32:28Z
    for caps in UTC_TIME_REGEX.captures_iter(text) {
        let time = &caps["time"];
        let naive_date_time = NaiveDateTime::parse_from_str(time, UTC_FORMAT).unwrap();
        result = result.replace(time, naive_date_time.timestamp().to_string().as_str());
    }

    // first for: 2021-08-20 06:32:28.537346
    for caps in RFC3339_NANO_REGEX.captures_iter(text) {
        let time = &caps["time"];
        let naive_date_time = NaiveDateTime::parse_from_str(time, DATETIME_NANO_FORMAT).unwrap();
        result = result.replace(time, naive_date_time.timestamp().to_string().as_str());
    }

    for caps in ISO8601_DATE_TIME_ZONE_REGEX.captures_iter(text) {
        let time = &caps["time"];
        let naive_date_time = DateTime::parse_from_str(time, DATETIME_ZONE_FORMAT).unwrap();
        let timestamp = naive_date_time.timestamp();
        result = result.replace(time, timestamp.to_string().as_str());
    }

    for caps in ISO8601_DATE_TIME_REGEX.captures_iter(text) {
        let time = &caps["time"];
        let naive_date_time = NaiveDateTime::parse_from_str(time, DATETIME_FORMAT).unwrap();
        let timestamp = naive_date_time.timestamp();
        result = result.replace(time, timestamp.to_string().as_str());
    }

    for caps in ISO8601_DATE_REGEX.captures_iter(text) {
        let time = &caps["time"];
        let naive_date = NaiveDate::parse_from_str(time, DATE_FORMAT).unwrap();
        let naive_datetime = naive_date.and_hms(0, 0, 0);
        result = result.replace(time, naive_datetime.timestamp().to_string().as_str());
    }

    for caps in SIMPLE_DATE_REGEX.captures_iter(text) {
        let time = &caps["time"];
        let naive_date = NaiveDate::parse_from_str(time, SIMPLE_DATE_FORMAT).unwrap();
        let naive_datetime = naive_date.and_hms(0, 0, 0);
        result = result.replace(time, naive_datetime.timestamp().to_string().as_str());
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::helper::quake_time::replace_to_unix;

    #[test]
    fn iso_time_replace() {
        let filter1 = "created_date > 2020-04-12 22:10:57 +08:00";
        assert_eq!(replace_to_unix(filter1), "created_date > 1586700657");

        let filter2 = "created_date > 2020-04-12 22:10:57";
        assert_eq!(replace_to_unix(filter2), "created_date > 1586729457");

        let filter3 = "created_date > 2021-12-09";
        assert_eq!(replace_to_unix(filter3), "created_date > 1639008000");

        let filter4 = "created_date > 2021.12.09";
        assert_eq!(replace_to_unix(filter4), "created_date > 1639008000");
    }

    #[test]
    fn rfc_time_replace() {
        let filter5 = "created_date > 2021-08-20 06:32:28.537346";
        assert_eq!(replace_to_unix(filter5), "created_date > 1629441148");

        let filter6 = "created_date > 2021-11-08T07:25:26Z";
        assert_eq!(replace_to_unix(filter6), "created_date > 1636356326");

        let filter7 = "created_date > 2021-11-08T07:25:26.125Z";
        assert_eq!(replace_to_unix(filter7), "created_date > 1636356326");
    }

    #[test]
    fn multiple_time_replace() {
        let filter1 = "created_date > 2020-04-12 22:10:57 +08:00 AND created_date < 2020-05-12 22:10:57 +08:00";
        assert_eq!(
            replace_to_unix(filter1),
            "created_date > 1586700657 AND created_date < 1589292657"
        );

        let filter1 = "created_date > 2020-04-12 22:10:57 +08:00 AND created_date < 2020-05-12";
        assert_eq!(
            replace_to_unix(filter1),
            "created_date > 1586700657 AND created_date < 1589241600"
        );
    }
}
