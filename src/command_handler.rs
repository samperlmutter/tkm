use crate::util::CmdError;

#[derive(Debug)]
pub struct Cmd {
    cmd: String,
    args: Vec<String>
}

named!(pub parse_args<&[u8], Vec<String>>,
    complete!(
        map!(separated_list!(nom::space, nom::alpha), |args|
            args.iter()
            .map(|arg| String::from_utf8((*arg).to_vec()).unwrap())
            .collect()
        )
    )
);

pub fn parse_cmd(buf: &[u8]) -> nom::IResult<&[u8], Cmd, CmdError> {
    do_parse!(
        buf,
        cmd: /*return_error!(nom::ErrorKind::Custom(CmdError::InvalidCmd),*/
            // fix_error!(CmdError,
                alt!(
                    do_parse!(
                        cmd: fix_error!(CmdError, tag!("sort")) >>
                        fix_error!(CmdError, take!(1)) >>
                        args: return_error!(nom::ErrorKind::Custom(CmdError::InvalidArgs),
                            fix_error!(CmdError, parse_args)
                        ) >>
                        return_error!(nom::ErrorKind::Custom(CmdError::InvalidArgs),
                        fix_error!(CmdError, cond_reduce!(args.len() == 1, call!(sort)))) >>
                        (Cmd {
                            cmd: String::from_utf8(cmd.to_vec()).unwrap(),
                            args: args
                        })
                    ) |
                    do_parse!(
                        cmd: fix_error!(CmdError, tag!("kill")) >>
                        fix_error!(CmdError, take!(1)) >>
                        args: return_error!(nom::ErrorKind::Custom(CmdError::InvalidArgs),
                            fix_error!(CmdError, parse_args)
                        ) >>
                        return_error!(nom::ErrorKind::Custom(CmdError::InvalidArgs),
                        fix_error!(CmdError, cond_reduce!(args.len() == 1, call!(kill)))) >>
                        (Cmd {
                            cmd: String::from_utf8(cmd.to_vec()).unwrap(),
                            args: args
                        })
                    )
                // )
            // )
        ) >>
        (cmd)
    )
}

fn sort(i: &[u8]) -> nom::IResult<&[u8], String, CmdError> {
    Ok((i, "Success".to_string()))
    // Err(nom::Err::Error(nom::Context::Code(i, nom::ErrorKind::Custom(CmdError::InvalidArgs))))
}

fn kill(i: &[u8]) -> nom::IResult<&[u8], String, CmdError> {
    Ok((i, "Success".to_string()))
    // Err(nom::Err::Error(nom::Context::Code(i, nom::ErrorKind::Custom(CmdError::InvalidArgs))))
}
