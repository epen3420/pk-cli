#![allow(dead_code)]

use std::{path::Path, process::Command};
use anyhow::{Error, anyhow, bail};

use crate::input_util;


const TMUX_COMMAND: &str = "tmux";
const TMUX_SESSION_NAME_PREFIX: &str = "pk_";

fn alias_to_session_name(alias: &str) -> String {
  format!("{}{}", TMUX_SESSION_NAME_PREFIX, alias)
}

fn session_name_to_alias(session_name: &str) -> Option<&str> {
  if let Some((_, after)) = session_name.split_once(TMUX_SESSION_NAME_PREFIX) {
    return Some(after);
  }

  None
}

fn build_launch_cmd_str(path: &Path) -> Result<String, Error> {
  let process_end_msg = "[Process exited. Press Enter to close session...]";

  let Some(dir) = path.parent() else {
    return Err(anyhow!("Invalid path {}", path.display()));
  };
  let Some(file) = path.file_name() else {
    return Err(anyhow!("Invalid path {}", path.display()));
  };

  Ok(format!("cd {} && \"./{}\";\\echo -e \"\n{}\" && read", dir.to_string_lossy(), file.to_string_lossy(), process_end_msg))
}

fn get_running_alias() -> Result<Vec<String>, Error> {
  let output = Command::new(TMUX_COMMAND)
    .arg("ls")
    .output()?;

  if !output.status.success() {
    return Err(anyhow!("No running session"));
  }

  let std_output = &String::from_utf8_lossy(&output.stdout);

  let lines = std_output
    .lines()
    .filter_map(|line| {
      let Some((before, _)) = line.split_once(":") else {
        return None;
      };

      let Some(alias) = session_name_to_alias(before) else {
        return  None;
      };

      Some(alias.to_string())
    })
    .collect();

  Ok(lines)
}

pub fn create_session(alias: &str, path: &Path) -> Result<(), Error> {
  let session_name = &alias_to_session_name(alias);

  let output = Command::new(TMUX_COMMAND)
    .args(["new-session", "-d", "-s", &session_name, &build_launch_cmd_str(&path)?])
    .output()?;

  if !output.status.success() {
    bail!("already running \"{}\"", alias);
  }

  Ok(())
}

pub fn attach_session(alias: &str) -> Result<(), Error> {
  if !has_running_session() {
    return Err(anyhow!("could not found running session"));
  }

  let session_name = &alias_to_session_name(alias);

  Command::new(TMUX_COMMAND)
    .args(["attach", "-t", session_name])
    .status()?;

  Ok(())
}

pub fn attach_session_interactive() -> Result<(), Error> {
  let running_alias = get_running_alias()?;

  if running_alias.len() == 1 {
    return attach_session(&running_alias[0]);
  }

  println!("===== Current running sessions =====");
  let mut count = 1;
  for alias in &running_alias {
    println!("{}: {}", count, alias);
    count += 1;
  }
  println!();
  let num = input_util::get_input_num()?;
  let index = num - 1;

  if index <= 0 {
    return Ok(());
  }

  attach_session(&running_alias[index])
}

pub fn has_running_session() -> bool {
  let Ok(sessions) = get_running_alias() else {
    return  false;
  };

  sessions.len() > 0
}



#[cfg(test)]
mod tests {
  use super::*;
  use std::env::home_dir;

  fn kill_session(alias: &str) -> Result<(), Error> {
    let session_name = &alias_to_session_name(alias);

    Command::new(TMUX_COMMAND)
      .args(["kill-session", "-t", session_name])
      .status()?;

    Ok(())
  }

  #[test]
  fn test_create_session() -> anyhow::Result<()> {
    let alias = "123";
    let path = home_dir().unwrap().join("test.sh");
    create_session(alias, &path)?;

    println!("{}", path.to_string_lossy());

    kill_session(alias)?;

    Ok(())
  }
}
