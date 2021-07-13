use serde_derive::Deserialize;
use std::{
    cmp::{
        Eq,
        PartialEq,
        PartialOrd,
        Ord,
    },
    collections::HashSet,
    ops::Deref,
};

// TODO: use a structure that allows multiple of the same element (bag/multiset)
#[derive(Debug, Deserialize)]
pub struct Trigger(HashSet<String>);

impl Deref for Trigger {
    type Target = HashSet<String>;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl Eq for Trigger {}
impl PartialEq for Trigger {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl PartialOrd for Trigger {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Trigger {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len().cmp(&other.len())
    }
}

// Must use a custom iterator to indicate consumption of `trigger`
struct TriggeredIter<I> {
    trigger: Trigger,
    args: I
}

impl<I> TriggeredIter<I> {
    fn new(trigger: Trigger, args: I) ->TriggeredIter<I> {
        TriggeredIter{
            trigger,
            args,
        }
    }
}

impl<I: Iterator> Iterator for TriggeredIter<I>
where
    I::Item: AsRef<str>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.args.next().filter(|arg| self.trigger.remove(arg.as_ref()))
    }
}

impl Trigger {
    /// Returns true if this trigger is triggered by the given arguments
    pub fn matches<'iter> (&self, args: impl IntoIterator<Item = impl AsRef<str>>) -> bool {
        let missing_triggers = self.clone();

        args.into_iter().for_each(|arg| {
            missing_triggers.remove(arg.as_ref());
        });

        return missing_triggers.is_empty();
    }

    // Returns the arguments with the trigger removed
    pub fn remove_from<I: Iterator>(self, args: I) -> impl Iterator<Item = I::Item>
    where
        I::Item: AsRef<str>,
    {
        TriggeredIter::new(self, args.into_iter())
    }
}