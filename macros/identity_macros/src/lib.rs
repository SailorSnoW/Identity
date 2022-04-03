extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, __private::quote::quote};

/// Derive le trait `Updatable`.
#[proc_macro_derive(Updatable)]
pub fn derive_updatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl Updatable for #name {
            fn update(&mut self, new_data: Self) {
                *self = new_data
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive les traits `FromStr` et `Display`.
///
/// `from_str()` valide que la chaine de caractères passée en argument ne dépasse pas
/// la limite de taille maximale autorisée pour les `String` contenu dans
/// une structure `Identity`.
/// Retourne une erreur `IdentityError::StringTooLarge` dans le cas contraire.
#[proc_macro_derive(IdentityString)]
pub fn derive_identity_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl FromStr for #name {
            type Err = IdentityError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s.len() > Identity::MAX_STRING_SIZE {
                    return Err(IdentityError::StringTooLarge)
                }
                Ok(Self(s.to_string()))
            }
        }

        impl Display for #name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };

    TokenStream::from(expanded)
}
