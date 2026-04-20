pub trait Command {
  fn execute(&self) -> Result<(), anyhow::Error>;
}
