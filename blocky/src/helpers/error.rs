use std::fmt::{Debug, Display};

// TODO: Show dialog
// Gets called from other threads
pub fn error_dialog<T: Debug + Display>(err: T) {
    error!("{}", err);
}
