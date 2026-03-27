use crate::remote_file_system;

macro_rules! remote_secure_shell_channel_close {
    ($remote_channel:expr) => {{
        if let Err(e) = $remote_channel.send_eof() {
            Some(e)
        } else if let Err(e) = $remote_channel.wait_eof() {
            Some(e)
        } else if let Err(e) = $remote_channel.close() {
            Some(e)
        } else if let Err(e) = $remote_channel.wait_close() {
            Some(e)
        } else {
            None
        }
    }};
}

pub(crate) use remote_secure_shell_channel_close;
