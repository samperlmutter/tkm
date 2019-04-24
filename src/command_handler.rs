use crate::util::CmdError;

#[derive(Debug)]
pub struct Cmd {
    cmd: String,
    args: Vec<String>
}

// Splits arguments by whitespace and converts them to a vector of strings
named!(parse_args<&[u8], Vec<String>, CmdError>,
    return_error!(nom::ErrorKind::Custom(CmdError::ParseErr),
        fix_error!(CmdError,
            complete!(
                map!(separated_list!(nom::space, nom::alpha), |args| // Splits words by spaces
                    args.iter()
                    .map(|arg| String::from_utf8((*arg).to_vec()).unwrap()) // Converts bytes to strings
                    .collect()
                )
            )
        )
    )
);

// Parses the first word as a command
fn cmd(i: &[u8], cmd_str: String)-> nom::IResult<&[u8], &[u8], CmdError> {
    do_parse!(
        i,
        cmd: fix_error!(CmdError,
            tag!(cmd_str.as_bytes())
        ) >>
        fix_error!(CmdError,
            take!(1) // Consumes the space after the command
        ) >>
        (cmd)
    )
}

// Parses the first word as the command and the remaining words as arguments, then runs the corresponding command
pub fn handle_cmd(i: &[u8]) -> nom::IResult<&[u8], Cmd, CmdError> {
    do_parse!(
        i,
        cmd: alt!(
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
                        cmd: String::from_utf8(cmd.to_vec()).unwrap(),
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
                        cmd: String::from_utf8(cmd.to_vec()).unwrap(),
                        args: args
                    })
                )
        ) >>
        (cmd)
    )
}

// Sorts the list of processes by the specified column
fn sort(i: &[u8]) -> nom::IResult<&[u8], String, CmdError> {
    // TODO: Implement
    Ok((i, "Success".to_string()))
    // Err(nom::Err::Error(nom::Context::Code(i, nom::ErrorKind::Custom(CmdError::InvalidArgs))))
}

// Kills the specified process
fn kill(i: &[u8]) -> nom::IResult<&[u8], String, CmdError> {
    // TODO: Implement
    Ok((i, "Success".to_string()))
    // Err(nom::Err::Error(nom::Context::Code(i, nom::ErrorKind::Custom(CmdError::InvalidArgs))))
}
