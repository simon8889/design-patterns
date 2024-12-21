use crate::gui::Dialog;
use crate::windows_gui::WindowsDialog;
use crate::html_gui::HtmlDialog;

pub fn initialize() -> &'static dyn Dialog {
	if cfg!(windows){
		println!("-- Windows detected, Creating windows GUI --");
		&WindowsDialog
	} else {
		println!("-- No OS detected, Creating HTML GUI --");
		&HtmlDialog
	}
}