//! Refresh tokens: opaque, single use, and revocable.
//!
//! A token is `<tenant id>.<256 random bits>`. The tenant prefix lets every
//! lookup filter on the distribution column; only the SHA-256 of the whole
//! token is stored.

use std::time::{SystemTime, UNIX_EPOCH};

use rand_core::{OsRng, RngCore};
use sea_orm::{DbConn, DbErr, TransactionTrait};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use models::domains::user;

use crate::persistence::refresh_token::{
    delete_expired, find_by_hash, insert_token, revoke_family, revoke_token,
};
use crate::persistence::user::get_user;
use crate::tenant::TenantId;

pub const REFRESH_TTL_SECS: i64 = 7 * 24 * 60 * 60;

pub enum Rotation {
    /// The old token is spent; here are the user as stored now and the new token.
    Rotated(user::Model, String),
    /// Unknown, malformed or expired.
    Invalid,
    /// A spent token came back, so it may be stolen: its whole family is revoked.
    Reused,
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as i64)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn new_token(tenant: TenantId) -> String {
    let mut secret = [0u8; 32];
    OsRng.fill_bytes(&mut secret);
    format!("{}.{}", tenant.0.simple(), hex(&secret))
}

/// The tenant to search in and the hash to search for.
fn parse(token: &str) -> Option<(TenantId, String)> {
    let (tenant, _) = token.split_once('.')?;
    let tenant = TenantId(Uuid::try_parse(tenant).ok()?);
    Some((tenant, hex(&Sha256::digest(token.as_bytes()))))
}

/// Starts a new token family, for a fresh login.
pub async fn start_session(db: &DbConn, tenant: TenantId, user_id: Uuid) -> Result<String, DbErr> {
    let now = now();
    delete_expired(db, tenant, user_id, now).await?;

    let token = new_token(tenant);
    let (_, hash) = parse(&token).ok_or_else(|| DbErr::Custom("bad token".to_string()))?;
    insert_token(
        db,
        tenant,
        user_id,
        Uuid::new_v4(),
        hash,
        now + REFRESH_TTL_SECS,
    )
    .await?;
    Ok(token)
}

/// Exchanges a refresh token for the next one in its family.
pub async fn rotate_session(db: &DbConn, token: &str) -> Result<Rotation, DbErr> {
    let Some((tenant, hash)) = parse(token) else {
        return Ok(Rotation::Invalid);
    };
    let now = now();
    let txn = db.begin().await?;

    let Some(stored) = find_by_hash(&txn, tenant, &hash).await? else {
        return Ok(Rotation::Invalid);
    };
    if stored.revoked_at.is_none() && stored.expires_at <= now {
        return Ok(Rotation::Invalid);
    }
    // the update fails for a token that was spent earlier or concurrently
    if !revoke_token(&txn, tenant, stored.id, now).await? {
        revoke_family(&txn, tenant, stored.family_id, now).await?;
        txn.commit().await?;
        return Ok(Rotation::Reused);
    }

    let Some(user) = get_user(&txn, tenant, stored.user_id).await? else {
        return Ok(Rotation::Invalid);
    };
    let next = new_token(tenant);
    let (_, next_hash) = parse(&next).ok_or_else(|| DbErr::Custom("bad token".to_string()))?;
    insert_token(
        &txn,
        tenant,
        user.id,
        stored.family_id,
        next_hash,
        now + REFRESH_TTL_SECS,
    )
    .await?;

    txn.commit().await?;
    Ok(Rotation::Rotated(user, next))
}

/// Revokes the family of the given token. Unknown tokens are ignored.
pub async fn end_session(db: &DbConn, token: &str) -> Result<(), DbErr> {
    let Some((tenant, hash)) = parse(token) else {
        return Ok(());
    };
    if let Some(stored) = find_by_hash(db, tenant, &hash).await? {
        revoke_family(db, tenant, stored.family_id, now()).await?;
    }
    Ok(())
}
