pub trait StateMachine<Transition, Snapshot> {
    type Snapshot;

    fn apply(&mut self, transition: Transition);
    fn snapshot(&mut self) -> Self::Snapshot;
}
