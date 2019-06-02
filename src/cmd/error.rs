#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CmdError<'a> {
    IncorrectArgNum(u32, u32),
    InvalidCmd(&'a str),
    InvalidArg(&'a str),
    Err(&'a str),
    ParseErr,
}

impl<'a> From<u32> for CmdError<'a> {
    fn from(_: u32) -> Self {
        CmdError::ParseErr
    }
}

impl<'a> CmdError<'a> {
    pub fn display(&self) -> String {
        match self {
            CmdError::IncorrectArgNum(exp, rec) => {
                format!("Wrong number of arguments: expected {}, found {}", exp, rec)
            }
            CmdError::InvalidCmd(cmd) => format!("Command not found: {}", cmd),
            CmdError::InvalidArg(arg) => format!("Invalid argument: {}", arg),
            CmdError::Err(err) => format!("Error: {}", err),
            CmdError::ParseErr => "Error during parsing".to_string(),
        }
    }
}
