use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use lazy_static::lazy_static;
use regex::Regex;

const DATETIME_ZONE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S %z";
const DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
const DATE_FORMAT: &'static str = "%Y-%m-%d";

lazy_static! {
    static ref ISO8601_DATE_REGEX: Regex =
        Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    static ref ISO8601_DATE_TIME_REGEX: Regex =
        Regex::new(r"((?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})\s\d{2}:\d{2}:\d{2})").unwrap();
    static ref ISO8601_DATE_TIME_ZONE_REGEX: Regex =
        Regex::new(r"((?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})\s\d{2}:\d{2}:\d{2}\s\+\d{2}:\d{2})")
            .unwrap();
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
pub fn text_date_to_unix(text: &str) -> i64 {
    let timestamp = 0;

    if let Some(_caps) = ISO8601_DATE_TIME_ZONE_REGEX.captures(text) {
        let naive_date_time = DateTime::parse_from_str(text, DATETIME_ZONE_FORMAT).unwrap();
        return naive_date_time.timestamp();
    };

    if let Some(_caps) = ISO8601_DATE_TIME_REGEX.captures(text) {
        let naive_date_time = NaiveDateTime::parse_from_str(text, DATETIME_FORMAT).unwrap();
        return naive_date_time.timestamp();
    };

    if let Some(_caps) = ISO8601_DATE_REGEX.captures(text) {
        let naive_date = NaiveDate::parse_from_str(text, DATE_FORMAT).unwrap();
        let naive_datetime = naive_date.and_hms(0, 0, 0);
        return naive_datetime.timestamp();
    };

    timestamp
}

#[cfg(test)]
mod tests {
    use crate::helper::quake_time::text_date_to_unix;

    #[test]
    fn time_regex_match() {
        assert_eq!(text_date_to_unix("2020-04-12 22:10:57 +08:00"), 1586700657);
        assert_eq!(text_date_to_unix("2020-04-12 22:10:57"), 1586729457);
        assert_eq!(text_date_to_unix("2021-12-09"), 1639008000);
    }
}
