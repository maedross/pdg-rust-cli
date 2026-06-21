use super::concepts::Player;
    use std::fmt;
    // Cards
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum EventType {
        Standard,
        Epoch,
        Pivotal,
    }

    #[derive(Clone, Debug)]
    pub struct Event {
        pub name: String,
        pub eligibility: Vec<Player>,
        pub unshaded: Option<u8>,
        pub shaded: Option<u8>,
        pub historical_notes: String,
        pub event_type: EventType,
    }

    impl fmt::Display for Event {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.eligibility)
        }
    }