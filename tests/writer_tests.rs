extern crate stevia;

use stevia::reader::*;
use stevia::writer::*;

#[allow(unused_macros)]
macro_rules! SETUP_WRITER {
    ($input:expr, $reader:ident, $writer:ident) => {
        let input = $input;
        let mut $reader = Reader::from_text(input);
        $reader.parse_all_lines();

        let mut $writer = Writer::new();
        $writer.process_lines(&$reader);
    };
}

#[allow(unused_macros)]
macro_rules! SETUP_SYMBOLS {
    ($name:expr, $address:expr, $writer:ident) => {
        $writer.symbols.insert($name, $address);
    };
}

// --- TEXT ---

#[test]
fn test_writer_print_one() {
    SETUP_WRITER!("Hello world", reader, writer);

    assert_eq!(writer.output, "P;Hello world");

    assert_eq!(writer.index, 13);
}

#[test]
fn test_writer_print_two() {
    SETUP_WRITER!(
        r#"Hello world
Ciao mondo"#,
        reader,
        writer
    );

    assert_eq!(writer.output, "P;Hello world|P;Ciao mondo");

    assert_eq!(writer.index, 26);
}

#[test]
fn test_writer_print_three() {
    SETUP_WRITER!(
        r#"Hello world
Ciao mondo
Bonjour monde"#,
        reader,
        writer
    );

    assert_eq!(writer.output, "P;Hello world|P;Ciao mondo|P;Bonjour monde");

    assert_eq!(writer.index, 42);
}

// --- QUESTIONS ---

#[test]
fn test_writer_question_fake_jump_one() {
    SETUP_WRITER!("+ [Hello world] -> example", reader, writer);

    SETUP_SYMBOLS!(String::from("example"), 0, writer);

    assert_eq!(writer.output, "Q;Hello world;00000");

    assert_eq!(writer.branch_table["example"], vec![14]);
}

#[test]
fn test_writer_question_fake_jump_two() {
    SETUP_WRITER!(
        "+ [Hello world] -> example
+ [Ciao mondo] -> sample",
        reader,
        writer
    );

    assert_eq!(writer.output, "Q;Hello world;00000;Ciao mondo;00000");

    SETUP_SYMBOLS!(String::from("example"), 0, writer);
    SETUP_SYMBOLS!(String::from("sample"), 0, writer);

    assert_eq!(writer.branch_table["example"], vec![14]);
    assert_eq!(writer.branch_table["sample"], vec![31]);
}

#[test]
fn test_writer_question_fake_jump_and_print() {
    SETUP_WRITER!(
        "+ [Hello world] -> example
+ [Ciao mondo] -> sample
Bonjour monde",
        reader,
        writer
    );

    assert_eq!(
        writer.output,
        "Q;Hello world;00000;Ciao mondo;00000|P;Bonjour monde"
    );

    SETUP_SYMBOLS!(String::from("example"), 0, writer);
    SETUP_SYMBOLS!(String::from("sample"), 0, writer);

    assert_eq!(writer.branch_table["example"], vec![14]);
    assert_eq!(writer.branch_table["sample"], vec![31]);
}

#[test]
fn test_writer_question_fake_jump_multiple() {
    SETUP_WRITER!(
        "+ [Hello world] -> example
+ [Ciao mondo] -> sample
Bonjour monde
+ [Hello world] -> example
+ [Ciao mondo] -> sample
",
        reader,
        writer
    );

    assert_eq!(
        writer.output,
        "Q;Hello world;00000;Ciao mondo;00000|P;Bonjour monde|Q;Hello world;00000;Ciao mondo;00000"
    );

    SETUP_SYMBOLS!(String::from("example"), 0, writer);
    SETUP_SYMBOLS!(String::from("sample"), 0, writer);

    assert_eq!(writer.index, 89);

    assert_eq!(writer.branch_table["example"], vec![14, 67]);
    assert_eq!(writer.branch_table["sample"], vec![31, 84]);
}

#[test]
fn test_writer_question_one() {
    SETUP_WRITER!(
        "+ [Hello world] -> example
+ [Ciao mondo] -> sample
=== example
Hello world
=== sample
Ciao mondo
",
        reader,
        writer
    );

    assert_eq!(
        writer.output,
        "Q;Hello world;00037;Ciao mondo;00051|P;Hello world|P;Ciao mondo"
    );

    assert_eq!(writer.index, 63);

    assert_eq!(writer.branch_table["example"], vec![14]);
    assert_eq!(writer.branch_table["sample"], vec![31]);

    assert_eq!(writer.symbols["example"], 37);
    assert_eq!(writer.symbols["sample"], 51);
}

// --- END ---

#[test]
fn test_writer_end_one() {
    SETUP_WRITER!("-> END", reader, writer);

    assert_eq!(writer.index, 2);

    assert_eq!(writer.output, "E;");
}

#[test]
fn test_writer_end_two() {
    SETUP_WRITER!(
        "Hello world
-> END",
        reader,
        writer
    );

    assert_eq!(writer.output, "P;Hello world|E;");

    assert_eq!(writer.index, 16);
}

// --- BOOKMARKS ---

#[test]
fn test_writer_bookmark_position_zero_one() {
    SETUP_WRITER!("=== hello", reader, writer);

    assert_eq!(writer.index, 0);

    assert_eq!(writer.symbols["hello"], 0);
}

#[test]
fn test_writer_bookmark_position_zero_one_spaces() {
    SETUP_WRITER!(" ===  hello", reader, writer);

    assert_eq!(writer.index, 0);

    assert_eq!(writer.symbols["hello"], 0);
}

#[test]
fn test_writer_bookmark_position_zero_two() {
    SETUP_WRITER!(
        "=== hello
=== world",
        reader,
        writer
    );

    assert_eq!(writer.index, 0);

    assert_eq!(writer.symbols["hello"], 0);
    assert_eq!(writer.symbols["world"], 0);
}

#[test]
fn test_writer_bookmark_position_zero_two_spaces() {
    SETUP_WRITER!(
        "   ===     hello
    ===     world",
        reader,
        writer
    );

    assert_eq!(writer.index, 0);

    assert_eq!(writer.symbols["hello"], 0);
    assert_eq!(writer.symbols["world"], 0);
}

#[test]
fn test_writer_bookmark_one() {
    SETUP_WRITER!(
        "Hello world
=== hello
Ciao mondo",
        reader,
        writer
    );

    assert_eq!(writer.output, "P;Hello world|P;Ciao mondo");

    assert_eq!(writer.index, 26);

    assert_eq!(writer.symbols["hello"], 14);
}

#[test]
fn test_writer_bookmark_two() {
    SETUP_WRITER!(
        "Hello world
=== hello
Ciao mondo
=== world
Bonjour monde",
        reader,
        writer
    );

    assert_eq!(writer.output, "P;Hello world|P;Ciao mondo|P;Bonjour monde");

    assert_eq!(writer.index, 42);

    assert_eq!(writer.symbols["hello"], 14);
    assert_eq!(writer.symbols["world"], 27);
}

// --- CONSTANTS ---

#[test]
fn test_writer_declare_constants_one() {
    SETUP_WRITER!("CONST HELLO = \"World\"", reader, writer);

    assert_eq!(writer.index, 0);

    assert_eq!(reader.lines[0].type_, LineType::Constant);

    assert_eq!(writer.constants["HELLO"], "World");
}

#[test]
fn test_writer_declare_constants_two() {
    SETUP_WRITER!(
        "CONST HELLO = \"World\"
CONST CIAO = \"Mondo\"",
        reader,
        writer
    );

    assert_eq!(writer.index, 0);

    assert_eq!(reader.lines[0].type_, LineType::Constant);
    assert_eq!(reader.lines[1].type_, LineType::Constant);

    assert_eq!(writer.constants["HELLO"], "World");
    assert_eq!(writer.constants["CIAO"], "Mondo");
}

#[test]
fn test_writer_declare_constants_one_space() {
    SETUP_WRITER!("CONST  HELLO  =  \"World\"", reader, writer);

    assert_eq!(writer.index, 0);

    assert_eq!(reader.lines[0].type_, LineType::Constant);

    assert_eq!(writer.constants["HELLO"], "World");
}

#[test]
fn test_writer_declare_constants_two_space() {
    SETUP_WRITER!(" CONST  HELLO  =  \"World\"", reader, writer);

    assert_eq!(writer.index, 0);

    assert_eq!(reader.lines[0].type_, LineType::Constant);

    assert_eq!(writer.constants["HELLO"], "World");
}

#[test]
fn test_writer_declare_constants_two_space_multiple() {
    SETUP_WRITER!(" CONST  HELLO  =  \" World \"", reader, writer);

    assert_eq!(writer.index, 0);

    assert_eq!(reader.lines[0].type_, LineType::Constant);

    assert_eq!(writer.constants["HELLO"], "World");
}

#[test]
fn test_writer_constants_one() {
    SETUP_WRITER!(
        "CONST HELLO = \"World\"
Hello {HELLO}",
        reader,
        writer
    );

    assert_eq!(writer.output, "P;Hello World");

    assert_eq!(writer.index, 13);

    assert_eq!(reader.lines[0].type_, LineType::Constant);
    assert_eq!(reader.lines[1].type_, LineType::Text);

    assert_eq!(writer.constants["HELLO"], "World");
}

#[test]
fn test_writer_constants_two() {
    SETUP_WRITER!(
        "CONST HELLO = \"World\"
CONST CIAO = \"Mondo\"
Hello {HELLO} Ciao {CIAO}",
        reader,
        writer
    );

    assert_eq!(writer.output, "P;Hello World Ciao Mondo");

    assert_eq!(writer.index, 24);

    assert_eq!(reader.lines[0].type_, LineType::Constant);
    assert_eq!(reader.lines[1].type_, LineType::Constant);
    assert_eq!(reader.lines[2].type_, LineType::Text);

    assert_eq!(writer.constants["HELLO"], "World");
    assert_eq!(writer.constants["CIAO"], "Mondo");
}

// --- COMMENTS ---

#[test]
fn test_writer_comment_one() {
    SETUP_WRITER!("// Hello world", reader, writer);

    assert_eq!(writer.output, "");

    assert_eq!(writer.index, 0);

    assert_eq!(reader.lines[0].type_, LineType::Comment);
}

#[test]
fn test_writer_comment_two() {
    SETUP_WRITER!(
        "// Hello world
// Ciao mondo",
        reader,
        writer
    );

    assert_eq!(writer.output, "");

    assert_eq!(writer.index, 0);

    assert_eq!(reader.lines[0].type_, LineType::Comment);
    assert_eq!(reader.lines[1].type_, LineType::Comment);
}

#[test]
fn test_writer_comment_and_text() {
    SETUP_WRITER!(
        "// Hello world
// Ciao mondo
Bonjour monde",
        reader,
        writer
    );

    assert_eq!(writer.output, "P;Bonjour monde");

    assert_eq!(writer.index, 15);

    assert_eq!(reader.lines[0].type_, LineType::Comment);
    assert_eq!(reader.lines[1].type_, LineType::Comment);
    assert_eq!(reader.lines[2].type_, LineType::Text);
}

// --- FUNCTIONAL TESTS ---

#[test]
fn functional_test_one() {
    SETUP_WRITER!(
        "Hello there

I'm a VN written in the Ink format

Do you like it?

-> END",
        reader,
        writer
    );

    assert_eq!(
        writer.output,
        "P;Hello there|P;I'm a VN written in the Ink format|P;Do you like it?|E;"
    );

    assert_eq!(writer.index, 71);

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
    assert_eq!(reader.lines[2].type_, LineType::Text);
    assert_eq!(reader.lines[3].type_, LineType::End);
}

#[test]
fn functional_test_two() {
    SETUP_WRITER!(
        "Hello there

I'm a VN written in the Ink format

Do you like it?

+ [Yes, I like it!] -> like
+ [No, I do not like it] -> hate

=== like

Thank you!

-> END

=== hate

Oh, I see

-> END",
        reader,
        writer
    );

    assert_eq!(writer.output, "P;Hello there|P;I'm a VN written in the Ink format|P;Do you like it?|Q;Yes, I like it!;00120;No, I do not like it;00136|P;Thank you!|E;|P;Oh, I see|E;");

    assert_eq!(writer.index, 150);

    assert_eq!(writer.symbols["like"], 120);
    assert_eq!(writer.symbols["hate"], 136);

    assert_eq!(writer.branch_table["like"], vec![87]);
    assert_eq!(writer.branch_table["hate"], vec![114]);

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
    assert_eq!(reader.lines[2].type_, LineType::Text);
    assert_eq!(reader.lines[3].type_, LineType::Question);
    assert_eq!(reader.lines[4].type_, LineType::Question);
    assert_eq!(reader.lines[5].type_, LineType::Bookmark);
    assert_eq!(reader.lines[6].type_, LineType::Text);
    assert_eq!(reader.lines[7].type_, LineType::End);
    assert_eq!(reader.lines[8].type_, LineType::Bookmark);
    assert_eq!(reader.lines[9].type_, LineType::Text);
    assert_eq!(reader.lines[10].type_, LineType::End);
}
