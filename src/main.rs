extern crate argparse;
extern crate rayon;
extern crate serde_json;
extern crate time;

extern crate mongodb;
use mongodb::coll::options::WriteModel;
use mongodb::coll::Collection;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, Bson};
use mongodb::{Client, ThreadedClient};

use serde_json::Value;

use argparse::{ArgumentParser, Collect, Store, StoreTrue};
use itertools::Itertools;
use rayon::prelude::*;

use std::io::*;

mod file;

#[derive(Default, Debug)]
pub struct Options {
    pub verbose: bool,
    pub show_errors: bool,
    pub host: String,
    pub database: String,
    pub collection: String,
    pub paths: Vec<String>,
}

pub fn import_stream(stream: Box<BufRead>, opts: &Options, coll: &Collection) {
    let mut lineno = 0;

    let chunks = stream
        .lines()
        .filter_map(|line| {
            if opts.verbose {
                lineno += 1;
                if lineno % 100_000 == 0 {
                    print!(".");
                    stdout().flush().unwrap();
                }
            }

            match line {
                Ok(l) => parse_line(l, opts),
                Err(e) => {
                    if opts.verbose {
                        eprintln!("Failed to read line:\n  {}", e);
                    }
                    None
                }
            }
        })
        .chunks(10_000);

    for times in chunks.into_iter() {
        let models = times
            .collect::<Vec<Bson>>()
            .iter()
            .map(|document| WriteModel::InsertOne {
                document: document.clone().as_document().unwrap().clone(),
            })
            .collect();

        coll.bulk_write(models, false);
    }

    if opts.verbose {
        println!("");
    }
}

pub fn parse_line(line: String, opts: &Options) -> Option<Bson> {
    let json_value = serde_json::from_str(&line);

    if json_value.is_err() {
        if opts.show_errors {
            eprintln!("Failed processing line:\n  {}", line);
        }
        return None;
    }

    let value: Value = json_value.unwrap();

    let bson_value = bson::to_bson(&value);

    if bson_value.is_err() {
        if opts.show_errors {
            eprintln!("Failed converting to BSON object:\n  {}", line);
        }
        return None;
    }

    return Some(bson_value.unwrap());
}

#[inline]
pub fn import_file(path: &str, opts: &Options, coll: &Collection) {
    let file_stream = file::reader(&path, &opts);
    import_stream(file_stream, &opts, &coll)
}

fn main() {
    let mut opts: Options = Default::default();
    opts.paths = ["test/sample_10.log".to_string()].to_vec();
    opts.host = "localhost".to_string();
    opts.database = "json".to_string();
    opts.collection = "logs".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Import JSON log file into MongoDB.");
        ap.refer(&mut opts.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut opts.show_errors).add_option(
            &["--errors"],
            StoreTrue,
            "Output into stderr lines with invalid JSON",
        );
        ap.refer(&mut opts.host)
            .add_option(&["--host"], Store, "MongoDB host");
        ap.refer(&mut opts.database)
            .add_option(&["--database"], Store, "MongoDB database");
        ap.refer(&mut opts.collection)
            .add_option(&["--collection"], Store, "MongoDB collection");
        ap.refer(&mut opts.paths).add_argument(
            "FILE",
            Collect,
            "Paths to the log file(s) to process",
        );
        ap.parse_args_or_exit();
    }

    let client =
        Client::connect(&opts.host, 27017).expect("Failed to initialize standalone client.");

    let coll = client.db(&opts.database).collection(&opts.collection);
    coll.drop().unwrap();

    opts.paths
        .par_iter()
        .for_each(|path| import_file(&path, &opts, &coll));
}
