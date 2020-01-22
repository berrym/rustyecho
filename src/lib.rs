use getargs::{Error, Opt, Options, Result};

#[derive(Default, Debug)]
pub struct CommandLineArguments<'a> {
    pub address: &'a str,
    pub port: &'a str,
}

pub fn parse_args<'a>(opts: &'a Options<'a, String>) -> Result<CommandLineArguments<'a>> {
    let mut result = CommandLineArguments {
        address: "127.0.0.1",
        port: "8888",
    };

    while let Some(opt) = opts.next() {
        match opt? {
            // -e EXPRESSION, or -eEXPRESSION, or
            // --execute EXPRESSION, or --execute=EXPRESSION
            Opt::Short('a') | Opt::Long("address") => result.address = opts.value_str()?,
            Opt::Short('p') | Opt::Long("port") => result.port = opts.value_str()?,
            // An unknown option was passed
            opt => return Err(Error::UnknownOpt(opt)),
        }
    }

    Ok(result)
}
