use shell_escape::escape;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub arguments: Vec<String>,
}

impl From<Command> for exec::Command {
    fn from(val: Command) -> Self {
        let mut cmd = exec::Command::new(val.program);
        cmd.args(&val.arguments);

        cmd
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", escape((&self.program).into()))?;

        for argument in self.arguments.iter() {
            write!(f, " {}", escape(argument.into()))?
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    #[test]
    fn display__no_special_characters__not_escaped() {
        assert_eq!(
            format!(
                "{}",
                Command {
                    program: "echo".to_string(),
                    arguments: vec!("argument1".to_string()),
                }
            ),
            "echo argument1",
        );
    }

    #[test]
    fn display__space_in_string__escaped() {
        assert_eq!(
            format!(
                "{}",
                Command {
                    program: "echo".to_string(),
                    arguments: vec!(r#"argument: "#.to_string()),
                }
            ),
            r#"echo 'argument: '"#,
        );
    }

    #[test]
    fn display__double_quote_in_string__escaped() {
        assert_eq!(
            format!(
                "{}",
                Command {
                    program: "echo".to_string(),
                    arguments: vec!(r#"argument:""#.to_string()),
                }
            ),
            r#"echo 'argument:"'"#,
        );
    }

    #[test]
    fn display__single_quote_in_string__escaped() {
        assert_eq!(
            format!(
                "{}",
                Command {
                    program: "echo".to_string(),
                    arguments: vec!(r#"argument:'"#.to_string()),
                }
            ),
            r#"echo 'argument:'\'''"#,
        );
    }
}
