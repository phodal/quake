use std::error::Error;
use std::fs::File;

use csv::Reader;
use json::{array, object, JsonValue};

pub fn csv_to_json(rdr: &mut Reader<File>) -> Result<JsonValue, Box<dyn Error>> {
    let mut json: JsonValue = array![];

    let mut header = vec![];
    for record in rdr.headers() {
        for str in record {
            header.push(String::from(str))
        }
    }

    for result in rdr.records() {
        let mut element = object! {};
        let record = result.unwrap();
        for (index, str) in record.iter().enumerate() {
            element[header[index].clone()] = str.into();
        }

        json.push(element.clone())?;
    }

    Ok(json)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;

    use crate::helper::csv_to_json::csv_to_json;

    #[test]
    fn json_to_csv() {
        let path = PathBuf::from("_fixtures").join("todo").join("entries.csv");
        let mut rdr = csv::Reader::from_reader(File::open(path).expect("cannot open file"));
        let output = csv_to_json(&mut rdr).unwrap();
        assert_eq!(output.to_string(), "[{\"id\":\"1\",\"title\":\"time support\",\"author\":\"\",\"content\":\"\",\"created_date\":\"2021-11-24 19:14:10\",\"updated_date\":\"2021-11-24 19:14:10\"}]")
    }
}
