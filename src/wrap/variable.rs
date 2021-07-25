use pest::Parser;
use serde_derive::Deserialize;
use std::{collections::HashMap, convert::TryFrom, env};
use anyhow::{Context, Result};
//use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "wrap/variable.pest"]
pub struct ArgumentParser;

impl ArgumentParser {
    // TODO: return Cow
    pub fn expand(argument: &str, variables: &HashMap<String, String>) -> Result<String> {
        // Parse the argument
        let parsed = ArgumentParser::parse(Rule::argument, argument)
            .with_context(|| "Argument parsing failed")?
            .next()
            .expect("Argument parsing will always produce a token")
            .into_inner();

        // Expand the argument's variables
        let mut argument = String::new();
        for pair in parsed {
            match pair.as_rule() {
                Rule::literal => argument.push_str(pair.as_str()),
                Rule::tilde => argument.push_str(Self::get_var("HOME", variables)),
                Rule::variable => {
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::variable_ident => argument.push_str(Self::get_var(pair.as_str(), variables)),
                            _ => {},
                        }
                    }
                }
                _ => {},
            };
        }

        Ok(argument)
    }

    fn get_var<'a>(name: &str, variables: &'a HashMap<String, String>) -> &'a str {
        variables
            .get(name)
            .map(|s| s.as_str())
            .unwrap_or_default()
    }

}

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
                variables.insert(key, ArgumentParser::expand(&value, &variables)?);
            }
        }

        Ok(Variables(variables))
    }
}

impl Variables {
    pub fn apply(&self, arguments: &Vec<String>) -> Result<Vec<String>> {
        let mut output = Vec::with_capacity(arguments.len());

        for argument in arguments {
            output.push(ArgumentParser::expand(&argument, &self.0)?);
        }

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn vars() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("VAR1".to_string(), "var1_output".to_string());
        map
    }

    #[test]
    fn argument_parse_empty_input_empty_output() {
        let input = "";
        let expected = input.to_owned();
        let actual = ArgumentParser::expand(input, &vars()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn argument_parse_literal_input_literal_output() {
        let input = "nothing special here";
        let expected = input.to_owned();
        let actual = ArgumentParser::expand(input, &vars()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn argument_parse_variable_input_replaced_output() {
        let input = "${VAR1}";
        let expected = "var1_output";
        let actual = ArgumentParser::expand(input, &vars()).unwrap();
        assert_eq!(expected, actual);
    }
}
