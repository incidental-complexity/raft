use crate::membership::Membership;
use crate::membership::Member;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SequenceNumber(u64);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Term(u64);

#[derive(Clone, PartialEq, Eq)]
pub enum Type<State> {
    Noop,
    ChangeMembership(Membership),
    ChangeState(State),
}

pub struct Entry {
    sn: SequenceNumber,
    term: Term,
    leader: Member,
}