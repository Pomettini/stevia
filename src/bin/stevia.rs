extern crate assert_cmd;
extern crate clap;
extern crate stevia;

use clap::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use stevia::reader::Reader;
use stevia::writer::Writer;

fn main() {
    let matches = App::new("stevia")
        .version("0.1")
        .author("Giorgio Pomettini <giorgio.pomettini@gmail.com>")
        .arg(Arg::with_name("file").index(1).required(true))
        .get_matches();

    let path = Path::new(matches.value_of("file").expect("Missing file argument"));

    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot read the file");

    let mut reader = Reader::from_text(&contents);
    reader.parse_all_lines();

    let mut writer = Writer::new();
    writer.process_lines(&reader);

    // TODO: Needs refactor urgently
    let mut output_file = File::create(format!(
        "{}.stevia",
        path.file_stem().unwrap().to_str().unwrap()
    ))
    .expect("Cannot create output file");

    output_file
        .write_all(writer.output.as_bytes())
        .expect("Cannot write file content");
}

mod tests {
    #[allow(unused_imports)]
    use assert_cmd::prelude::*;
    #[allow(unused_imports)]
    use std::fs::*;
    use std::process::Command;

    #[test]
    fn test_load_no_argument() {
        Command::cargo_bin("stevia").unwrap().assert().failure();

        clean();
    }

    #[test]
    fn test_load_file() {
        Command::cargo_bin("stevia")
            .unwrap()
            .arg("examples/example.ink")
            .assert()
            .success();

        clean();
    }

    #[test]
    fn test_load_non_existent_file() {
        Command::cargo_bin("stevia")
            .unwrap()
            .arg("examples/nonexistent.ink")
            .assert()
            .failure();

        clean();
    }

    #[test]
    fn test_functional_process_file_green() {
        Command::cargo_bin("stevia")
            .unwrap()
            .arg("examples/example.ink")
            .assert()
            .success();

        // Check contents of output file
        let expected = "P;Hello there|P;I'm a VN written in the Ink format|P;Do you like it?|Q;Yes, I like it!;00120;No, I do not like it;00136|P;Thank you!|E;|P;Oh, I see|E;";
        let contents = read_to_string("example.stevia").expect("Cannot find .stevia file");

        assert_eq!(expected, contents);

        clean();
    }

    #[allow(dead_code)]
    fn clean() {
        Command::new("find")
            .arg(".")
            .arg("-name")
            .arg("*.stevia")
            .arg("-delete")
            .output()
            .expect("Clean command failed");
    }
}
