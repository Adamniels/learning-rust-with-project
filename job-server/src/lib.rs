//! A synchronous in-memory job server.
//!
//! The crate exposes the job model and the [`JobServer`] use cases through a
//! small public facade. Queue management, execution simulation, and state
//! transitions remain implementation details of the library crate.

mod error;
mod job;
mod server;

pub use error::JobError;
pub use job::{Job, JobKind, JobOperation, JobState, JobStateKind};
pub use server::JobServer;
