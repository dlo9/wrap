use std::fmt::{
    Display,
    Formatter,
    Result,
};

#[derive(Debug)]
pub struct Command<I>
{
    process: String,
    arguments: I,
}

impl<I: IntoIterator> Command<I>
where
    I::Item: AsRef<str>
{
    pub fn new(process: String, arguments: I) -> Command<I> {
        Command{
            process,
            arguments,
        }
    }
}

impl<I: Iterator> Into<exec::Command> for Command<I>
where
    I::Item: AsRef<str>
{
    fn into(self) -> exec::Command {
        let cmd = exec::Command::new(self.process);
        for argument in self.arguments {
            cmd.arg(argument.as_ref());
        }

        cmd
    }
}

impl<I: Iterator> Display for Command<I>
where
    I::Item: AsRef<str>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.process)?;

        // TODO: only quote if `argument` has spaces
        // TODO: escape quotes in `argument`
        // Or, just print as an array...
        for argument in self.arguments {
            write!(f, " \"{}\"", argument.as_ref())?
        }

        Ok(())
    }
}