use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use regex::Regex;

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
const ISO8601_DATE: &'static str = "%Y-%m-%d %H:%M:%S";
lazy_static! {
    static ref ISO8601_DATE_REGEX: Regex =
        Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
}

pub fn date_now() -> String {
    let local: DateTime<Local> = Local::now();
    local.format(FORMAT).to_string()
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
pub fn text_date_to_unix() {}

#[cfg(test)]
mod tests {
    use crate::helper::quake_time::ISO8601_DATE_REGEX;
    use chrono::{DateTime, NaiveDate, NaiveDateTime};

    #[test]
    fn time_regex_match() {
        let time = "2021-12-09";
        let timestamp = match ISO8601_DATE_REGEX.captures(&time) {
            Some(_caps) => {
                let naive_date = NaiveDate::parse_from_str(time, "%Y-%m-%d").unwrap();
                let naive_datetime = naive_date.and_hms(0, 0, 0);

                naive_datetime.timestamp()
            }
            _ => 0,
        };

        assert_eq!(timestamp, 1639008000);
    }
}
