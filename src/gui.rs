pub mod egui;
pub mod iced;
pub mod slint;

#[derive(Debug)]
pub struct GUI_Error {}

impl std::fmt::Display for GUI_Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A failure (probably a panic) happened in the GUI Framework)"
        )
    }
}

pub fn gui_mode() -> Result<i32, GUI_Error> {
    todo!()
}
