use std::thread;
use std::thread::JoinHandle;

use crate::network::Network;
use crate::raft::app::ApplicationStateMachine;
use crate::raft::event_loop;
use crate::raft::log::Log;
use crate::raft::membership::Endpoint;
use crate::raft::membership::Membership;
use crate::raft::state::RaftState;

/// Starts up a server, real or otherwise
pub fn setup_node<Cmd, A, L>(endpoint: Endpoint, membership: Membership, network: &impl Network<Cmd=Cmd>, application: A, log: L) -> JoinHandle<()>
where Cmd: 'static + Send,
      A: 'static + ApplicationStateMachine<Cmd> + Send,
      L: 'static + Log<Cmd> + Send {
    let (receiver, sender) = network.register(endpoint.clone() );
    thread::spawn(move || {
        let state = RaftState::new(endpoint, log, application, membership);
        event_loop(receiver, sender, state);
    })
}
