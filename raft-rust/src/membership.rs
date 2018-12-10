use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq)]
pub enum Membership {
    Stable(HashSet<Member>),
    Transitioning{ old: HashSet<Member>, new: HashSet<Member> },
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Member {
    host: String,
    port: u16,
}