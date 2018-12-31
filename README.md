# Overview
This project is serving a few main goals.  
1. play around with [Raft](https://raft.github.io/).
2. play around with languages.  
3. play around with raspberry pi.

The idea here is to build basic Raft implementations in various languages, perhaps some day having the different implementations actually cooperate in a single heterogenous Raft cluster.

This toy application is responsible for choosing colors.  The color selected will be output to stdout, but also echoed to a udp server which can set the LED on the raspberry pi [sense hat](https://www.raspberrypi.org/products/sense-hat/).  That part is optional (stdout suffices, albeit is boring).  Eventually the UDP server may be replaced with language specific drivers for the sense hat (see goal 3 above).

Unless otherwise specified, everything in this repository is covered by the MIT License.

# Wire Protocol
## LED Server
The LED server will listen to UDP messages in the form of:

```
pixel 1,2,3,4,5
```

which will set pixel `(1,2)` to `[r,b,g]` value `[3,4,5]`.

Leaders will be indicated by a stripe on the top of the sense hat of length equal to the leader id:
```
###xxxxx
xxxxxxxx
xxxxxxxx
xxxxxxxx
xxxxxxxx
xxxxxxxx
xxxxxxxx
xxxxxxxx
```

Indicates that node 3 is the leader.

## Inter-cluster messages
The Raft participants can send messages to each other with the following format, which is generally JSON preceded by a verb:

#### Command
Set color to `(r,g,b)=(1,2,3)`:
```
CMD [1, 2, 3]
```

#### RequestVote
Example (note last_position is an array of `[index, term]`:
```
REQ {"term":1,"candidate":"127.0.0.4:10001","last_position":[0,0]}
```

#### VoteResponse
Example:
```
VOT {"term":1,"voter":"127.0.0.3:10001","candidate":"127.0.0.4:10001","vote":"Grant"}
```

#### AppendEntries
Example:
* Note that previous_position is `[index, term]` and position is `[index, term]`
* Note that the command in change state is our color command, and looks like `[r,g,b]`
```
APP {"term":1,"leader":"127.0.0.4:10001","previous_position":[0,0],"commit_index":0,"entries":[{"position":[1,1],"payload":{"Noop":"127.0.0.4:10001"}},{"position":[2,1],"payload":{"ChangeState":[1,2,3]}}]}
```

#### AppendAck
```
ACK [sender,[new_latest_index,new_latest_term]]
```
Example:
```
ACK ["127.0.0.3:10001",[2,1]]
```

#### AppendNack
```
NCK [term_of_sender, prev_index_from_append_msg, sender]
```
Example:
```
NCK [2, 1, "127.0.0.3:10001"]
```

# Discovery
Initially the participants will find each other via a hard-coded list.  Participants will know their ip and port and their peers at startup.

# Advanced

## setup

* [Raspberry Pi Model 3B](https://www.raspberrypi.org/products/raspberry-pi-3-model-b/)
* [sense hat](https://www.raspberrypi.org/products/sense-hat/)

## iptables
Play around with firewalls and watch the pixels go stale until majority is crossed, then the whole thing falls apart.

```
iptables -I INPUT -p udp  --dport $PORT_TO_BLOCK -j DROP
```
