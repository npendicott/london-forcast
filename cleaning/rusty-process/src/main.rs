extern crate csv;
extern crate chrono;
extern crate serde;

//use std::error::Error;
//use std::fs;
//use std::io;
//use std::process;
use std::path::Path;


mod parquet_methods;
//use parquet_methods;

mod schemas;
use schemas::{HHourlyReading};


fn main() {
    //block_0.csv
    let input_path_prefix = "/home/nathaniel/LondonForcast/data/raw/halfhourly_dataset/";
    let output_path_prefix = "/home/nathaniel/LondonForcast/data/cleaned/";

    for file_num in 0..1 {
        let file_name = format!("block_{}.csv", file_num);
        println!("Processing file: {}", file_name);

        // Get input path
        let input_path_str = format!("{}{}", input_path_prefix, file_name);
        let input_path = Path::new(&input_path_str);
        println!("Input: {}", input_path_str);

        // Get output path
        let output_path_str = format!("{}{}", output_path_prefix, file_name);
        let output_path = Path::new(&output_path_str);
        println!("Output: {}", output_path_str);

        process_file(&input_path, &output_path);

        println!();
    }
    
    // TODO: Parquet?
    // println!("Reading {:?}", sample_path);
    // process_file(sample_path);

    // parquet_methods::write_test_file();
    // parquet_methods::read_test_file();
}

fn process_file(input_path: &Path, output_path: &Path) {
    // Get the reader
    let mut input_reader;  // Is this the right way to declare things?
    match csv::Reader::from_path(input_path) {
        Ok(result) => input_reader = result,
        Err(err) => panic!("Couldn't find {:?}:\n{:?}", input_path, err),
    };

    // Get the writer
    let mut output_writer;  // Is this the right way to declare things?
    match csv::Writer::from_path(output_path) {
        Ok(result) => output_writer = result,
        Err(err) => panic!("Couldn't find {:?}:\n{:?}", output_path, err),
    };

    // Run through it
    for result in input_reader.deserialize() {
        // Pull a record
        let record: HHourlyReading;
        match result {
            Ok(result) => record = result,
            Err(err) =>  panic!("Error reading record:\n{:?}", err),
        };

        // record.print();

        // Filtering?
        // match record.energy_kwh_hh {
        //     Some(val) => output_writer.serialize(record),
        //     None => output_writer.serialize(record),
        // };

        output_writer.serialize(record);
    }

}
