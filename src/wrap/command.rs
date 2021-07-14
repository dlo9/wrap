use std::fmt::{
    Display,
    Formatter,
    Result,
};

#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub arguments: Vec<String>,
}

impl Into<exec::Command> for Command {
    fn into(self) -> exec::Command {
        let mut cmd = exec::Command::new(self.program);
        cmd.args(&self.arguments);

        cmd
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.program)?;

        // TODO: only quote if `argument` has spaces
        // TODO: escape quotes in `argument`
        // Or, just print as an array...
        for argument in self.arguments.iter() {
            write!(f, " \"{}\"", argument)?
        }

        Ok(())
    }
}
