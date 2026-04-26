#![allow(dead_code)]

use std::env::home_dir;
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use anyhow::{anyhow, Context, Error};

const ALIAS_FILE_NAME: &str = ".pk_alias";
const SPLIT_CHAR: char = ':';


pub struct AliasManager {
    file_path: PathBuf,
    tmp_path: PathBuf,
}

impl AliasManager {
    pub fn new() -> Result<Self, Error> {
        let home_path = home_dir().context("Could not find home directory")?;
        let file_path = home_path.join(ALIAS_FILE_NAME);
        let tmp_path = file_path.with_extension("tmp");

        Ok(Self { file_path, tmp_path })
    }

    pub fn with_path(file_path: PathBuf) -> Self {
        let tmp_path = file_path.with_extension("tmp");
        Self { file_path, tmp_path }
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

    fn alias_iterator(&self) -> Result<impl Iterator<Item = Result<(String, PathBuf), Error>>, Error> {
        let alias_file = File::open(&self.file_path).context("Failed to open alias file")?;
        let reader = BufReader::new(alias_file);

        let iter = reader.lines().map(|line_result| {
            let line = line_result?;
            match Self::deserialize_line(&line) {
                Ok((alias, path)) => Ok((String::from(alias), PathBuf::from(path))),
                Err(e) => Err(e)
            }
        });

        Ok(iter)
    }

    fn modify_lines<F>(&self, mut modifier: F) -> Result<(), Error>
    where
        F: FnMut(&str, &str, &Path) -> Result<Option<String>, Error>,
    {
        let file_in = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file_path)
            .with_context(|| format!("Could not open file {}", self.file_path.display()))?;
        let reader = BufReader::new(file_in);

        let file_out = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.tmp_path)
            .with_context(|| format!("Could not open file {}", self.tmp_path.display()))?;
        let mut writer = BufWriter::new(file_out);

        for line_result in reader.lines() {
            let line = line_result?;
            let (alias, path) = Self::deserialize_line(&line)?;

            if let Some(modified_line) = modifier(&line, &alias, &path)? {
                writeln!(writer, "{}", modified_line)?;
            }
        }

        writer.flush()?;
        fs::rename(&self.tmp_path, &self.file_path)?;

        Ok(())
    }

    pub fn create(&self, alias: &str, path: &Path) -> Result<(), Error> {
        let alias_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path)
            .context("Could not open while creating alias and path.")?;
        let mut writer = BufWriter::new(alias_file);

        writeln!(writer, "{}", Self::serialize_line(alias, path))?;

        Ok(())
    }

    pub fn rename(&self, old_alias: &str, new_alias: &str) -> Result<(), Error> {
        self.modify_lines(|current_line, current_alias, current_path| {
            if current_alias == old_alias {
                Ok(Some(Self::serialize_line(new_alias, current_path)))
            } else {
                Ok(Some(current_line.to_string()))
            }
        })
    }

    pub fn update(&self, alias: &str, new_path: &Path) -> Result<(), Error> {
        self.modify_lines(|current_line, current_alias, _| {
            if current_alias == alias {
                Ok(Some(Self::serialize_line(current_alias, new_path)))
            } else {
                Ok(Some(current_line.to_string()))
            }
        })
    }

    pub fn delete(&self, alias: &str) -> Result<(), Error> {
        self.modify_lines(|current_line, current_alias, _| {
            if current_alias == alias {
                Ok(None)
            } else {
                Ok(Some(current_line.to_string()))
            }
        })
    }

    pub fn has_alias(&self, alias: &str) -> Result<bool, Error> {
        let alias_iter = self.alias_iterator()?;

        for result in alias_iter {
            let (current_alias, _) = result?;

            if current_alias == alias {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn get_path_by_alias(&self, alias: &str) -> Result<PathBuf, Error>{
        let alias_iter = self.alias_iterator()?;

        for result in alias_iter {
            let (current_alias, current_path) = result?;

            if current_alias == alias {
                return Ok(current_path);
            }
        }

        Err(anyhow!("Alias {} not found", alias))
    }

    pub fn show_list(&self) -> Result<(), Error> {
        let alias_iter = self.alias_iterator()?;

        for result in alias_iter {
            let (alias, path) = result?;
            println!("{} => {}", alias, path.display());
        }

        Ok(())
    }

    pub fn remove_alias_file(&self) -> Result<(), Error> {
        fs::remove_file(&self.file_path).context(format!("Could not remove {}", self.file_path.display()))
    }

    pub fn remove_alias_temp_file(&self) -> Result<(), Error> {
        fs::remove_file(&self.file_path).context(format!("Could not remove {}", self.file_path.display()))
    }

    pub fn rmeove_all_alias_files(&self) -> Result<(), Error> {
        let res1 = self.remove_alias_file();
        let res2 = self.remove_alias_temp_file();

        res1.and(res2).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn test_alias_manager() -> Result<(), anyhow::Error> {
        let test_file_path = temp_dir().join(".pk_alias_test");
        let alias_manager = AliasManager::with_path(test_file_path);
        let alias = "1234";
        let path = Path::new("abcd");

        println!("creating: alias = {}, path = {}", alias, path.display());
        alias_manager.create(alias, path)?;

        alias_manager.show_list()?;
        println!();

        let alias2 = "4321";
        let path2 = Path::new("dcba");

        println!("creating: alias = {}, path = {}", alias2, path2.display());
        alias_manager.create(alias2, path2)?;

        alias_manager.show_list()?;
        println!();

        let old_alias = "1234";
        let new_alias = "9876";

        println!("renaming alias: {} to {}", old_alias, new_alias);
        alias_manager.rename(old_alias, new_alias)?;

        alias_manager.show_list()?;
        println!();

        let alias3 = "4321";
        let new_path = Path::new("zyxw");

        println!("updating on alias {}: new_path = {}", alias3, new_path.display());
        alias_manager.update(alias3, new_path)?;

        alias_manager.show_list()?;
        println!();

        let alias_to_del = "9876";

        println!("deleting: alias = {}", alias_to_del);
        alias_manager.delete(alias_to_del)?;

        alias_manager.show_list()?;

        let _ = alias_manager.remove_alias_file();
        let _ = alias_manager.remove_alias_temp_file();

        Ok(())
    }
}
