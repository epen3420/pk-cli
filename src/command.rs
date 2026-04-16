pub trait Command {
  type Arg;

  fn execute(&self);
}
