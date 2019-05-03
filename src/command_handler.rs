use nom::types::CompleteStr;

use crate::util::CmdError;

#[derive(PartialEq, Debug, Clone)]
pub struct Cmd<'a> {
    cmd: String,
    args: Vec<CompleteStr<'a>>
}

// Tokenize a word, ignoring whitespace
named!(pub word<CompleteStr, CompleteStr>,
    do_parse!(
        arg: ws!(nom::alphanumeric) >>
        (arg)
    )
);

// Splits arguments by whitespace and converts them to a vector of strings
named!(parse_args<CompleteStr, Vec<CompleteStr>, CmdError>,
    fix_error!(CmdError,
        ws!(many0!(word))
    )
);

// Parses the first word as the command and the remaining words as arguments, then runs the corresponding command
pub fn handle_cmd(i: CompleteStr) -> nom::IResult<CompleteStr, Cmd, CmdError> {
    do_parse!(i,
        cmd: add_return_error!(nom::ErrorKind::Custom(CmdError::InvalidCmd(i.0)),
                fix_error!(CmdError,
                    switch!(fix_error!(CmdError, word),
                        CompleteStr("sort") => do_parse!(
                            args: parse_args >> // Parses the remaining words as arguments
                            return_error!(
                                nom::ErrorKind::Custom(CmdError::IncorrectArgNum(1, args.len() as u32)),
                                fix_error!(CmdError,
                                    cond_reduce!(args.len() == 1 as usize, // Calls the sort function if there's exactly 1 argument. Otherwise throws an error
                                        call!(sort)
                                    )
                                )
                            ) >>
                            // Returns the parsed command as a struct
                            (Cmd {
                                cmd: "sort".to_string(),
                                args: args
                            })
                        ) |
                        CompleteStr("kill") => do_parse!(
                            args: parse_args >> // Parses the remaining words as arguments
                            return_error!(
                                nom::ErrorKind::Custom(CmdError::IncorrectArgNum(1, args.len() as u32)),
                                fix_error!(CmdError,
                                    cond_reduce!(args.len() == 1 as usize, // Calls the sort function if there's exactly 1 argument. Otherwise throws an error
                                        call!(kill)
                                    )
                                )
                            ) >>
                            // Returns the parsed command as a struct
                            (Cmd {
                                cmd: "kill".to_string(),
                                args: args
                            })
                        )
                    )
                )
        ) >>
        (cmd)
    )
}

// Sorts the list of processes by the specified column
fn sort(i: CompleteStr) -> nom::IResult<CompleteStr, String, CmdError> {
    // TODO: Implement
    Ok((i, "Success".to_string()))
    // Err(Error(error_position!(i, nom::ErrorKind::Custom(CmdError::IncorrectArgNum))))
}

// Kills the specified process
fn kill(i: CompleteStr) -> nom::IResult<CompleteStr, String, CmdError> {
    // TODO: Implement
    Ok((i, "Success".to_string()))
    // Err(Error(error_position!(i, nom::ErrorKind::Custom(CmdError::IncorrectArgNum))))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::Err::{Error, Failure};

    #[test]
    fn arg_test() {
        assert_eq!(word(CompleteStr("sort")), Ok((CompleteStr(""), CompleteStr("sort"))));
        assert_eq!(word(CompleteStr("sort   ")), Ok((CompleteStr(""), CompleteStr("sort"))));
        assert_eq!(word(CompleteStr("   sort")), Ok((CompleteStr(""), CompleteStr("sort"))));
        assert_eq!(word(CompleteStr("   sort   ")), Ok((CompleteStr(""), CompleteStr("sort"))));
        assert_eq!(word(CompleteStr("   sort  fail ")), Ok((CompleteStr("fail "), CompleteStr("sort"))));
        assert_eq!(word(CompleteStr("sort pid")), Ok((CompleteStr("pid"), CompleteStr("sort"))));
    }

    #[test]
    fn parse_args_test() {
        assert_eq!(parse_args(CompleteStr("sort pid")), Ok((CompleteStr(""), vec![CompleteStr("sort"), CompleteStr("pid")])));
        assert_eq!(parse_args(CompleteStr("sort pid    ")), Ok((CompleteStr(""), vec![CompleteStr("sort"), CompleteStr("pid")])));
        assert_eq!(parse_args(CompleteStr(" sort pid" )), Ok((CompleteStr(""), vec![CompleteStr("sort"), CompleteStr("pid")])));
        assert_eq!(parse_args(CompleteStr("    sort pid" )), Ok((CompleteStr(""), vec![CompleteStr("sort"), CompleteStr("pid")])));
        assert_eq!(parse_args(CompleteStr("sort   pid" )), Ok((CompleteStr(""), vec![CompleteStr("sort"), CompleteStr("pid")])));
        assert_eq!(parse_args(CompleteStr("sort   " )), Ok((CompleteStr(""), vec![CompleteStr("sort")])));
        assert_eq!(parse_args(CompleteStr("    sort" )), Ok((CompleteStr(""), vec![CompleteStr("sort")])));
        assert_eq!(parse_args(CompleteStr(" sort" )), Ok((CompleteStr(""), vec![CompleteStr("sort")])));

        assert_eq!(parse_args(CompleteStr("   sort   ")), Ok((CompleteStr(""), vec![CompleteStr("sort")])));
        assert_eq!(parse_args(CompleteStr("sort")), Ok((CompleteStr(""), vec![CompleteStr("sort")])));
        assert_eq!(parse_args(CompleteStr("   sort pid pid")), Ok((CompleteStr(""), vec![CompleteStr("sort"), CompleteStr("pid"), CompleteStr("pid")])));
        assert_eq!(parse_args(CompleteStr("sort pid    pid    ")), Ok((CompleteStr(""), vec![CompleteStr("sort"), CompleteStr("pid"), CompleteStr("pid")])));

        assert_eq!(parse_args(CompleteStr("   sort pid    pid    ")), Ok((CompleteStr(""), vec![CompleteStr("sort"), CompleteStr("pid"), CompleteStr("pid")])));
        assert_eq!(parse_args(CompleteStr("")), Ok((CompleteStr(""), vec![])));
    }

    #[test]
    fn handle_cmd_test() {
        assert_eq!(handle_cmd(CompleteStr("sort pid")), Ok((CompleteStr(""), Cmd {
            cmd: "sort".to_string(),
            args: vec![CompleteStr("pid")]
        })));

        assert_eq!(handle_cmd(CompleteStr("kill 123")), Ok((CompleteStr(""), Cmd {
            cmd: "kill".to_string(),
            args: vec![CompleteStr("123")]
        })));

        assert_eq!(handle_cmd(CompleteStr("kill        123")), Ok((CompleteStr(""), Cmd {
            cmd: "kill".to_string(),
            args: vec![CompleteStr("123")]
        })));

        assert_eq!(handle_cmd(CompleteStr("sort pid pid")), Err(Failure(error_position!(CompleteStr(""), nom::ErrorKind::Custom(CmdError::IncorrectArgNum(1, 2))))));
        assert_eq!(handle_cmd(CompleteStr("sort pid    pid")), Err(Failure(error_position!(CompleteStr(""), nom::ErrorKind::Custom(CmdError::IncorrectArgNum(1, 2))))));
        assert_eq!(handle_cmd(CompleteStr("kill    123 456")), Err(Failure(error_position!(CompleteStr(""), nom::ErrorKind::Custom(CmdError::IncorrectArgNum(1, 2))))));

        assert_eq!(handle_cmd(CompleteStr("sort  ")), Err(Failure(error_position!(CompleteStr(""), nom::ErrorKind::Custom(CmdError::IncorrectArgNum(1, 0))))));
        assert_eq!(handle_cmd(CompleteStr("kill  ")), Err(Failure(error_position!(CompleteStr(""), nom::ErrorKind::Custom(CmdError::IncorrectArgNum(1, 0))))));

        assert_eq!(handle_cmd(CompleteStr("fail")), Err(Error(error_position!(CompleteStr("fail"), nom::ErrorKind::Custom(CmdError::InvalidCmd("fail"))))));
        assert_eq!(handle_cmd(CompleteStr("sortpid")), Err(Error(error_position!(CompleteStr("sortpid"), nom::ErrorKind::Custom(CmdError::InvalidCmd("sortpid"))))));
    }
}
