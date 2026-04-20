#[derive(Debug)]
pub enum FmtError {
    Io(std::io::Error),
    Syntax {
        errors: Vec<super::visitor::SyntaxError>,
    },
}

impl std::fmt::Display for FmtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(inner) => write!(f, "{inner}")?,
            Self::Syntax { errors } => {
                for error in errors {
                    write!(f, "{error}")?;
                }
            }
        }
        Ok(())
    }
}

impl std::error::Error for FmtError {}
