use std::fs::File;

use comfy_table::Table;
use csv::Reader;

pub fn csv_to_table(rdr: &mut Reader<File>) -> Table {
    let mut table = Table::new();

    let mut header = vec![];
    header.push("id".to_string());
    for record in rdr.headers() {
        for str in record {
            header.push(String::from(str))
        }
    }

    table.set_header(header);

    let mut index = 1;
    for result in rdr.records() {
        let record = result.unwrap();
        let mut row = vec![];
        row.push(index.to_string());
        for str in &record {
            row.push(String::from(str));
        }
        index = index + 1;
        table.add_row(row);
    }
    table
}
