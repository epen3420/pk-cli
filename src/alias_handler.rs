#![allow(dead_code)]

use std::env::home_dir;
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write};

const ALIAS_FILE_NAME: &str = ".pk_alias";
const SPLIT_CHAR: char = ':';

fn get_alias_path() -> PathBuf {
  let home_path = home_dir().expect("Could not find home directory");

  Path::join(&home_path, ALIAS_FILE_NAME)
}

fn get_alias_tmp_path() -> PathBuf {
  get_alias_path().with_extension("tmp")
}

fn deserialize_line(line: &str) -> Result<(String, PathBuf), Error> {
  let (alias_str, path_str) = line
    .split_once(SPLIT_CHAR)
    .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid alias format in file"))?;

  Ok((String::from(alias_str), PathBuf::from(path_str)))
}

fn serialize_line(alias: String, path: PathBuf) -> String {
  let path_str = path.to_string_lossy();

  format!("{}{}{}", alias, SPLIT_CHAR, path_str)
}

fn modify_lines<F>(mut modifier: F) -> Result<(), Error>
where
    F: FnMut(&str) -> Result<Option<String>, Error>,
{
    let alias_path = get_alias_path();
    let temp_path = get_alias_tmp_path();

    let file_in = File::open(&alias_path)?;
    let reader = BufReader::new(file_in);

    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&temp_path)?;
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

pub fn create(alias: String, path: PathBuf) -> Result<(), Error> {
  let alias_file = OpenOptions::new()
      .append(true)
      .create(true)
      .open(get_alias_path())?;
  let mut writer = BufWriter::new(alias_file);

  writeln!(writer, "{}", serialize_line(alias, path))?;

  Ok(())
}

pub fn rename(old_alias: String, new_alias: String) -> Result<(), Error> {
  modify_lines(|line| {
    let (current_alias, current_path) = deserialize_line(line)?;

    if current_alias == old_alias {
      Ok(Some(serialize_line(new_alias.clone(), current_path.clone())))
    }
    else {
      Ok(Some(line.to_string()))
    }
  })
}

pub fn update(alias: String, new_path: PathBuf) -> Result<(), Error> {
  modify_lines(|line| {
    let (current_alias, _) = deserialize_line(line)?;

    if current_alias == alias {
      Ok(Some(serialize_line(alias.clone(), new_path.clone())))
    }
    else {
      Ok(Some(line.to_string()))
    }
  })
}

pub fn delete(alias: String) -> Result<(), Error> {
  modify_lines(|line| {
    let (current_alias, _) = deserialize_line(line)?;

    if current_alias == alias {
      Ok(None)
    } else {
      Ok(Some(line.to_string()))
    }
  })
}

pub fn show_list() {
  let alias_file = File::open(get_alias_path()).unwrap();
  let reader = BufReader::new(alias_file);

  for line_result in reader.lines() {
    let line = line_result.unwrap();
    let (alias, path) = deserialize_line(&line).unwrap();

    println!("{} => {}", alias, path.to_string_lossy());
  }
}

fn main(){
  let alias = "1234";
  let path = "abcd";

  println!("create: alias = {}, path = {}", alias, path);
  create(alias.to_string(), PathBuf::from(path)).unwrap();

  show_list();
  println!();

  let alias = "4321";
  let path = "dcba";

  println!("create: alias = {}, path = {}", alias, path);
  create(alias.to_string(), PathBuf::from(path)).unwrap();

  show_list();
  println!();

  let old_alias = String::from("1234");
  let new_alias = String::from("9876");

  println!("rename alias: {} to {}", old_alias, new_alias);
  rename(old_alias, new_alias).unwrap();

  show_list();
  println!();

  let alias = String::from("4321");
  let new_path = PathBuf::from("zyxw");

  println!("update on alias {}: new_path = {}", alias, new_path.to_string_lossy());
  update(alias, new_path).unwrap();

  show_list();
  println!();

  let alias = String::from("9876");

  println!("delete: alias = {}", alias);
  delete(alias).unwrap();

  show_list();

  let _ = fs::remove_file(get_alias_path());
  let _ = fs::remove_file(get_alias_tmp_path());
}
