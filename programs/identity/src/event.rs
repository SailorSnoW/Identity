use anchor_lang::prelude::*;

#[event]
pub struct IdentityCreated {
    pub pubkey: Pubkey,
    pub username: String,
    pub timestamp: i64,
}

#[event]
pub struct IdentityClosed {
    pub pubkey: Pubkey,
    pub timestamp: i64,
}

pub mod update_event {
    use super::*;

    #[event]
    pub struct MailUpdated {
        pub pubkey: Pubkey,
        pub old_data: Option<String>,
        pub new_data: Option<String>,
        pub timestamp: i64,
    }

    #[event]
    pub struct UsernameUpdated {
        pub pubkey: Pubkey,
        pub old_data: String,
        pub new_data: String,
        pub timestamp: i64,
    }

    #[event]
    pub struct FirstNameUpdated {
        pub pubkey: Pubkey,
        pub old_data: String,
        pub new_data: String,
        pub timestamp: i64,
    }
}
