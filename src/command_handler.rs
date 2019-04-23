use crate::util::CmdError;

#[derive(Debug)]
pub struct Cmd {
    cmd: String,
    args: Vec<String>
}

named!(pub parse_args<&[u8], Vec<String>>,
    complete!(
        map!(separated_list!(nom::space, nom::alpha), |args| {
            args.iter()
            .map(|arg| String::from_utf8((*arg).to_vec()).unwrap())
            .collect()
        })
    )
);

pub fn parse_cmd(buf: &[u8]) -> nom::IResult<&[u8], Cmd, CmdError> {
    do_parse!(
        buf,
        cmd: return_error!(nom::ErrorKind::Custom(CmdError::InvalidCmd),
            fix_error!(CmdError,
                alt!(
                    tag!("sort") |
                    tag!("kill")
                )
            )
        ) >>
        return_error!(nom::ErrorKind::Custom(CmdError::ParseErr),
            fix_error!(CmdError, take!(1))
        ) >>
        args: add_return_error!(nom::ErrorKind::Custom(CmdError::ParseErr),
            fix_error!(CmdError, parse_args)
        ) >>
        (Cmd {
            cmd: String::from_utf8(cmd.to_vec()).unwrap(),
            args: args
        })
    )
    // do_parse!(
    //     buf,
    //     result: alt!(
    //         do_parse!(
    //             cmd: tag!("sort") >>
    //             take!(1) >>
    //             ws: separated_list!(nom::space, nom::alpha) >>
    //             (Cmd {cmd: String::from_utf8(cmd.to_vec()).unwrap(), args: ws})
    //         ) |
    //         do_parse!(
    //             cmd: tag!("kill") >>
    //             take!(1) >>
    //             ws: separated_list!(nom::space, nom::alpha) >>
    //             (Cmd {cmd: String::from_utf8(cmd.to_vec()).unwrap(), args: ws})
    //         )
    //     ) >>
    //     (result)
    // )

    // Cmd {
    //     cmd: String::from_utf8((*cmd).to_vec()).unwrap(),
    //     args: (*args).to_vec().iter().map(|arg| String::from_utf8((*arg).to_vec()).unwrap()).collect()
    // }
    // if let Some((cmd, args)) = tokens.split_first() {

    // }
    // do_parse!(tokens,
    //     alt!(
    //         do_parse!(
    //             cmd: tag!("sort") >>
    //             args: many0!(map_res!(nom::alpha, std::str::from_utf8)) >>
    //             ((cmd, args))
    //         )
    //     )
    // )
    // Ok(())
}

// named!(pub parse_cmd<(&[u8], Vec<&str>)>,
//     alt!(
//         do_parse!(
//             cmd: tag!("sort") >>
//             args: many0!(map_res!(nom::alpha, std::str::from_utf8)) >>
//             ((cmd, args))
//         )
//     )
// );
