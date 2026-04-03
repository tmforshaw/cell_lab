use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::genomes::Genome;

const GENOME_FILE_EXT: &str = "genome";

#[must_use]
pub fn sanitise_filename(input: &str) -> String {
    let illegal_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*', '.'];

    input
        .chars()
        .filter(|c| !illegal_chars.contains(c)) // Remove illegal characters
        .map(|c| if c.is_whitespace() { '_' } else { c }) // Replace spaces with _
        .collect()
}

#[must_use]
fn get_data_dir() -> Option<String> {
    // Get the app name
    let mut args = env::args();
    let app_name = args.next()?;

    let app_filepath = Path::new(&app_name);
    let app_filestem = if let Some(app_filestem) = app_filepath.file_stem() {
        app_filestem.to_string_lossy().into_owned()
    } else {
        eprintln!("Could not get file stem of application name");
        return None;
    };

    // Check to see if XDG_DATA_DIRS exist
    let mut path = if let Some(path) = env::var_os("XDG_DATA_HOME") {
        PathBuf::from(path)
    } else {
        let Some(home) = env::var_os("HOME") else {
            eprintln!("HOME environment variable not set");
            return None;
        };

        PathBuf::from(home).join(".local/share")
    };

    // Add the app name to the path
    path.push(app_filestem);

    if let Err(e) = fs::create_dir_all(&path) {
        eprintln!("Failed to create app data directory: {e}");
        return None;
    }

    Some(path.to_string_lossy().into_owned())
}

pub fn write_genome_to_file<S: AsRef<str>>(filename: S, genome: &Genome) {
    // Get the data folder
    if let Some(data_dir) = get_data_dir() {
        let path = Path::new(&data_dir).join(filename.as_ref()).with_extension(GENOME_FILE_EXT);

        // Create the file from the path
        match File::create(&path) {
            Ok(mut file) => {
                // Serialise the content into a slice
                match postcard::to_allocvec(&genome) {
                    Ok(content) => {
                        // Write to the file
                        match file.write(&content) {
                            Ok(_byte_length) => {}
                            Err(e) => {
                                eprintln!("Could not write serialised genome to file: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Could not create file '{data_dir}/{}':\n\t{e}", filename.as_ref());
                    }
                }
            }
            Err(e) => {
                eprintln!("Could not serialise genome for file '{}':\n\t{e}", filename.as_ref());
            }
        }
    } else {
        eprintln!("Could not find genomes directory");
    }
}

pub fn get_genomes_in_folder() -> Option<Vec<String>> {
    // Get the data folder
    let files = if let Some(data_dir) = get_data_dir() {
        match fs::read_dir(data_dir) {
            Ok(entries) => entries
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .is_some_and(|ext| ext == GENOME_FILE_EXT)
                })
                .filter_map(|entry| entry.path().file_stem().and_then(OsStr::to_str).map(ToString::to_string))
                .collect::<Vec<_>>(),
            Err(e) => {
                eprintln!("Could not iterate over entries in genones directory:\n\t{e}");
                return None;
            }
        }
    } else {
        eprintln!("Could not find genomes directory");

        return None;
    };

    Some(files)
}

pub fn does_genome_exist_in_folder<S: AsRef<str>>(filename: S) -> bool {
    let Some(files) = get_genomes_in_folder() else {
        return false;
    };

    files.contains(&filename.as_ref().to_string())
}

pub fn read_genome_file<S: AsRef<str>>(filename: S) -> Option<Genome> {
    // Get the data folder
    let genome = if let Some(data_dir) = get_data_dir() {
        // Get the path to the genome file
        let path = Path::new(&data_dir).join(filename.as_ref()).with_extension(GENOME_FILE_EXT);

        // Read the file at data_dir
        match fs::read(&path) {
            // Parse the contents into a Genome
            Ok(content) => match postcard::from_bytes(&content) {
                Ok(genome) => genome,
                Err(e) => {
                    eprintln!("Could not deserialise genome '{data_dir}'\n\t{e}");
                    return None;
                }
            },
            Err(e) => {
                eprintln!("Could not read file '{data_dir}':\n\t{e}");
                return None;
            }
        }
    } else {
        eprintln!("Could not find genomes directory");

        return None;
    };

    Some(genome)
}

pub fn delete_genome_file<S: AsRef<str>>(filename: S) {
    // Get the data folder
    if let Some(data_dir) = get_data_dir() {
        // Get the path to the genome file
        let path = Path::new(&data_dir).join(filename.as_ref()).with_extension(GENOME_FILE_EXT);

        // Remove the file from disk
        if let Err(e) = fs::remove_file(&path) {
            eprintln!("Could not remove file at '{data_dir}\n\t{e}");
        }
    } else {
        eprintln!("Could not find genomes directory");
    }
}
