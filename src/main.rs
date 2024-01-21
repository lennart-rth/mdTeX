use std::env;
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use std::borrow::Cow;

mod lexer;
mod config;
mod parser;

use lexer::lex;
use config::USEPACKAGES;
use parser::parse;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <./path/filename.md>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    println!("Filename used: {}", filename);

    let input_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading {}: {}", filename, err);
            std::process::exit(1);
        }
    };

    let lexed_markdown = lex(&input_contents);

    let mut latex_output = parse(lexed_markdown);

    if unsafe{USEPACKAGES.cancle} {
        latex_output.insert(1,"\\usepackage{cancel}\n\n".to_string());
    }

    latex_output.push("\\end{document}".to_string());

    let out = latex_output.join("").to_string();


    let mut path_buf = std::path::PathBuf::from(filename);

    // Change the file extension from "md" to "tex"
    if let Some(_ext) = path_buf.extension() {
        path_buf.set_extension("tex");
    }

    println!("Generating {:?}", &path_buf);

    fs::write(&path_buf, out).expect("Unable to write file");

    let pdflatex_command = "pdflatex";
    let file_to_process = &path_buf;

    let mut cmd = Command::new(pdflatex_command);
    cmd.current_dir(env::current_dir().expect("Unable to get current directory"));
    cmd.arg(file_to_process);

    let pdflatex_output = cmd.output();

    match pdflatex_output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Command failed with exit code: {:?}", output.status);
            } else {
                println!("Cleaning up...");
                cleanup_files(&path_buf);
            }
        }
        Err(err) => {
            eprintln!("Error executing command: {}", err);
        }
    }
}

fn cleanup_files(path: &PathBuf) {
    if let Ok(current_dir) = env::current_dir() {
        if let Some(file_stem_osstr) = path.file_stem() {
            if let Some(file_stem_str) = file_stem_osstr.to_str() {
                let file_stem_string: Cow<str> = file_stem_str.into();
                // List all files in the directory
                if let Ok(entries) = fs::read_dir(current_dir) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let file_name = entry.file_name();
                            let file_name_str = file_name.to_string_lossy();
                            // Delete all files except <name>.pdf and <name>.md
                            if file_name_str.starts_with(&*file_stem_string) && !file_name_str.ends_with(".pdf") && !file_name_str.ends_with(".tex") && !file_name_str.ends_with(".md") {
                                println!("deleting: {}", entry.file_name().to_string_lossy());
                                if let Err(err) = fs::remove_file(entry.path()) {
                                    eprintln!("Error deleting file {}: {}", file_name_str, err);
                                }
                            }
                        }
                    }
                }
            }
        }
        
    }
}