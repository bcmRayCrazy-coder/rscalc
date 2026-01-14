pub trait Command {
    fn get_name(&self) -> String;
    fn get_icon(&self) -> String;
    fn get_enable(&self) -> String;
    fn exec(&mut self);
}
