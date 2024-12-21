use crate::gui::{Dialog, Button};

pub struct WindowsButton;

impl Button for WindowsButton {
	fn render(&self) {
		println!("Rendering windows button");
		self.on_click();
	}
	 
	fn on_click(&self) {
		println!("Clicking windows button");
	}
}

pub struct WindowsDialog;

impl Dialog for WindowsDialog {
	fn create_button(&self) -> Box<dyn Button> {
		Box::new(WindowsButton)
	}
}