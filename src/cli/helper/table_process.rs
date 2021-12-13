use std::fs::File;

use comfy_table::Table;
use csv::Reader;

pub fn csv_to_terminal_table(rdr: &mut Reader<File>) -> Table {
    let mut table = Table::new();

    let mut header = vec![];
    if let Ok(record) = rdr.headers() {
        for str in record {
            header.push(String::from(str))
        }
    }

    table.set_header(header);

    for result in rdr.records() {
        let record = result.unwrap();
        let mut row = vec![];
        for str in &record {
            row.push(String::from(str));
        }
        table.add_row(row);
    }

    table
}
