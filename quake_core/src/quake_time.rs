use chrono::{DateTime, Local};

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub fn date_now() -> String {
    let local: DateTime<Local> = Local::now();
    local.format(FORMAT).to_string()
}
