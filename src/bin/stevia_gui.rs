#[rustfmt::skip]
extern crate stevia;
#[rustfmt::skip]
extern crate iui;
#[rustfmt::skip]
use iui::prelude::*;
#[rustfmt::skip]
use iui::controls::*;
#[rustfmt::skip]
use std::path::PathBuf;
#[rustfmt::skip]
use stevia::reader::*;
#[rustfmt::skip]
use stevia::writer::*;
#[rustfmt::skip]
use std::fs::File;
#[rustfmt::skip]
use std::io::prelude::*;
#[rustfmt::skip]
use std::fs::read_to_string;
#[rustfmt::skip]
use std::cell::RefCell;
#[rustfmt::skip]
use std::rc::Rc;

#[derive(Clone)]
struct LogContext<'a> {
    ui: &'a UI,
    entry: MultilineEntry,
}

#[derive(PartialEq)]
enum ExportFormat {
    Stevia,
    Epub,
}

fn main() {
    // Wrapped with Interior Mutability Pattern
    // Because I need to pass the state around between UI controls
    let export_format: Rc<RefCell<Option<ExportFormat>>> = Rc::new(RefCell::new(None));

    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(&ui, "Stevia GUI", 320, 480, WindowType::NoMenubar);

    let mut multiline_entry = MultilineEntry::new(&ui);
    let mut log_ctx = LogContext {
        ui: &ui,
        entry: multiline_entry.clone(),
    };

    let mut program_vbox = VerticalBox::new(&ui);
    program_vbox.set_padded(&ui, true);

    let mut button = Button::new(&ui, "Load Ink File");
    button.on_clicked(&ui, {
        let ui = ui.clone();
        let win = win.clone();
        let export_format = export_format.clone();
        move |_| {
            if &*export_format.borrow() == &None {
                win.modal_err(&ui, "Warning", "Please select an export file format");
                return;
            }

            let file = match win.open_file(&ui) {
                Some(f) => f,
                None => return,
            };

            process(&mut log_ctx, &*export_format.borrow(), file);
        }
    });
    program_vbox.append(&ui, button, LayoutStrategy::Compact);

    let mut file_format_cb = Combobox::new(&ui);
    file_format_cb.append(&ui, "Select export file format");
    file_format_cb.append(&ui, "Stevia");
    file_format_cb.append(&ui, "ePub");
    file_format_cb.set_selected(&ui, 0);
    file_format_cb.clone().on_selected(&ui, {
        move |index| {
            match index {
                // TODO: Must refactor
                0 => *export_format.borrow_mut() = None,
                1 => *export_format.borrow_mut() = Some(ExportFormat::Stevia),
                2 => *export_format.borrow_mut() = Some(ExportFormat::Epub),
                _ => *export_format.borrow_mut() = None,
            }
        }
    });

    program_vbox.append(&ui, file_format_cb, LayoutStrategy::Compact);

    program_vbox.append(&ui, HorizontalSeparator::new(&ui), LayoutStrategy::Compact);

    program_vbox.append(&ui, multiline_entry, LayoutStrategy::Stretchy);

    win.set_child(&ui, program_vbox);
    win.show(&ui);
    ui.main();
}

fn process(ctx: &mut LogContext, export_format: &Option<ExportFormat>, path: PathBuf) {
    clear_log(ctx);

    let file = File::open(path.clone());
    let mut file = match file {
        Ok(f) => {
            log(ctx, "File loaded");
            f
        }
        Err(_) => {
            log(ctx, "Cannot load the file");
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            log(ctx, "File read");
        }
        Err(_) => {
            log(ctx, "Cannot read the file");
            return;
        }
    };

    log(ctx, "Started parsing");

    let mut reader = Reader::from_text(&contents);
    reader.parse_all_lines();

    log(ctx, "Completed parsing");

    match export_format {
        None => (),
        Some(ExportFormat::Stevia) => {
            log(ctx, "Started exporting to Stevia");

            let mut writer = Writer::new();
            writer.process_lines(&reader);

            // TODO: Needs refactor urgently
            let file_name = path.file_stem().unwrap().to_str().unwrap();

            let mut output_file = match File::create(format!("{}.stevia", &file_name)) {
                Ok(file) => {
                    log(ctx, "Created output file");
                    file
                }
                Err(_) => {
                    log(ctx, "Cannot create output file");
                    return;
                }
            };

            match output_file.write_all(writer.output.as_bytes()) {
                Ok(_) => {
                    log(ctx, "Written to Stevia file");
                }
                Err(_) => {
                    log(ctx, "Cannot write to Stevia file");
                    return;
                }
            }

            log(ctx, "Stevia exporting completed");
        }
        Some(ExportFormat::Epub) => {
            log(ctx, "Started exporting to ePub");

            log(ctx, "Needs to be implemented!");

            // let file_name = path.file_stem().unwrap().to_str().unwrap();

            // let mut epub_writer = EpubWriter::new("I love Rust", "Pomettini", "examples/cover.jpg");
            // epub_writer.process_lines(&reader);
            // let epub = epub_writer.generate();

            // if let Some(contents) = epub {
            //     let mut file = File::create(format!("{}.epub", file_name)).unwrap();
            //     file.write_all(&contents).unwrap();

        }
    }
}

fn log(ctx: &mut LogContext, message: &str) {
    let mut content = ctx.entry.value(&ctx.ui);
    content.push_str(&message);
    content.push_str("\n");
    ctx.entry.set_value(&ctx.ui, &content);
}

fn clear_log(ctx: &mut LogContext) {
    ctx.entry.set_value(&ctx.ui, "");
}