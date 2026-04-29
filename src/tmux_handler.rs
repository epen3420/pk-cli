#![allow(dead_code)]

use std::{path::Path, process::Command};
use anyhow::{Context, Error};


const TMUX_COMMAND: &str = "tmux";

fn alias_to_session_name(alias: &str) -> String {
  format!("pk_{}", alias)
}

fn build_launch_cmd_str(path: &Path) -> String {
  let process_end_msg = "[Process exited. Press Enter to close session...]";

  format!("\"{}\";\\echo -e \"\n{}\" && read", path.to_string_lossy(), process_end_msg)
}

fn get_running_session_of_pk() -> Result<Vec<String>, Error> {
  let output = Command::new(TMUX_COMMAND)
    .arg("ls")
    .output()
    .with_context(|| format!("no running {} sessions", TMUX_COMMAND))?;

  let std_output = &String::from_utf8_lossy(&output.stdout);

  let lines = std_output
    .lines()
    .filter_map(|line| {
      match line.split_once(":") {
        Some((before, _after)) => Some(before.to_string()),
        None => None
      }
    })
    .collect();

  Ok(lines)
}

pub fn create_session(alias: &str, path: &Path) -> Result<(), Error> {
  let session_name = &alias_to_session_name(alias);

  let mut awaiter = Command::new(TMUX_COMMAND)
    .args(["new-session", "-d", "-s", &session_name, &build_launch_cmd_str(&path)])
    .spawn()
    .with_context(|| format!("failed to create {}", &session_name))?;

  awaiter.wait().with_context(|| format!("failed to run {}", &session_name))?;

  Ok(())
}

pub fn attach_session(alias: &str) -> Result<(), Error> {
  let session_name = &alias_to_session_name(alias);

  Command::new(TMUX_COMMAND)
    .args(["attach", "-t", session_name])
    .status()?;

  Ok(())
}

pub fn has_running_session() -> bool {
  let Ok(sessions) = get_running_session_of_pk() else {
    return  false;
  };

  sessions.len() > 0
}

pub fn has_session_of_alias(alias: &str) -> bool {
  let session_name = alias_to_session_name(alias);

  let output_result = Command::new(TMUX_COMMAND)
  .args(["has-session", "-t", &session_name])
  .output()
  .with_context(|| "failed to get session status");

  match output_result {
    Ok(output) => output.status.success(),
    Err(_) => false
  }
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
    println!("{}", has_session_of_alias(alias));

    kill_session(alias)?;

    Ok(())
  }
}
