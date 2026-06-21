use colored::Colorize;
    use std::fmt;
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Player {
        Civitates,
        Dux,
        Saxons,
        Scotti,
    }

    impl fmt::Display for Player {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Player::Civitates => write!(f, "{}", "Civitates".blue()),
                Player::Dux => write!(f, "{}", "Dux".red()),
                Player::Saxons => write!(f, "{}", "Saxons".black()),
                Player::Scotti => write!(f, "{}", "Scotti".green()),
            }
        }
    }

    impl fmt::Debug for Player {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Player::Civitates => write!(f, "{}", "Civitates".blue()),
                Player::Dux => write!(f, "{}", "Dux".red()),
                Player::Saxons => write!(f, "{}", "Saxons".black()),
                Player::Scotti => write!(f, "{}", "Scotti".green()),
            }
        }
    }