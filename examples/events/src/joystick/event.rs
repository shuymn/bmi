use super::ButtonCode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    KeyPressed(ButtonCode),
    KeyReleased(ButtonCode),
    Scratched(f64),
}
