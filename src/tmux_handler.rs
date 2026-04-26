#![allow(dead_code)]

use std::{path::{Path}, process::Command};
use anyhow::{Context, Error};


const TMUX_COMMAND: &str = "tmux";

fn alias_to_session_name(alias: &str) -> String {
  format!("pk_{}", alias)
}

fn build_launch_cmd_str(path: &Path) -> String {
  let process_end_msg = "[Process exited. Press Enter to close session...]";

  format!("\"{}\";\\echo -e \"\n{}\" && read", path.to_string_lossy(), process_end_msg)
}

fn create_session(alias: &str, path: &Path) -> Result<(), Error> {
  let session_name = &alias_to_session_name(alias);

  let mut awaiter = Command::new(TMUX_COMMAND)
    .args(["new-session", "-d", "-s", &session_name, &build_launch_cmd_str(&path)])
    .spawn()
    .with_context(|| format!("failed to create {}", &session_name))?;

  awaiter.wait().with_context(|| format!("failed to run {}", &session_name))?;

  Ok(())
}

fn attach_session(alias: &str) -> Result<(), Error> {
  let session_name = &alias_to_session_name(alias);

  Command::new(TMUX_COMMAND)
    .args(["attach", "-t", session_name])
    .status()?;

  Ok(())
}

fn has_session(alias: &str) -> Result<bool, Error> {
  let session_name = alias_to_session_name(alias);

  let output = Command::new(TMUX_COMMAND)
  .args(["has-session", "-t", &session_name])
  .output()
  .with_context(|| "failed to get session status")?;

  Ok(output.status.success())
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
    println!("{}", has_session(alias)?);

    kill_session(alias)?;

    Ok(())
  }
}
