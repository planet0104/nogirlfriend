#![windows_subsystem = "windows"]

use native_dialog::{MessageDialog, MessageType};

fn main() {
    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title("错误")
        .set_text("找不到对象!")
        .show_alert()
        .unwrap();
}
