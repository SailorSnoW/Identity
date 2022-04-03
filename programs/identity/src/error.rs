use anchor_lang::error_code;

#[error_code]
pub enum IdentityError {
    #[msg("Specified string is higher than the expected maximum space")]
    StringTooLarge,
    #[msg("2 year is needed since the creation of the identity to be closed")]
    TimeNotPassed,
}
