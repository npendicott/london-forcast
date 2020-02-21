use std::{fs, path::Path, rc::Rc};

extern crate parquet;
use parquet::{
    column::{reader::ColumnReader, writer::ColumnWriter},
    file::{
        properties::WriterProperties,
        reader::{FileReader, SerializedFileReader},
        writer::{FileWriter, SerializedFileWriter},
    },
    schema::parser::parse_message_type,
};

// // File
// pub fn write_test_file() {
//     let sample_path = Path::new("./input/live/test_out.pq");

//     let message_type = "
//     message schema {
//         optional group values (LIST) {
//         repeated group list {
//             optional INT32 element;
//         }
//         }
//     }
//     ";

//     // let message_type = "
//     // message schema {
//     //     REQUIRED INT32 b;
//     // }
//     // ";
//     let schema = Rc::new(parse_message_type(message_type).unwrap());
//     let props = Rc::new(WriterProperties::builder().build());
//     let file = fs::File::create(&sample_path).unwrap();

//     let mut writer = SerializedFileWriter::new(file, schema, props).unwrap();

//     let mut row_group_writer = writer.next_row_group().unwrap();
//     while let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
//         // ... write values to a column writer
//         match col_writer {
//             // You can also use `get_typed_column_writer` method to extract typed writer.
//             ColumnWriter::Int32ColumnWriter(ref mut typed_writer) => {
//                 typed_writer
//                     .write_batch(&[1, 2, 3], Some(&[3, 3, 3, 2, 2]), Some(&[0, 1, 0, 1, 1]))
//                     .unwrap();
//             }
//             _ => {}
//         }
//         row_group_writer.close_column(col_writer).unwrap();
//     }
//     writer.close_row_group(row_group_writer).unwrap();
//     writer.close().unwrap();

// }


// Column level
pub fn write_test_file() {
    // Schema
    let message_type = "
    message schema {
        optional group values (LIST) {
        repeated group list {
            optional INT32 element;
        }
        }
    }
    ";
    let schema = Rc::new(parse_message_type(message_type).unwrap());

    let props = Rc::new(WriterProperties::builder().build());

    // Open file
    let sample_path = Path::new("./input/live/test_out.pq");
    let file = fs::File::create(sample_path).unwrap();


    // Writer
    let mut writer = SerializedFileWriter::new(file, schema, props).unwrap();
    let mut row_group_writer = writer.next_row_group().unwrap();
    while let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        match col_writer {
            // You can also use `get_typed_column_writer` method to extract typed writer.
            ColumnWriter::Int32ColumnWriter(ref mut typed_writer) => {
                typed_writer
                    .write_batch(&[1, 2, 3], Some(&[3, 3, 3, 2, 2]), Some(&[0, 1, 0, 1, 1]))
                    .unwrap();
            }
            _ => {}
        }
        row_group_writer.close_column(col_writer).unwrap();
    }
    writer.close_row_group(row_group_writer).unwrap();
    writer.close().unwrap();
    println!("Sup");
}

pub fn read_test_file() {
    let sample_path = Path::new("./input/live/test_out.pq");
    let file = fs::File::open(sample_path).unwrap();
    let reader = SerializedFileReader::new(file).unwrap();
    let mut iter = reader.get_row_iter(None).unwrap();
    while let Some(record) = iter.next() {
        println!("{}", record);
    }
}


// pub fn read_test_file() {
//     let sample_path = Path::new("./input/live/test_out.pq");

//     let file = fs::File::open(sample_path).unwrap();
//     let reader = SerializedFileReader::new(file).unwrap();
//     let metadata = reader.metadata();

//     let mut res = Ok((0, 0));
//     let mut values = vec![0; 8];
//     let mut def_levels = vec![0; 8];
//     let mut rep_levels = vec![0; 8];

//     for i in 0..metadata.num_row_groups() {
//         let row_group_reader = reader.get_row_group(i).unwrap();
//         let row_group_metadata = metadata.row_group(i);

//         for j in 0..row_group_metadata.num_columns() {
//             let mut column_reader = row_group_reader.get_column_reader(j).unwrap();
//             match column_reader {
//                 // You can also use `get_typed_column_reader` method to extract typed reader.
//                 ColumnReader::Int32ColumnReader(ref mut typed_reader) => {
//                     res = typed_reader.read_batch(
//                         8, // batch size
//                         Some(&mut def_levels),
//                         Some(&mut rep_levels),
//                         &mut values,
//                     );
//                 }
//                 _ => {}
//             }
//         }
//     }

//     for item in values {
//         println!("{:?}", item);
        
//     }
//     println!();

//     for item in def_levels {
//         println!("{:?}", item);
        
//     }
//     println!();

//     for item in rep_levels {
//         println!("{:?}", item);
        
//     }
//     println!();

// }
