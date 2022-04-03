use super::*;
use std::str::from_utf8;

/// Définit la structure d'une identité d'un utilisateur
#[account]
pub struct Identity {
    pub first_name: String,   // 128 + 4 = 132
    pub last_name: String,    // 128 + 4 = 132
    pub username: String,     // 128 + 4 = 132
    pub birth: i64,           // 8
    pub mail: Option<String>, // 128 + 1 = 129
    pub created: i64,         // 8
    pub bump: u8,             // 1
}

impl Identity {
    /// Représente la taille maximale en bytes qu'un champ de type `String` peut contenir
    pub const MAX_STRING_SIZE: usize = 128;
    /// Représente la taille maximale en bytes que la structure `Identity` peut contenir
    pub const MAX_IDENTITY_SIZE: usize = 132 + 132 + 132 + 8 + 129 + 8 + 1;
    pub const BUMP_STRING_STORE: [u8; 8] = *b"Identity";

    pub fn try_new(
        ctx: &Context<CreateIdentity>,
        first_name: &str,
        last_name: &str,
        username: &str,
        birth: i64,
        mail: Option<String>,
    ) -> Result<Self> {
        require_gte!(
            Identity::MAX_STRING_SIZE,
            first_name.len(),
            IdentityError::StringTooLarge
        );
        require_gte!(
            Identity::MAX_STRING_SIZE,
            last_name.len(),
            IdentityError::StringTooLarge
        );
        require_gte!(
            Identity::MAX_STRING_SIZE,
            username.len(),
            IdentityError::StringTooLarge
        );
        if mail.is_some() {
            require_gte!(
                Identity::MAX_STRING_SIZE,
                mail.as_ref().unwrap().len(),
                IdentityError::StringTooLarge
            );
        }

        Ok(Identity {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            username: username.to_string(),
            birth,
            mail,
            created: Clock::get().unwrap().unix_timestamp,
            bump: *ctx
                .bumps
                .get("identity")
                .unwrap(),
        })
    }
}
