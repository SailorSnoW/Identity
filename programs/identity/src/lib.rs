mod error;
mod event;
mod identity;

use anchor_lang::prelude::*;
use error::IdentityError;
use event::update_event;
use identity::Identity;

use std::convert::TryInto;

use crate::identity::types::Updatable;

declare_id!("GxyJLSDuC7BkeorKoMg87uhaXvuaUxDjDKT7iQWCbxXJ");

#[program]
pub mod program_identity {
    use super::*;

    /// Représente le nombre de seconde requis depuis la création de l'identité pour pouvoir fermer l'account
    pub const CAN_DELETE_AFTER: i64 = 31556926 * 2;

    /// Permet à un utilisateur sans identité de créer son identité
    pub fn create_identity(
        ctx: Context<CreateIdentity>,
        first_name: String,
        last_name: String,
        username: String,
        birth: i64,
        mail: Option<String>,
    ) -> Result<()> {
        ctx.accounts.identity.set_inner(Identity::new(
            &ctx,
            first_name.parse()?,
            last_name.parse()?,
            username.parse()?,
            birth,
            mail.try_into()?,
        ));

        // Emet un `Event` signifiant qu'une nouvelle identité est crée
        emit!(event::IdentityCreated {
            pubkey: ctx.accounts.user.key(),
            username,
            timestamp: ctx.accounts.identity.created
        });

        Ok(())
    }

    /// Permet à un utilisateur de mettre à jour son prénom
    pub fn update_name(ctx: Context<UpdateIdentity>, first_name: String) -> Result<()> {
        let old_data = ctx.accounts.identity.first_name.clone();
        ctx.accounts.identity.first_name.update(first_name.parse()?);

        emit!(update_event::FirstNameUpdated {
            pubkey: ctx.accounts.user.key(),
            old_data: old_data.to_string(),
            new_data: first_name,
            timestamp: Clock::get().unwrap().unix_timestamp
        });

        Ok(())
    }

    /// Permet à un utilisateur de mettre à jour son pseudonyme
    pub fn update_username(ctx: Context<UpdateIdentity>, username: String) -> Result<()> {
        let old_data = ctx.accounts.identity.username.clone();
        ctx.accounts.identity.username.update(username.parse()?);

        emit!(update_event::UsernameUpdated {
            pubkey: ctx.accounts.user.key(),
            old_data: old_data.to_string(),
            new_data: username,
            timestamp: Clock::get().unwrap().unix_timestamp
        });

        Ok(())
    }

    /// Permet à un utilisateur de mettre à jour ou supprimer son mail
    pub fn update_mail(ctx: Context<UpdateIdentity>, mail: Option<String>) -> anchor_lang::Result<()> {
        let old_data = ctx.accounts.identity.mail.clone();
        ctx.accounts.identity.mail.update(mail.clone().try_into()?);

        emit!(update_event::MailUpdated {
            pubkey: ctx.accounts.user.key(),
            old_data: old_data.try_into().unwrap(),
            new_data: mail,
            timestamp: Clock::get().unwrap().unix_timestamp
        });

        Ok(())
    }

    /// Permet à un utilisateur ayant une identité depuis plus de 2 ans
    /// de fermer l'account contenant son identité
    pub fn delete_identity(ctx: Context<CloseIdentity>) -> Result<()> {
        let now = Clock::get().unwrap().unix_timestamp;
        let created = ctx.accounts.identity.created;
        let since = now - created;

        require_gt!(since, CAN_DELETE_AFTER, IdentityError::TimeNotPassed);

        emit!(event::IdentityClosed {
            pubkey: ctx.accounts.user.key(),
            timestamp: Clock::get().unwrap().unix_timestamp
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateIdentity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = Identity::MAX_IDENTITY_SIZE + 8,
        seeds = [&Identity::BUMP_STRING_STORE, user.key().as_ref()], bump
    )]
    pub identity: Account<'info, Identity>,
    pub system_program: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdateIdentity<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [&Identity::BUMP_STRING_STORE, user.key().as_ref()], bump = identity.bump
    )]
    pub identity: Account<'info, Identity>,
}

#[derive(Accounts)]
pub struct CloseIdentity<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        close = user,
        seeds = [&Identity::BUMP_STRING_STORE, user.key().as_ref()], bump = identity.bump
    )]
    pub identity: Account<'info, Identity>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FirstNameTest{
    pub test: String
}
