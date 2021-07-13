use serde_derive::Deserialize;
use std::{
    cmp::{
        Eq,
        PartialEq,
        PartialOrd,
        Ord,
    },
    collections::BTreeMap,
    ops::Deref,
};

use super::{command::Command, wrapper::{SimpleWrapper, Wrapper}};

#[derive(Debug, Deserialize)]
pub struct Alias {
    pub alias: String,
    pub command: String,
    pub wrappers: Vec<Wrapper>,
}

impl PartialEq<str> for Alias {
    fn eq(&self, other: &str) -> bool {
        self.alias == other
    }
}

impl Alias {
    fn get_command<I: IntoIterator + Copy>(self, args: &I) -> Option<Command<impl Iterator<Item = impl AsRef<str>>>>
    where
        I::Item: AsRef<str>,
    {
        let best_wrapper: Option<SimpleWrapper> = None;
        for wrapper in self.wrappers {
            if let Some(&trigger) = wrapper.longest_match(args.into_iter()) {
                if best_wrapper.map(|w| trigger > w.trigger).unwrap_or(true) {
                    best_wrapper = Some(SimpleWrapper{
                        description: wrapper.description,
                        trigger,
                        arguments: wrapper.arguments,
                    })
                }
            }
        }

        best_wrapper.map(|wrapper| {
            let arguments =  wrapper.arguments.into_iter().map(|s| AsRef::<str>::as_ref(&s));
            let arguments = wrapper.trigger.remove_from(args.into_iter()).chain(arguments);
            //Command::new(self.command, arguments)
        });
        None

        // self.wrappers
        //     .into_iter()
        //     .map(|wrapper| (wrapper, wrapper.longest_match(args.into_iter())))
        //     .filter_map(|(wrapper, trigger)| trigger.map(|trigger| (wrapper, trigger)))
        //     .max_by_key(|&(_, trigger)| trigger)
        //     .map(|(wrapper, trigger)| Command::new(self.command, trigger.remove_from(args.into_iter()).chain(wrapper.)))

        // self.wrappers
        //     .iter()
        //     .map(|wrapper| (wrapper, wrapper.longest_match(args)))
        //     .filter(|(_, trigger)| trigger.is_some())
        //     .max_by(|(_, trigger)| trigger)
        //     .map(|(wrapper, trigger)|)
    }
}
