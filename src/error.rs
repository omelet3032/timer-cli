#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn input<E: std::error::Error>(err: E) -> Error {
        Error {
            kind: ErrorKind::Input(err.to_string()),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    Input(String),
}

impl std::error::Error for Error {
    /*     fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Input(_) => "input error",
        }
    } */
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Input(s) => write!(f, "{}", s),
        }
    }
}
