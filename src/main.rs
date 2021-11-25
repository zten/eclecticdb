use std::{fmt, fs};
use std::fmt::Formatter;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use rand::Rng;

struct BlockHeader {
    element_length: u16,
    blocks: Vec<BlockInfo>
}

struct BlockInfo {
    min_value: u64,
    max_value: u64,
    starting_offset: u64,
    block_length: u64,
    row_id_start: u64,
    row_id_end: u64,
    rows: u16
}

struct Block<T> {
    rows: Vec<T>
}

struct Row {
    null: bool,
    bytes: Vec<u8>
}

struct Rows(pub Vec<Row>);

struct Column {
    name: String,
    rows: Rows
}

struct Columns(pub Vec<Column>);

struct Table {
    name: String,
    columns: Columns
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Table(name: {}, columns: ({}))", self.name, self.columns)
    }
}

impl fmt::Display for Columns {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, column| {
            result.and_then(|_| write!(f, "{},", column))
        })
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Column(name: {}, rows: {})", self.name, self.rows.0.len())
    }
}

fn main() {
    println!("Hello, world!");

    let table = make_table("test_table");
    println!("{}", table)
}

fn write_bytes() {
    let path = Path::new("test");
    let file = OpenOptions::new().write(true)
        .create(true)
        .open(path)
        .unwrap();

    let mut bytes = vec![1, 2, 3, 4];
    let mut writer = BufWriter::new(file);
    writer.write(&bytes);
}

fn make_table(name: &str) -> Table {
    let mut columns: Vec<Column> = vec![];
    columns.push(make_id_column("id"));
    columns.push(make_column("column1"));
    columns.push(make_column("column2"));
    return Table {
        name: String::from(name),
        columns: Columns(columns)
    }
}

fn make_column(name: &str) -> Column {
    let mut rng = rand::thread_rng();

    let mut rows: Vec<Row> = vec![];
    for _ in 1..11 {
        let value: u32 = rng.gen_range(0..10000);
        rows.push(Row {
            null: false,
            bytes: Vec::from(value.to_be_bytes())
        })
    }

    return Column {
        name: String::from(name),
        rows: Rows(rows)
    }
}

fn make_id_column(name: &str) -> Column {
    let mut rows: Vec<Row> = vec![];
    for i in 1u32..11 {
        rows.push(Row {
            null: false,
            bytes: Vec::from(i.to_be_bytes())
        })
    }

    return Column {
        name: String::from(name),
        rows: Rows(rows)
    }
}

fn write_table(table: &Table) {

}