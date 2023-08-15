pub trait Mixable {
    fn mix(&mut self, move_list: Vec<String>) -> Result<(), String>;
}
