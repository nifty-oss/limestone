//! Limestone is a program that allows creating accounts that have a unique address generated
//! based on the current slot number. As a result, when the account is closed (burned), it is
//! not possible to recreate the same account address.
//!
//! This feature is useful to avoid reusing an account for something completely different or
//! in cases when applications or indexers store any information about the account, which could
//! get out of sync if the account is closed and recreated on the same address.

pub mod entrypoint;
pub mod instruction;
pub mod processor;

solana_program::declare_id!("EPHSqv4H9HG5xy1kQaQaLN14zyBP36Jzq7hrQ2ZEZbBj");
