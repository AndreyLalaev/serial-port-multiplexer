use std::os::unix::prelude::RawFd;

use nix::{
    errno::Errno,
    sys::termios::{cfmakeraw, tcgetattr, tcsetattr, SetArg, Termios},
};

/// Sets raw mode for TTY with a specified fd.
/// Returns an original TTY settings.
///
/// # Arguments
///
/// * `fd` - file descriptor associated with some TTY
pub fn set_raw_mode(fd: RawFd) -> Result<Termios, Errno> {
    let original_settings = tcgetattr(fd)?;
    let mut new_settings = original_settings.clone();

    cfmakeraw(&mut new_settings);
    tcsetattr(fd, SetArg::TCSANOW, &new_settings)?;

    Ok(original_settings)
}
