use sqlx::{Postgres, Transaction};

pub mod claims;
pub mod events;
pub mod parties;

pub type PostgresTx<'t> = Transaction<'t, Postgres>;
