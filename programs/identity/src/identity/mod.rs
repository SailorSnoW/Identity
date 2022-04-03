pub mod types;

use super::*;
use std::str::from_utf8;
use types::{FirstName, LastName, Mail, Username};

/// Définit la structure d'une identité d'un utilisateur
#[account]
pub struct Identity {
    pub first_name: FirstName, // 128 + 4 = 132
    pub last_name: LastName,   // 128 + 4 = 132
    pub username: Username,    // 128 + 4 = 132
    pub birth: i64,            // 8
    pub mail: Mail,            // 128 + 1 = 129
    pub created: i64,          // 8
    pub bump: u8,              // 1
}

impl Identity {
    /// Représente la taille maximale en bytes qu'un champ de type `String` peut contenir
    pub const MAX_STRING_SIZE: usize = 128;
    /// Représente la taille maximale en bytes que la structure `Identity` peut contenir
    pub const MAX_IDENTITY_SIZE: usize = 132 + 132 + 132 + 8 + 129 + 8 + 1;
    pub const BUMP_STRING_STORE: [u8; 8] = *b"Identity";

    pub fn new(
        ctx: &Context<CreateIdentity>,
        first_name: FirstName,
        last_name: LastName,
        username: Username,
        birth: i64,
        mail: Mail,
    ) -> Self {
        Identity {
            first_name,
            last_name,
            username,
            birth,
            mail,
            created: Clock::get().unwrap().unix_timestamp,
            bump: *ctx
                .bumps
                .get(from_utf8(&Identity::BUMP_STRING_STORE).unwrap())
                .unwrap(),
        }
    }
}
