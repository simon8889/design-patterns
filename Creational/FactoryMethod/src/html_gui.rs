use crate::gui::{Button, Dialog};

pub struct HtmlButton;

impl Button for HtmlButton {
	fn render(&self) {
		println!("<button>Click me!</button>");
		self.on_click();
	}
	
	fn on_click(&self){
		println!("Click! button says");
	}
}

pub struct HtmlDialog;

impl Dialog for HtmlDialog {
	fn create_button(&self) -> Box<dyn Button> {
		Box::new(HtmlButton)
	}
}