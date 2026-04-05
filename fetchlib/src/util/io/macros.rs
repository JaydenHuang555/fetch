macro_rules! error_code {
    ($raw:expr) => {
        if let Some(code) = $raw { code } else { 1 }
    };
}

pub(crate) use error_code;
