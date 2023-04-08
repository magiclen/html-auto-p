use std::{fs::File, io::Read, path::Path};

use html_auto_p::*;

const DATA_FOLDER: &str = "data";

#[test]
fn test_files_in_the_data_folder() {
    let data_folder = Path::new("tests").join(DATA_FOLDER);

    for dir in data_folder.read_dir().unwrap().map(|dir| dir.unwrap()) {
        if dir.file_type().unwrap().is_file() {
            let file_path = dir.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            if let Some(file_name) = file_name.strip_suffix(".test.html") {
                let html_file_path = &file_path;
                let autoped_file_path =
                    html_file_path.parent().unwrap().join(format!("{}.autoped.html", file_name));

                let mut html_file = File::open(html_file_path).unwrap();
                let mut html = String::new();

                html_file.read_to_string(&mut html).unwrap();

                let autoped_html = auto_p(
                    html,
                    Options::new().br(true).esc_pre(true).remove_useless_newlines_in_pre(true),
                );

                let mut autoped_file = File::open(autoped_file_path.as_path()).unwrap();
                let mut autoped_file_content = String::new();

                autoped_file.read_to_string(&mut autoped_file_content).unwrap();

                if autoped_file_content.trim().ne(&autoped_html) {
                    eprintln!("{}", autoped_html);
                    panic!(
                        "The `auto_p`ed html does not match the `auto_p`ed file: {}",
                        autoped_file_path.to_str().unwrap()
                    );
                }
            }
        }
    }
}
