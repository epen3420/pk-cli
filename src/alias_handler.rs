#![allow(dead_code)]

use std::env::home_dir;
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use anyhow::{anyhow, Context, Error};

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

fn alias_iterator() -> Result<impl Iterator<Item = Result<(String, PathBuf), Error>>, Error> {
    let alias_path = get_alias_path()?;
    let alias_file = File::open(&alias_path).context("Failed to open alias file")?;
    let reader = BufReader::new(alias_file);

    let iter = reader.lines().map(|line_result| {
        let line = line_result?;
        match deserialize_line(&line) {
            Ok((alias, path)) => Ok((String::from(alias), PathBuf::from(path))),
            Err(e) => Err(e)
        }
    });

    Ok(iter)
}

fn modify_lines<F>(mut modifier: F) -> Result<(), Error>
where
    F: FnMut(&str, &str, &Path) -> Result<Option<String>, Error>,
{
    let alias_path = get_alias_path()?;
    let temp_path = get_alias_tmp_path()?;

    let file_in = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&alias_path)
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
        let (alias, path) = deserialize_line(&line)?;

        if let Some(modified_line) = modifier(&line, &alias, &path)? {
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
    modify_lines(|current_line, current_alias, current_path| {
        if current_alias == old_alias {
            Ok(Some(serialize_line(new_alias, current_path)))
        } else {
            Ok(Some(current_line.to_string()))
        }
    })
}

pub fn update(alias: &str, new_path: &Path) -> Result<(), Error> {
    modify_lines(|current_line, current_alias, _| {
        if current_alias == alias {
            Ok(Some(serialize_line(current_alias, new_path)))
        } else {
            Ok(Some(current_line.to_string()))
        }
    })
}

pub fn delete(alias: &str) -> Result<(), Error> {
    modify_lines(|current_line, current_alias, _| {
        if current_alias == alias {
            Ok(None)
        } else {
            Ok(Some(current_line.to_string()))
        }
    })
}

pub fn has_alias(alias: &str) -> Result<bool, Error> {
    let alias_iter = alias_iterator()?;

    for result in alias_iter {
        let (current_alias, _) = result?;

        if current_alias == alias {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn get_path_by_alias(alias: &str) -> Result<PathBuf, Error>{
    let alias_iter = alias_iterator()?;

    for result in alias_iter {
        let (current_alias, current_path) = result?;

        if current_alias == alias {
            return Ok(current_path);
        }
    }

    Err(anyhow!("Alias {} not found", alias))
}

pub fn show_list() -> Result<(), Error> {
    let alias_iter = alias_iterator()?;

    for result in alias_iter {
        let (alias, path) = result?;
        println!("{} => {}", alias, path.display());
    }

    Ok(())
}

pub fn remove_alias_file() -> Result<(), Error> {
    let file_path = get_alias_path()?;

    fs::remove_file(&file_path).context(format!("Could not remove {}", &file_path.display()))
}

pub fn remove_alias_temp_file() -> Result<(), Error> {
    let file_path = get_alias_tmp_path()?;

    fs::remove_file(&file_path).context(format!("Could not remove {}", &file_path.display()))
}

pub fn rmeove_all_alias_files() -> Result<(), Error> {
    let res1 = remove_alias_file();
    let res2 = remove_alias_temp_file();

    res1.and(res2).map_err(|e| e.into())
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

    let _ = remove_alias_file();
    let _ = remove_alias_temp_file();
}
