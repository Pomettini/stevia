extern crate regex;

// Priority:
// TODO: Document the format
// Secondary:
// TODO: Add a way to load/manage/change backgrounds
// TODO: Add a way to test all the branches automatically
// TODO: Add a test executable (GGEZ?)
// TODO: Export the .h file for GBA
// TODO: Implement jumps
// TODO: Implement multi line comments

use reader::{LineType, Reader};
use writer::Writer;

pub mod reader;
pub mod writer;