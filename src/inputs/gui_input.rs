pub trait GuiInput {
    fn new(value: String) -> Self;
    fn is_sane(&self) -> bool;
    fn get_value(&self) -> String;
}