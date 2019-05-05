use nom::types::CompleteStr;

use crate::util::{Cmd, CmdError, Action};

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
                            // Returns the parsed command as a struct
                            (Cmd {
                                cmd: Action::Sort,
                                args: args
                            })
                        ) |
                        CompleteStr("kill") => do_parse!(
                            args: parse_args >> // Parses the remaining words as arguments
                            // Returns the parsed command as a struct
                            (Cmd {
                                cmd: Action::Kill,
                                args: args
                            })
                        )
                    )
                )
        ) >>
        (cmd)
    )
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
            cmd: Action::Sort,
            args: vec![CompleteStr("pid")]
        })));

        assert_eq!(handle_cmd(CompleteStr("kill 123")), Ok((CompleteStr(""), Cmd {
            cmd: Action::Kill,
            args: vec![CompleteStr("123")]
        })));

        assert_eq!(handle_cmd(CompleteStr("kill        123")), Ok((CompleteStr(""), Cmd {
            cmd: Action::Kill,
            args: vec![CompleteStr("123")]
        })));

        assert_eq!(handle_cmd(CompleteStr("sort pid pid")), Ok((CompleteStr(""), Cmd {
            cmd: Action::Sort,
            args: vec![CompleteStr("pid"), CompleteStr("pid")]
        })));

        assert_eq!(handle_cmd(CompleteStr("sort pid    pid")), Ok((CompleteStr(""), Cmd {
            cmd: Action::Sort,
            args: vec![CompleteStr("pid"), CompleteStr("pid")]
        })));

        assert_eq!(handle_cmd(CompleteStr("kill    123 456")), Ok((CompleteStr(""), Cmd {
            cmd: Action::Kill,
            args: vec![CompleteStr("123"), CompleteStr("456")]
        })));

        assert_eq!(handle_cmd(CompleteStr("sort  ")), Ok((CompleteStr(""), Cmd {
            cmd: Action::Sort,
            args: vec![]
        })));
        assert_eq!(handle_cmd(CompleteStr("kill  ")), Ok((CompleteStr(""), Cmd {
            cmd: Action::Kill,
            args: vec![]
        })));

        assert_eq!(handle_cmd(CompleteStr("fail")), Err(Error(error_position!(CompleteStr("fail"), nom::ErrorKind::Custom(CmdError::InvalidCmd("fail"))))));
        assert_eq!(handle_cmd(CompleteStr("sortpid")), Err(Error(error_position!(CompleteStr("sortpid"), nom::ErrorKind::Custom(CmdError::InvalidCmd("sortpid"))))));
    }
}
