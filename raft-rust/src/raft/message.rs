use std::fmt;
use std::fmt::Debug;

use crate::raft::log::LogEntry;
use crate::raft::log::LogPosition;
use crate::raft::membership::Endpoint;
use crate::raft::state::Index;
use crate::raft::state::Term;

// -----------------------------
// REQUEST_VOTE struct
// -----------------------------
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct RequestVote{ pub term: Term, pub candidate: Endpoint, pub last_position: LogPosition }

// -----------------------------
// VOTE_RESPONSE struct
// -----------------------------
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Vote { Grant, Deny }
#[derive(Debug, Serialize, Deserialize)]
pub struct VoteResponse{ pub term: Term, pub voter: Endpoint, pub candidate: Endpoint, pub vote: Vote }

// -----------------------------
// APPEND_ENTRIES struct
// -----------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct AppendEntries<Cmd> {
    pub term: Term,
    pub leader: Endpoint,
    pub previous_position: LogPosition,
    pub commit_index: Index,

    // starts with index == previous_position.0 + 1 and goes from there
    pub entries: Vec<LogEntry<Cmd>>,
}
impl<Cmd> AppendEntries<Cmd> {
    pub fn new(term: Term, leader: Endpoint, previous_position: LogPosition, commit_index: Index, entries: Vec<LogEntry<Cmd>>) -> Self {
        Self {
            term,
            leader,
            previous_position,
            commit_index,
            entries,
        }
    }
}

/// An ack that contains the latest position of the log after the apply succeeded
/// (Sender, LogPosition)
#[derive(Debug, Serialize, Deserialize)]
pub struct AppendAck(pub Endpoint, pub LogPosition);

/// An negative acknowledgement that indicates the term in which the AppendEntries was sent,
/// as well as the index that was requested in the AppendEntries previous log position.  These
/// two values (current term and requested index) are used by the recipient of the nack to properly
/// reason about the response.  Otherwise delayed or out of order responses to the leader could
/// confuse the leader's state.
///
/// In other words, this Term, Index tuple is _not_ a log position, but rather an index and term
/// with separate semantics.
///
/// (Term, Index, Sender)
#[derive(Debug, Serialize, Deserialize)]
pub struct AppendNack(pub Term, pub Index, pub Endpoint);

// -----------------------------
// MESSAGE enum
// -----------------------------
pub enum Contents<Cmd> {
    RequestVote(RequestVote),
    VoteResponse(VoteResponse),

    AppendEntries(AppendEntries<Cmd>),
    AppendAck(AppendAck),
    AppendNack(AppendNack),

    Command(Cmd, Box<dyn 'static + ClientResponder + Send>),
}

impl<Cmd: Debug> fmt::Debug for Contents<Cmd> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Contents::RequestVote(inner) => write!(f, "{:?}", inner),
            Contents::VoteResponse(inner) => write!(f, "{:?}", inner),
            Contents::AppendEntries(inner) => write!(f, "{:?}", inner),
            Contents::AppendAck(inner) => write!(f, "{:?}", inner),
            Contents::AppendNack(inner) => write!(f, "{:?}", inner),
            Contents::Command(cmd, _) => write!(f, "Command({:?})", cmd),
        }
    }
}

pub struct Message<Cmd>(pub Endpoint, pub Contents<Cmd>);

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientResponse {
    Redirect(Endpoint),
    UnknownLeader,
    Received(LogPosition),
}

pub trait ClientResponder {
    fn respond(&mut self, msg: ClientResponse);
}
