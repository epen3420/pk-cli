#![allow(dead_code)]

use std::{path::Path, process::Command};
use anyhow::{Error, anyhow, bail};


const TMUX_COMMAND: &str = "tmux";

fn alias_to_session_name(alias: &str) -> String {
  format!("pk_{}", alias)
}

fn session_name_to_alias(session_name: &str) -> Option<&str> {
  if let Some((_, after)) = session_name.split_once("pk_") {
    return Some(after);
  }

  None
}

fn build_launch_cmd_str(path: &Path) -> String {
  let process_end_msg = "[Process exited. Press Enter to close session...]";

  format!("\"{}\";\\echo -e \"\n{}\" && read", path.to_string_lossy(), process_end_msg)
}

pub fn get_running_alias() -> Result<Vec<String>, Error> {
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

  let status = Command::new(TMUX_COMMAND)
    .args(["new-session", "-d", "-s", &session_name, &build_launch_cmd_str(&path)])
    .status()?;

  if !status.success() {
    bail!(anyhow!("failed to create session of \"{}\"", alias));
  }

  Ok(())
}

pub fn attach_session(alias: &str) -> Result<(), Error> {
  if !has_running_session() {
    return Err(anyhow!("could not found running session"));
  }

  let session_name = &alias_to_session_name(alias);

  let status = Command::new(TMUX_COMMAND)
    .args(["attach", "-t", session_name])
    .status()?;

  if !status.success() {
    bail!(anyhow!("failed to attach session of \"{}\"", alias));
  }

  Ok(())
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
