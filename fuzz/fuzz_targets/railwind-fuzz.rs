#![no_main]
use libfuzzer_sys::fuzz_target;
use railwind::*;
use std::str;
use std::io::Write;
use std::path::Path;

fuzz_target!(|data: &[u8]| {
    match str::from_utf8(data) {
        Ok(in_string) => {
            let mut file = std::fs::File::create("data");
            match file {
                Ok(mut in_file) => {
                    match in_file.write_all(in_string.as_bytes()) {
                        Ok(..) => {
                            railwind::parse_html_to_file(Path::new("data"), Path::new("out"), false);
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        },
        Err(..) => ()
    }
    
});