use std::collections::HashSet;
use std::fmt;

/// An `Endpoint` is uniquely identified as a host and port, which also doubles as
/// an address for communicating with that actor.
// TODO this is being cloned all over the place.  Maybe keep endpoints in some central
// TODO lookup and pass around instead some nonce that is cheap and implements Copy.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Endpoint(pub String);
impl fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The cluster `Membership`, which can either be stable or in the process of transitioning
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Membership {

    /// Stable membership.  Decisions must be made in a majority of the set.
    Stable(HashSet<Endpoint>),

    /// Transitioning membership.  Decisions must be made in a majority of old and a majority of new
    Transitioning{ old: HashSet<Endpoint>, new: HashSet<Endpoint> },

}

impl Membership {
    pub fn parse(input: String) -> Self {
        let members = input.split(",").map(|ep| Endpoint(ep.trim().to_string())).collect::<HashSet<Endpoint>>();
        Membership::Stable(members)
    }

    pub fn is_majority(&self, pool: &HashSet<Endpoint>) -> bool {

        // an inner function!  nuts, right?
        // return true if the intersection of a and b is larger than half of b
        fn is_majority_(a: &HashSet<Endpoint>, b: &HashSet<Endpoint>) -> bool {
            let mut count = 0;
            for i in a {
                if b.contains(i) {
                    count += 1;
                }
            }
            count > b.len()/2
        }

        match self {
            Membership::Stable(members) => is_majority_(pool, members),
            Membership::Transitioning{old, new} => is_majority_(pool, old) && is_majority_(pool, new),
        }
    }

    pub fn as_set(&self) -> HashSet<Endpoint> {
        match self {
            Membership::Stable(set1) => set1.iter().map(|m| m.clone()).collect(),
            Membership::Transitioning{old: set1, new: set2} => set1.union(set2).map(|m| m.clone()).collect(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::raft::membership::Endpoint;

    use super::*;

    #[test]
    pub fn test_majority_stable() {

        let m1 = Endpoint("foo:1".to_string());
        let m2 = Endpoint("foo:2".to_string());
        let m3 = Endpoint("foo:3".to_string());
        let mut set = HashSet::new();
        set.insert(m1.clone());
        set.insert(m2.clone());
        set.insert(m3.clone());
        let membership = Membership::Stable(set);

        set = HashSet::new();
        set.insert(m1.clone());
        assert!(!membership.is_majority(&set));

        set.insert(m2.clone());
        assert!(membership.is_majority(&set));

        // now make sure it's not just raw count that matters
        set = HashSet::new();
        set.insert(Endpoint("bar:1".to_string()));
        set.insert(Endpoint("bar:2".to_string()));
        assert!(!membership.is_majority(&set));
    }

    #[test]
    pub fn test_majority_transitioning() {

        let m1 = Endpoint("foo:1".to_string());
        let m2 = Endpoint("foo:2".to_string());
        let m3 = Endpoint("foo:3".to_string());
        let mut set1 = HashSet::new();
        set1.insert(m1.clone());
        set1.insert(m2.clone());
        set1.insert(m3.clone());

        let m4 = Endpoint("foo:4".to_string());
        let m5 = Endpoint("foo:5".to_string());
        let m6 = Endpoint("foo:6".to_string());
        let mut set2 = HashSet::new();
        set2.insert(m4.clone());
        set2.insert(m5.clone());
        set2.insert(m6.clone());

        let membership = Membership::Transitioning {old: set1, new: set2};

        let mut set = HashSet::new();
        set.insert(m1.clone());
        set.insert(m2.clone());
        set.insert(m4.clone());
        assert!(!membership.is_majority(&set));

        set.insert(m5.clone());
        assert!(membership.is_majority(&set));
    }

    #[test]
    fn parse() {
        let mut endpoints = HashSet::new();
        endpoints.insert(Endpoint("abc:123".to_string()));
        endpoints.insert(Endpoint("xyz:789".to_string()));
        endpoints.insert(Endpoint("mmm:555".to_string()));
        let membership = Membership::Stable(endpoints);
        let membership1 = Membership::parse("abc:123,xyz:789,mmm:555".to_string());
        let membership2 = Membership::parse("abc:123, xyz:789, mmm:555".to_string());
        assert_eq!(membership, membership1);
        assert_eq!(membership, membership2);

    }
}