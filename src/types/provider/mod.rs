// generic
// gitlab (subgroups) style
// azure devops

use crate::{GitUrl, GitUrlParseError};

pub trait GitProvider: Clone + std::fmt::Debug {
    fn from_git_url(url: &GitUrl) -> Self
    where
        Self: Sized;
    //fn get_url(&self, provider: &str
    //fn register(&self);
    //fn unregister(&self);o
    fn to_obj(&self) -> Box<Self> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GenericProvider {
    host: String,
    user: String,
    repo: String,
}

impl GitProvider for GenericProvider {
    fn from_git_url(url: &GitUrl) -> Self {
        GenericProvider {
            host: String::from(""),
            user: String::from(""),
            repo: String::from(""),
        }
    }
}
