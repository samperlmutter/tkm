use nom::types::CompleteStr;

use crate::util::CmdError;

#[derive(PartialEq, Debug, Clone)]
pub struct Cmd {
    cmd: String,
    args: Vec<String>
}

// Tokenize a word, ignoring whitespace
named!(pub arg<CompleteStr, String>,
    do_parse!(
        arg: ws!(nom::alphanumeric) >>
        (arg.to_string())
    )
);

// Splits arguments by whitespace and converts them to a vector of strings
named!(parse_args<CompleteStr, Vec<String>, CmdError>,
    fix_error!(CmdError,
        ws!(many0!(arg))
    )
);

fn cmd(i: CompleteStr, cmd: String) -> nom::IResult<CompleteStr, CompleteStr, CmdError> {
    do_parse!(i,
        c: add_return_error!(nom::ErrorKind::Custom(CmdError::InvalidCmd(i.0)),
            fix_error!(CmdError,
                ws!(tag!(cmd.as_str()))
            )
        ) >>
        (c)
    )
}

// Parses the first word as the command and the remaining words as arguments, then runs the corresponding command
pub fn handle_cmd(i: CompleteStr) -> nom::IResult<CompleteStr, Cmd, CmdError> {
    do_parse!(
        i,
        cmd: add_return_error!(nom::ErrorKind::Custom(CmdError::InvalidCmd(i.0)), fix_error!(CmdError, alt!(
                do_parse!(
                    cmd: call!(cmd, "sort".to_string()) >> // Parses the first word as the command
                    args: parse_args >> // Parses the remaining words as arguments
                    return_error!(
                        nom::ErrorKind::Custom(CmdError::InvalidArgs(1, args.len() as u32)),
                        fix_error!(CmdError,
                            cond_reduce!(args.len() == 1 as usize, // Calls the sort function if there's exactly 1 argument. Otherwise throws an error
                                call!(sort)
                            )
                        )
                    ) >>
                    // Returns the parsed command as a struct
                    (Cmd {
                        cmd: cmd.to_string(),
                        args: args
                    })
                ) |
                do_parse!(
                    cmd: call!(cmd, "kill".to_string()) >> // Parses the first word as the command
                    args: parse_args >> // Parses the remaining words as arguments
                    return_error!(
                        nom::ErrorKind::Custom(CmdError::InvalidArgs(1, args.len() as u32)),
                        fix_error!(CmdError,
                            cond_reduce!(args.len() == 1 as usize, // Calls the sort function if there's exactly 1 argument. Otherwise throws an error
                                call!(kill)
                            )
                        )
                    ) >>
                    // Returns the parsed command as a struct
                    (Cmd {
                        cmd: cmd.to_string(),
                        args: args
                    })
                )
        ))) >>
        (cmd)
    )
}

// Sorts the list of processes by the specified column
fn sort(i: CompleteStr) -> nom::IResult<CompleteStr, String, CmdError> {
    // TODO: Implement
    Ok((i, "Success".to_string()))
    // Err(Error(error_position!(i, nom::ErrorKind::Custom(CmdError::InvalidArgs))))
}

// Kills the specified process
fn kill(i: CompleteStr) -> nom::IResult<CompleteStr, String, CmdError> {
    // TODO: Implement
    Ok((i, "Success".to_string()))
    // Err(Error(error_position!(i, nom::ErrorKind::Custom(CmdError::InvalidArgs))))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::Err::{Error, Failure};

    #[test]
    fn arg_test() {
        assert_eq!(arg(CompleteStr("sort")), Ok((CompleteStr(""), "sort".to_string())));
        assert_eq!(arg(CompleteStr("sort   ")), Ok((CompleteStr(""), "sort".to_string())));
        assert_eq!(arg(CompleteStr("   sort")), Ok((CompleteStr(""), "sort".to_string())));
        assert_eq!(arg(CompleteStr("   sort   ")), Ok((CompleteStr(""), "sort".to_string())));
        assert_eq!(arg(CompleteStr("   sort  fail ")), Ok((CompleteStr("fail "), "sort".to_string())));
    }

    #[test]
    fn cmd_test() {
        assert_eq!(cmd(CompleteStr("sort"), "sort".to_string()), Ok((CompleteStr(""), CompleteStr("sort"))));
        assert_eq!(cmd(CompleteStr("kill"), "kill".to_string()), Ok((CompleteStr(""), CompleteStr("kill"))));
        assert_eq!(cmd(CompleteStr("    kill"), "kill".to_string()), Ok((CompleteStr(""), CompleteStr("kill"))));
        assert_eq!(cmd(CompleteStr("kill   "), "kill".to_string()), Ok((CompleteStr(""), CompleteStr("kill"))));
        assert_eq!(cmd(CompleteStr("    kill     "), "kill".to_string()), Ok((CompleteStr(""), CompleteStr("kill"))));

        assert_eq!(cmd(CompleteStr("sort"), "else".to_string()), Err(Error(error_position!(CompleteStr("sort"), nom::ErrorKind::Custom(CmdError::InvalidCmd("sort"))))));
    }

    #[test]
    fn parse_args_test() {
        assert_eq!(parse_args(CompleteStr("sort pid")), Ok((CompleteStr(""), vec!["sort".to_string(), "pid".to_string()])));
        assert_eq!(parse_args(CompleteStr("sort pid    ")), Ok((CompleteStr(""), vec!["sort".to_string(), "pid".to_string()])));
        assert_eq!(parse_args(CompleteStr(" sort pid" )), Ok((CompleteStr(""), vec!["sort".to_string(), "pid".to_string()])));
        assert_eq!(parse_args(CompleteStr("    sort pid" )), Ok((CompleteStr(""), vec!["sort".to_string(), "pid".to_string()])));
        assert_eq!(parse_args(CompleteStr("sort   pid" )), Ok((CompleteStr(""), vec!["sort".to_string(), "pid".to_string()])));
        assert_eq!(parse_args(CompleteStr("sort   " )), Ok((CompleteStr(""), vec!["sort".to_string()])));
        assert_eq!(parse_args(CompleteStr("    sort" )), Ok((CompleteStr(""), vec!["sort".to_string()])));
        assert_eq!(parse_args(CompleteStr(" sort" )), Ok((CompleteStr(""), vec!["sort".to_string()])));

        assert_eq!(parse_args(CompleteStr("   sort   ")), Ok((CompleteStr(""), vec!["sort".to_string()])));
        assert_eq!(parse_args(CompleteStr("sort")), Ok((CompleteStr(""), vec!["sort".to_string()])));
        assert_eq!(parse_args(CompleteStr("   sort pid pid")), Ok((CompleteStr(""), vec!["sort".to_string(), "pid".to_string(), "pid".to_string()])));
        assert_eq!(parse_args(CompleteStr("sort pid    pid    ")), Ok((CompleteStr(""), vec!["sort".to_string(), "pid".to_string(), "pid".to_string()])));
    }

    #[test]
    fn handle_cmd_test() {
        assert_eq!(handle_cmd(CompleteStr("sort pid")), Ok((CompleteStr(""), Cmd {
            cmd: "sort".to_string(),
            args: vec!["pid".to_string()]
        })));

        assert_eq!(handle_cmd(CompleteStr("kill 123")), Ok((CompleteStr(""), Cmd {
            cmd: "kill".to_string(),
            args: vec!["123".to_string()]
        })));

        assert_eq!(handle_cmd(CompleteStr("kill        123")), Ok((CompleteStr(""), Cmd {
            cmd: "kill".to_string(),
            args: vec!["123".to_string()]
        })));

        assert_eq!(handle_cmd(CompleteStr("sort pid pid")), Err(Failure(error_position!(CompleteStr(""), nom::ErrorKind::Custom(CmdError::InvalidArgs(1, 2))))));
        assert_eq!(handle_cmd(CompleteStr("sort pid    pid")), Err(Failure(error_position!(CompleteStr(""), nom::ErrorKind::Custom(CmdError::InvalidArgs(1, 2))))));
        assert_eq!(handle_cmd(CompleteStr("kill    123 456")), Err(Failure(error_position!(CompleteStr(""), nom::ErrorKind::Custom(CmdError::InvalidArgs(1, 2))))));

        assert_eq!(handle_cmd(CompleteStr("sort  ")), Err(Failure(error_position!(CompleteStr(""), nom::ErrorKind::Custom(CmdError::InvalidArgs(1, 0))))));
        assert_eq!(handle_cmd(CompleteStr("kill  ")), Err(Failure(error_position!(CompleteStr(""), nom::ErrorKind::Custom(CmdError::InvalidArgs(1, 0))))));

        assert_eq!(handle_cmd(CompleteStr("fail")), Err(Error(error_position!(CompleteStr("fail"), nom::ErrorKind::Custom(CmdError::InvalidCmd("fail"))))));
    }
}
