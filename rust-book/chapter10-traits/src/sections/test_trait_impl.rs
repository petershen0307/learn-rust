use crate::sections::summary::{AnotherSummary, Summary};

pub struct TraitsHaveSameFn {}

impl Summary for TraitsHaveSameFn {
    fn summarize_author(&self) -> String {
        String::from("TraitsHaveSameFn::Summary")
    }
}

impl AnotherSummary for TraitsHaveSameFn {
    fn summarize_author(&self) -> String {
        String::from("TraitsHaveSameFn::AnotherSummary")
    }
}

pub struct OverWriteDefaultTraitImpl {}
impl Summary for OverWriteDefaultTraitImpl {
    fn summarize_author(&self) -> String {
        String::from("OverWriteDefaultTraitImpl::Summary")
    }
    fn summarize(&self) -> String {
        String::from("Overwrite default implementation")
    }
}
