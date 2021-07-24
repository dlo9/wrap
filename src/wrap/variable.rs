use serde_derive::Deserialize;
use std::{collections::HashMap, convert::TryFrom, env};
use anyhow::{bail, Result};

#[derive(Debug, Default, Deserialize)]
#[serde(try_from = "HashMap<String, String>")]
pub struct Variables(HashMap<String, String>);

impl TryFrom<HashMap<String, String>> for Variables {
    type Error = anyhow::Error;

    fn try_from(config_variables: HashMap<String, String>) -> Result<Variables> {
        // Start with environment variables
        let mut variables: HashMap<String, String> = env::vars().collect();

        // Insert variables from the config if not overridden by the environment
        for (key, value) in config_variables {
            if !variables.contains_key(&key) {
                // Expand variables as we insert them
                let expended_value = CharacterMode::parse(&value, |var|
                    variables.get(var)
                        .map(|s| s.as_str())
                        .unwrap_or_default()
                )?;
                variables.insert(key, expended_value);
            }
        }

        Ok(Variables(variables))
    }
}

enum CharacterMode {
    Literal,
    Escape,
    VariableStart,
    VariableMiddle(bool, String), // has braces, variable name
    VariableEnd(String), // variable name
}

enum CharType {
    Escape,
    Tilde,
    VariableStart,
    StartingBrace,
    EndingBrace,
    Literal,
}

impl CharType {
    pub fn is_variable(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
}

impl From<char> for CharType {
    fn from(c: char) -> CharType {
        use CharType::*;

        match c {
            '\\' => Escape,
            '~' => Tilde,
            '$' => VariableStart,
            '{' => StartingBrace,
            '}' => EndingBrace,
            _ => Literal,
        }
    }
}

impl CharacterMode {
    fn parse<'a>(input: &str, get_var: impl Fn(&str) -> &'a str) -> Result<String> {
        use CharacterMode as M;
        use CharType::*;

        let mut output = String::with_capacity(input.len());
        let mut state = M::Literal;
        for c in input.chars() {
            // Move to next state
            state = match (state, CharType::from(c)) {
                (M::Escape, Escape | Tilde | VariableStart) => M::Literal,
                (M::Escape, _) => bail!("Not a valid escape sequence: {}", c),

                (M::VariableStart, StartingBrace) => M::VariableMiddle(true, String::new()),
                (M::VariableStart, Literal) if !CharType::is_variable(c) => { bail!("Alphanumeric variable or starting brace expected: {}", c) },
                (M::VariableStart, Literal) => M::VariableMiddle(false, c.into()),
                (M::VariableStart, _) => bail!("Only `{` or an alphanumeric character allowed after $"),

                (M::VariableMiddle(true, s), EndingBrace) => M::VariableEnd(s),
                (M::VariableMiddle(true, s), Literal) if !CharType::is_variable(c) => { bail!("Alphanumeric variable name or ending brace expected for variable {}. Found: {}", s, c) },
                (M::VariableMiddle(false, s), Literal) if !CharType::is_variable(c) => M::VariableEnd(s),
                (M::VariableMiddle(b, mut s), Literal) => { s.push(c); M::VariableMiddle(b, s) },

                (_, Escape) => M::Escape,
                (_, Tilde) => M::VariableEnd("HOME".to_string()),
                (_, VariableStart) => M::VariableStart,
                _ => { M::Literal },
            };

            // Do action
            match &state {
                M::Literal => output.push(c),
                M::VariableEnd(s) => {
                    output.push_str(&get_var(s))
                },
                _ => {},
            }
        }

        match &state {
            M::Escape => bail!("Cannot end in the escape character"),
            M::VariableStart => bail!("No variable name given"),
            M::VariableMiddle(true, s) => bail!("No ending brace for variable: {}", s),
            M::VariableMiddle(false, s) => output.push_str(&get_var(s)),
            _ => {},
        };

        Ok(output)
    }
}

impl Variables {
    pub fn apply(&self, arguments: &Vec<String>) -> Result<Vec<String>> {
        let mut output = Vec::with_capacity(arguments.len());

        for argument in arguments {
            output.push(CharacterMode::parse(argument, |var| 
                self.0
                    .get(var)
                    .map(|s| s.as_str())
                    .unwrap_or_default()
            )?);
        }

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    fn get_var<'a>(var: &str) -> &'a str {
        match var {
            "VAR1" => "var1_output",
            _ => "",
        }
    }

    #[test]
    fn character_mode_parse_literal_input_literal_output() {
        let input = "nothing special here";
        let expected = input.to_owned();
        let actual = CharacterMode::parse(input, get_var).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn character_mode_parse_variable_input_replaced_output() {
        let input = "$VAR1";
        let expected = "var1_output";
        let actual = CharacterMode::parse(input, get_var).unwrap();
        assert_eq!(expected, actual);
    }
}
