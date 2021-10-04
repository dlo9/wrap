use pest::Parser;
use serde_derive::Deserialize;
use std::{collections::HashMap, convert::TryFrom, env};
use anyhow::{anyhow, Context, Result};
use indexmap::IndexMap;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "wrap/variable.pest"]
pub struct ArgumentParser;

impl ArgumentParser {
    // TODO: return Cow
    pub fn expand(argument: &str, variables: &HashMap<String, String>, positionals: &mut Vec<&str>) -> Result<String> {
        // Parse the argument
        let pairs = ArgumentParser::parse(Rule::argument, argument)
            .with_context(|| "Argument parsing failed")?;

        // Expand the argument's variables
        let mut argument = String::new();
        for pair in pairs.flatten() {
            match pair.as_rule() {
                Rule::literal => argument.push_str(pair.as_str()),
                Rule::tilde => argument.push_str(Self::get_var("HOME", variables)),
                Rule::variable_identifier => argument.push_str(Self::get_var(pair.as_str(), variables)),
                Rule::positional_identifier => argument.push_str(Self::get_positional(pair.as_str(), positionals)?),
                Rule::positionals_identifier => {
                    for positional in positionals {
                        argument.push_str(positional)
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

    fn get_positional<'a>(index: &str, positionals: &'a mut Vec<&str>) -> Result<&'a str> {
        let index = index.parse::<usize>()
            .with_context(|| format!("Positional identifier is not a number: {}", index))?;

        positionals
            .get(index - 1)
            .map(|index| *index)
            //.map(|s| s.as_str())
            .ok_or(anyhow!("Missing positional #{}", index))
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(try_from = "IndexMap<String, String>")]
pub struct Variables(HashMap<String, String>);

impl TryFrom<IndexMap<String, String>> for Variables {
    type Error = anyhow::Error;

    fn try_from(config_variables: IndexMap<String, String>) -> Result<Variables> {
        // Start with environment variables
        let mut variables: HashMap<String, String> = env::vars().collect();

        // Insert variables from the config if not overridden by the environment
        for (key, value) in config_variables {
            if !variables.contains_key(&key) {
                // Expand variables as we insert them
                variables.insert(key, ArgumentParser::expand(&value, &variables, &vec![])?.0);
            }
        }

        Ok(Variables(variables))
    }
}

impl Variables {
    pub fn apply(&self, arguments: &Vec<String>) -> Result<Vec<String>> {
        let mut output = Vec::with_capacity(arguments.len());

        for argument in arguments {
            output.push(ArgumentParser::expand(&argument, &self.0, &vec![])?);
        }

        Ok(output)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    fn vars() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("VAR1".to_string(), "var1_output".to_string());
        map
    }

    #[test]
    fn expand__empty_input__empty_output() {
        let input = "";
        let expected = (input.to_string(), 0);
        let actual = ArgumentParser::expand(input, &vars(), &vec![]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn expand__literal_input__literal_output() {
        let input = "nothing special here";
        let expected = (input.to_string(), 0);
        let actual = ArgumentParser::expand(input, &vars(), &vec![]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn expand__variable_input__replaced_output() {
        let input = "${VAR1}";
        let expected = ("var1_output".to_string(), 0);
        let actual = ArgumentParser::expand(input, &vars(), &vec![]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn expand__one_positional_input__replaced_output_one_skipped() {
        let input = "The first positional is: $1";
        let expected = ("The first positional is: first".to_string(), 1);
        let actual = ArgumentParser::expand(input, &vars(), &vec!["first"]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn expand__duplicate_positional_input__replaced_output_one_skipped() {
        let input = "The first positional is: $1, $1";
        let expected = ("The first positional is: first, first".to_string(), 1);
        let actual = ArgumentParser::expand(input, &vars(), &vec!["first"]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn expand__two_positional_inputs__replaced_output_one_skipped() {
        let input = "The first positional is: $1, $2";
        let expected = ("The first positional is: first, second".to_string(), 2);
        let actual = ArgumentParser::expand(input, &vars(), &vec!["first", "second"]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn expand__positionals_input__replaced_output_all_skipped() {
        let input = "The positionals are: $@";
        let expected = ("The positionals are: firstsecond".to_string(), 2);
        let actual = ArgumentParser::expand(input, &vars(), &vec!["first", "second"]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn expand__missing_positional__error() {
        let input = "The first positional is: $1";
        let expected = "Missing positional #1";
        let actual = ArgumentParser::expand(input, &vars(), &vec![]).unwrap_err().to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn expand__missing_positionals__empty_replacement() {
        let input = "The positionals are: $@";
        let expected = ("The positionals are: ".to_string(), 0);
        let actual = ArgumentParser::expand(input, &vars(), &vec![]).unwrap();
        assert_eq!(expected, actual);
    }
}
