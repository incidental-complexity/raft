=== TODOs ===
==== High Level ====
* Move these to issue tracker
* Log Compaction/Snapshotting
* Membership Changes
* Durable Journal
* Membership discovery

==== Specific ====
* Make sure the entries being inserted into the log are actually for the correct index
* Ignore re-transmitting to servers if their match index is greater than their NACK (implies delayed NACK)
* Have leaders who haven't received an ACK within election timeout to turn into followers
* ACK the client properly (only after the value is committed, and send back the result of the state machine apply, not just the log apply)
* Reduce cloning
