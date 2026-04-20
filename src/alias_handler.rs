#![allow(dead_code)]

use std::env::home_dir;
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use anyhow::{Context, Error};

const ALIAS_FILE_NAME: &str = ".pk_alias";
const SPLIT_CHAR: char = ':';

fn get_alias_path() -> Result<PathBuf, Error> {
    let home_path = home_dir().context("Could not find home directory")?;
    Ok(home_path.join(ALIAS_FILE_NAME))
}

fn get_alias_tmp_path() -> Result<PathBuf, Error> {
    Ok(get_alias_path()?.with_extension("tmp"))
}

fn deserialize_line(line: &str) -> Result<(&str, &Path), Error> {
    let (alias_str, path_str) = line
        .split_once(SPLIT_CHAR)
        .context("Invalid alias format in file")?;

    Ok((alias_str, Path::new(path_str)))
}

fn serialize_line(alias: &str, path: &Path) -> String {
    format!("{}{}{}", alias, SPLIT_CHAR, path.display())
}

fn modify_lines<F>(mut modifier: F) -> Result<(), Error>
where
    F: FnMut(&str) -> Result<Option<String>, Error>,
{
    let alias_path = get_alias_path()?;
    let temp_path = get_alias_tmp_path()?;

    let file_in = File::open(&alias_path)
        .with_context(|| format!("Could not open file {}", alias_path.display()))?;
    let reader = BufReader::new(file_in);

    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp_path)
        .with_context(|| format!("Could not open file {}", temp_path.display()))?;
    let mut writer = BufWriter::new(file_out);

    for line_result in reader.lines() {
        let line = line_result?;

        if let Some(modified_line) = modifier(&line)? {
            writeln!(writer, "{}", modified_line)?;
        }
    }

    writer.flush()?;
    fs::rename(&temp_path, &alias_path)?;

    Ok(())
}

pub fn create(alias: &str, path: &Path) -> Result<(), Error> {
    let alias_path = get_alias_path()?;

    let alias_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&alias_path)
        .context("Could not open while creating alias and path.")?;
    let mut writer = BufWriter::new(alias_file);

    writeln!(writer, "{}", serialize_line(alias, path))?;

    Ok(())
}

pub fn rename(old_alias: &str, new_alias: &str) -> Result<(), Error> {
    modify_lines(|line| {
        let (current_alias, current_path) = deserialize_line(line)?;

        if current_alias == old_alias {
            Ok(Some(serialize_line(new_alias, current_path)))
        } else {
            Ok(Some(line.to_string()))
        }
    })
}

pub fn update(alias: &str, new_path: &Path) -> Result<(), Error> {
    modify_lines(|line| {
        let (current_alias, _) = deserialize_line(line)?;

        if current_alias == alias {
            Ok(Some(serialize_line(alias, new_path)))
        } else {
            Ok(Some(line.to_string()))
        }
    })
}

pub fn delete(alias: &str) -> Result<(), Error> {
    modify_lines(|line| {
        let (current_alias, _) = deserialize_line(line)?;

        if current_alias == alias {
            Ok(None)
        } else {
            Ok(Some(line.to_string()))
        }
    })
}

pub fn show_list() -> Result<(), Error> {
    let alias_path = get_alias_path()?;

    if !alias_path.exists() {
        println!("No aliases registered yet.");
        return Ok(());
    }

    let alias_file = File::open(&alias_path).context("Failed to open alias file")?;
    let reader = BufReader::new(alias_file);

    for line_result in reader.lines() {
        let line = line_result?;
        let (alias, path) = deserialize_line(&line)?;
        println!("{} => {}", alias, path.display());
    }

    Ok(())
}

fn main() {
    let alias = "1234";
    let path = Path::new("abcd");

    println!("create: alias = {}, path = {}", alias, path.display());
    create(alias, path).unwrap();

    show_list().unwrap();
    println!();

    let alias2 = "4321";
    let path2 = Path::new("dcba");

    println!("create: alias = {}, path = {}", alias2, path2.display());
    create(alias2, path2).unwrap();

    show_list().unwrap();
    println!();

    let old_alias = "1234";
    let new_alias = "9876";

    println!("rename alias: {} to {}", old_alias, new_alias);
    rename(old_alias, new_alias).unwrap();

    show_list().unwrap();
    println!();

    let alias3 = "4321";
    let new_path = Path::new("zyxw");

    println!("update on alias {}: new_path = {}", alias3, new_path.display());
    update(alias3, new_path).unwrap();

    show_list().unwrap();
    println!();

    let alias_to_del = "9876";

    println!("delete: alias = {}", alias_to_del);
    delete(alias_to_del).unwrap();

    show_list().unwrap();

    if let Ok(p) = get_alias_path() {
        let _ = fs::remove_file(p);
    }
    if let Ok(p) = get_alias_tmp_path() {
        let _ = fs::remove_file(p);
    }
}
