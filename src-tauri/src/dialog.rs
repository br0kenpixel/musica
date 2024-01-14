use std::process::exit;
use tauri::api::dialog::{blocking::MessageDialogBuilder, MessageDialogButtons, MessageDialogKind};

const DIALOG_TITLE: &str = "Musica";
const ERROR_EXIT_CODE: i32 = 1;

pub fn error<S: AsRef<str>>(message: S) -> ! {
    MessageDialogBuilder::new(DIALOG_TITLE, message)
        .kind(MessageDialogKind::Error)
        .buttons(MessageDialogButtons::Ok)
        .show();
    exit(ERROR_EXIT_CODE);
}
