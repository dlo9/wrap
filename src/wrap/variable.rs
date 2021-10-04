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
    pub fn expand(argument: &str, variables: &HashMap<String, String>, positionals: &[String]) -> Result<(String, usize)> {
        // Parse the argument
        let pairs = ArgumentParser::parse(Rule::argument, argument)
            .with_context(|| "Argument parsing failed")?;

        // Expand the argument's variables
        let mut argument = String::new();
        let mut positionals_used = 0;
        for pair in pairs.flatten() {
            match pair.as_rule() {
                Rule::literal => argument.push_str(pair.as_str()),
                Rule::tilde => argument.push_str(Self::get_var("HOME", variables)),
                Rule::variable_identifier => argument.push_str(Self::get_var(pair.as_str(), variables)),
                Rule::positional_identifier => {
                    positionals_used += 1;
                    argument.push_str(Self::get_positional(positionals_used - 1, positionals)?)
                }
                _ => {},
            };
        }

        Ok((argument, positionals_used))
    }

    fn get_var<'a>(name: &str, variables: &'a HashMap<String, String>) -> &'a str {
        variables
            .get(name)
            .map(|s| s.as_str())
            .unwrap_or_default()
    }

    fn get_positional<'a>(index: usize, positionals: &'a [String]) -> Result<&'a String> {
        positionals
            .get(index)
            .ok_or(anyhow!("Missing positional #{}", index + 1))
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

        let mut arguments_to_skip = 0;
        for (index, argument) in arguments.iter().enumerate() {
            if arguments_to_skip > 0 {
                arguments_to_skip -= 1;
                continue;
            }

            let positionals = if arguments.len() > index {
                &arguments[index+1..]
            } else {
                &arguments[0..0]
            };

            let results = ArgumentParser::expand(&argument, &self.0, positionals)?;
            output.push(results.0);
            arguments_to_skip = results.1;
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
        let input = "The first positional is: $#";
        let expected = ("The first positional is: first".to_string(), 1);
        let actual = ArgumentParser::expand(input, &vars(), &vec!["first".to_string()]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn expand__two_positional_inputs__replaced_output_one_skipped() {
        let input = "The first positional is: $#, ${#}";
        let expected = ("The first positional is: first, second".to_string(), 2);
        let actual = ArgumentParser::expand(input, &vars(), &vec!["first".to_string(), "second".to_string()]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn expand__missing_positional__error() {
        let input = "The first positional is: $#";
        let expected = "Missing positional #1";
        let actual = ArgumentParser::expand(input, &vars(), &vec![]).unwrap_err().to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn apply__one_positional__one_skipped() {
        let input = vec!["first: 1".to_string(), "second: $#".to_string(), "2".to_string(), "third: 3".to_string()];
        let expected = vec!["first: 1", "second: 2", "third: 3"];
        let actual = Variables(HashMap::new()).apply(&input).unwrap();
        // let actual = ArgumentParser::expand(input, &vars(), &vec![]).unwrap_err().to_string();
        assert_eq!(expected, actual);
    }
}
