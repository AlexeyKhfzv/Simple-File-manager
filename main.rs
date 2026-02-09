use inquire::{Select, Text};
use colored::*;
use std::fs::{File, read_dir, read_to_string, write, remove_file, create_dir, remove_dir};
use std::env;

fn main() {
    println!("{}", "Welcome to File manager".purple().bold());
    home(None);
}

// Functions for user interaction

pub fn home(directory: Option<String>) {
    let main_dir = directory.clone().unwrap_or(get_main_dir());

    let mut options = vec![
        "Open directory".to_string(),
        "Find directory".to_string(),
        "Find file".to_string(),
        "Create file".to_string(),
        "Create directory".to_string(),
        "Read file".to_string(),
        "Write to file".to_string(),
        "Remove file".to_string(),
        "Remove directory".to_string(),
    ];
    if !directory.is_none() && directory.unwrap() != main_dir {
        options.push("Return to the main dir".blue().bold().to_string());
    }
    options.push("Exit".red().bold().to_string());

    let option = Select::new(&format!("[{}] {}", main_dir, "Select option:".white().bold()), options)
        .prompt().unwrap();

    if option == "Return to the main dir".blue().bold().to_string() {
        home(None)
    } else if option == "Create file" {
        let dir = select_dir(&main_dir, Some("Create here"));
        
        if dir == "Exit" {
            return;
        } else if dir == "Return" {
            return home(Some(main_dir));
        } else {
            let mut filename = Text::new("Enter file name: ").prompt().unwrap();
            filename.insert_str(0, "/");
            filename.insert_str(0, &dir);
            
            match File::create(&filename) {
                Ok(_) => {
                    if Text::new("File successfully created! Return? [y/n]")
                        .prompt().unwrap().to_lowercase() == "y" {
                            home(Some(main_dir));
                        } else {
                            println!("{}", "Exit".red().bold());
                    }
                },
                Err(e) => {
                    if Text::new(&format!("Error while creating file: {}. Return? [y/n]", e))
                        .prompt().unwrap().to_lowercase() == "y" {
                            home(Some(main_dir));
                        } else {
                            println!("{}", "Exit".red().bold());
                    }
                }
            }
        }
    } else if option == "Find file" {
        let filename = Text::new("Enter name of file to find (example: file.txt):").prompt().unwrap();
        let mut filepaths: Vec<String> = Vec::new();

        find_file(&filename, &main_dir, &mut filepaths);
        if filepaths.len() == 0 {
            if Text::new("No such file. Return? [y/n]")
                .prompt().unwrap().to_lowercase() == "y" {
                    home(Some(main_dir));
                } else {
                    println!("{}", "Exit".red().bold());
            }
        } else {
            let mut fp_cnt = 1u32;
            for fp in &filepaths {
                if fp_cnt == 1u32 {
                    println!("Full file path: {}", fp);
                } else {
                    println!("One more file path ({}): {}", fp_cnt, fp);
                }
                fp_cnt += 1;
            }
            if Text::new("Return? [y/n]")
                .prompt().unwrap().to_lowercase() == "y" {
                    home(Some(main_dir));
                } else {
                    println!("{}", "Exit".red().bold());
            }
        }
    } else if option == "Read file" {
        let file = select_file(&main_dir, None);

        if file == "Exit".to_string() {
            return;
        } else if file == "Return".to_string() {
            return home(Some(main_dir));
        } else {
            match read_to_string(file) {
                Ok(content) => {
                    println!("{}\n{}", "File content:".cyan().bold(), content);
                    if Text::new("Return? [y/n]")
                                .prompt().unwrap().to_lowercase() == "y" {
                                    home(Some(main_dir));
                                } else {
                                    println!("{}", "Exit".red().bold());
                            }
                },
                Err(e) => {
                    if Text::new(&format!("Error while reading file: {}. Return? [y/n]", e))
                        .prompt().unwrap().to_lowercase() == "y" {
                            home(Some(main_dir));
                        } else {
                            println!("{}", "Exit".red().bold());
                    }
                },
            }
        }
    } else if option == "Write to file" {
        let file = select_file(&main_dir, None);

        if file == "Exit".to_string() {
            return;
        } else if file == "Return".to_string() {
            return home(Some(main_dir));
        } else {
            match write(file, Text::new("Enter new file content:").prompt().unwrap()) {
                Ok(_) => {
                    if Text::new("New content has been read. Return? [y/n]")
                                .prompt().unwrap().to_lowercase() == "y" {
                                    home(Some(main_dir));
                                } else {
                                    println!("{}", "Exit".red().bold());
                            }
                },
                Err(e) => {
                    if Text::new(&format!("Error while writing to file: {}. Return? [y/n]", e))
                        .prompt().unwrap().to_lowercase() == "y" {
                            home(Some(main_dir));
                        } else {
                            println!("{}", "Exit".red().bold());
                    }
                },
            }
        }
    } else if option == "Open directory" {
        home(Some(select_dir(&main_dir, Some("Open here"))));
    } else if option == "Remove file" {
        let file = select_file(&main_dir, None);

        if file == "Exit".to_string() {
            return;
        } else if file == "Return".to_string() {
            return home(Some(main_dir));
        } else {
            if Select::new("Are you sure?", vec![
                    "Remove this file".green().bold().to_string(),
                    "Cancel".red().bold().to_string(),
                ])
                .prompt().unwrap() != "Remove this file".green().bold().to_string() {
                    println!("{}", "Operation canceled".red().bold());
                    return home(Some(main_dir));
                }
            match remove_file(file) {
                Ok(_) => {
                    if Text::new("The file has been successfully removed. Return? [y/n]")
                                .prompt().unwrap().to_lowercase() == "y" {
                                    home(Some(main_dir));
                                } else {
                                    println!("{}", "Exit".red().bold());
                            }
                },
                Err(e) => {
                    if Text::new(&format!("Error while removing file: {}. Return? [y/n]", e))
                        .prompt().unwrap().to_lowercase() == "y" {
                            home(Some(main_dir));
                        } else {
                            println!("{}", "Exit".red().bold());
                    }
                },
            }
        }
    } else if option == "Find directory" {
        let dirname = Text::new("Enter name of directory to find:").prompt().unwrap();
        let mut dirpaths: Vec<String> = Vec::new();

        find_directory(&dirname, &main_dir, &mut dirpaths);
        if dirpaths.len() == 0 {
            if Text::new("No such directory. Return? [y/n]")
                .prompt().unwrap().to_lowercase() == "y" {
                    home(Some(main_dir));
                } else {
                    println!("{}", "Exit".red().bold());
            }
        } else {
            dirpaths.push("No, return".red().bold().to_string());
            let selected_dir = Select::new(
                &format!("Found {} dirs. Open it?", dirpaths.len() - 1).white().bold().to_string(), dirpaths
            ).prompt().unwrap();
            if selected_dir == "No, return".red().bold().to_string() {
                home(Some(main_dir));
            } else {
                home(Some(selected_dir));
            }
        }
    } else if option == "Create directory" {
        let dir = select_dir(&main_dir, Some("Create here"));
        
        if dir == "Exit" {
            return;
        } else if dir == "Return" {
            return home(Some(main_dir));
        } else {
            let mut filename = Text::new("Enter directory name:").prompt().unwrap();
            filename.insert_str(0, "/");
            filename.insert_str(0, &dir);
            
            match create_dir(&filename) {
                Ok(_) => {
                    if Text::new("Directory successfully created! Return? [y/n]")
                        .prompt().unwrap().to_lowercase() == "y" {
                            home(Some(main_dir));
                        } else {
                            println!("{}", "Exit".red().bold());
                    }
                },
                Err(e) => {
                    if Text::new(&format!("Error while creating directory: {}. Return? [y/n]", e))
                        .prompt().unwrap().to_lowercase() == "y" {
                            home(Some(main_dir));
                        } else {
                            println!("{}", "Exit".red().bold());
                    }
                }
            }
        }
    }
    else if option == "Remove directory" {
        let dir = select_dir(&main_dir, Some("Remove this"));
        
        if dir == "Exit" {
            return;
        } else if dir == "Return" {
            return home(Some(main_dir));
        } else {
            if Select::new(
                &"Are you sure? All files in this dir will also be removed".white().bold().to_string(),
                vec![
                    "Remove this directory".green().bold().to_string(),
                    "Cancel".red().bold().to_string(),
                ]
            ).prompt().unwrap() != "Remove this directory".green().bold().to_string() {
                    println!("{}", "Operation canceled".red().bold());
                    return home(Some(main_dir));
                }
            if let Ok(entries) = read_dir(&dir) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            match remove_file(entry.path().to_str().unwrap()) {
                                Ok(_) => {
                                    println!("{}", &format!(
                                        "The file ({}) has been successfully removed",
                                        entry.file_name().to_str().unwrap()
                                    ))},
                                Err(e) => {
                                    if Text::new(&format!(
                                        "Error while removing file ({}) in the directory: {}. Return? [y/n]",
                                        entry.file_name().to_str().unwrap(), e
                                    )).prompt().unwrap().to_lowercase() == "y" {
                                            home(Some(main_dir.clone()));
                                        } else {
                                            println!("{}", "Exit".red().bold());
                                    }
                                },
                            }
                        }
                    }
                }
            }
            match remove_dir(&dir) {
                Ok(_) => {
                    if Text::new("Directory successfully removed! Return? [y/n]")
                        .prompt().unwrap().to_lowercase() == "y" {
                            home(Some(main_dir));
                        } else {
                            println!("{}", "Exit".red().bold());
                    }
                },
                Err(e) => {
                    if Text::new(&format!("Error while removing directory: {}. Return? [y/n]", e))
                        .prompt().unwrap().to_lowercase() == "y" {
                            home(Some(main_dir));
                        } else {
                            println!("{}", "Exit".red().bold());
                    }
                }
            }
        }
    }
}

fn select_dir(path: &str, return_name: Option<&str>) -> String {
    let mut dirs: Vec::<String> = Vec::new();
    match return_name {
        Some(name) => dirs.push(name.green().bold().to_string()),
        None => {},
    }

    if let Ok(entries) = read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    match entry.path().to_str() {
                        Some(dir) => dirs.push(dir.to_string()),
                        None => {},
                    }
                }
            }
        }
    }
    if dirs.len() == 0 {
        return path.to_string();
    } else {
        dirs.push("Return".blue().bold().to_string());
        dirs.push("Exit".red().bold().to_string());
        
        let dirnow = Select::new(&"Select directory:".white().bold().to_string(), dirs.clone())
            .prompt().unwrap();

        if dirnow == "Exit".red().bold().to_string() {
            return "Exit".to_string();
        } else if dirnow == "Return".blue().bold().to_string() {
            return "Return".to_string();
        } else if let Some(rn) = return_name {
            if dirnow == rn.green().bold().to_string() {
                return path.to_string();
            }
        }
        return select_dir(&dirnow, return_name);
    }
}

fn select_file(path: &str, return_name: Option<&str>) -> String {
    let mut dirs_files: Vec::<String> = Vec::new();
    match return_name {
        Some(name) => dirs_files.push(name.green().bold().to_string()),
        None => {},
    }

    if let Ok(entries) = read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    match entry.path().to_str() {
                        Some(obj) => dirs_files.push(format!("Dir: {}", obj)),
                        None => {},
                    }
                } else if file_type.is_file() {
                    match entry.path().to_str() {
                        Some(obj) => dirs_files.push(format!("File: {}", obj)),
                        None => {},
                    }
                }
            }
        }
    }
    if dirs_files.len() == 0 {
        return path.to_string();
    } else {
        dirs_files.push("Return".blue().bold().to_string());
        dirs_files.push("Exit".red().bold().to_string());
        
        let objnow = &Select::new(&"Select file or directory:".white().bold().to_string(), dirs_files.clone())
            .prompt().unwrap();

        if objnow == &"Exit".red().bold().to_string() {
            return "Exit".to_string();
        } else if objnow == &"Return".blue().bold().to_string() {
            return "Return".to_string();
        } else if let Some(rn) = return_name {
            if *objnow == rn.green().bold().to_string() {
                return path.to_string();
            }
        }
        if objnow.chars().take(3).collect::<String>() == "Dir".to_string() {
            return select_file(&objnow.chars().skip(5).collect::<String>(), return_name);
        } else {
            return objnow.chars().skip(6).collect::<String>();
        }
    }
}

// General functions

pub fn find_file(filename: &str, directory: &str, filepaths: &mut Vec<String>) -> () {
    if let Ok(entries) = read_dir(directory) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    find_file(filename, entry.path().to_str().unwrap(), filepaths)
                } else if file_type.is_file() {
                    match entry.path().to_str() {
                        Some(filepath) => {
                            if entry.file_name() == filename {
                                filepaths.push(filepath.to_string());
                            }
                        },
                        None => {},
                    }
                }
            }
        }
    }
}

pub fn find_directory(dirname: &str, directory: &str, dirpaths: &mut Vec<String>) -> () {
    if let Ok(entries) = read_dir(directory) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    match entry.path().to_str() {
                        Some(filepath) => {
                            if entry.file_name() == dirname {
                                dirpaths.push(filepath.to_string());
                            }
                            find_file(dirname, entry.path().to_str().unwrap(), dirpaths)
                        },
                        None => {},
                    }
                }
            }
        }
    }
}

pub fn get_main_dir() -> String {
    let mut current_dirs: Vec<String> = Vec::new();
    let mut is = String::new();
    for i in String::from(env::current_dir().unwrap().to_str().unwrap()).chars() {
        if i != '/' {
            is.push_str(&i.to_string());
            is = String::new();
        } else {
            current_dirs.push(is.clone())
        }
    }
    
    if current_dirs[0] == "".to_string() {
        format!("/{}", current_dirs[1])
    } else {
        current_dirs[0].clone()
    }
}

