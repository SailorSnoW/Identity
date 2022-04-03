use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::result::Result;
use std::str::FromStr;

use crate::error::IdentityError;
use identity_macros::{IdentityString, Updatable};

use super::*;

// TRAITS DEFINITION

/// Trait indiquant qu'un type de `Identity` peut être modifié par le program
pub trait Updatable {
    /// Remplace la valeur de `self` par `new_data`
    fn update(&mut self, new_data: Self);
}

// TYPES DEFINITION

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Updatable, IdentityString)]
pub struct FirstName(String);

#[derive(AnchorSerialize, AnchorDeserialize, Clone, IdentityString)]
pub struct LastName(String);

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Updatable, IdentityString)]
pub struct Username(String);

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Updatable)]
pub struct Mail(Option<String>);

impl<T: ToString> TryFrom<Option<T>> for Mail {
    type Error = IdentityError;

    fn try_from(mail: Option<T>) -> Result<Self, Self::Error> {
        match mail {
            Some(m) => {
                let m_string = m.to_string();
                if m_string.len() > Identity::MAX_STRING_SIZE {
                    return Err(IdentityError::StringTooLarge);
                }
                Ok(Mail(Some(m_string)))
            }
            None => Ok(Mail(None)),
        }
    }
}

impl From<Mail> for Option<String> {
    fn from(m: Mail) -> Option<String> {
        m.0
    }
}