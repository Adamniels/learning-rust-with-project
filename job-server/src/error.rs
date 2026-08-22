use crate::job::{JobOperation, JobStateKind};

/// An expected failure returned by the job server or a job transition.
pub enum JobError {
    /// No job is waiting in the processing queue.
    QueueEmpty,
    /// A job cannot begin another attempt.
    AttemptsExhausted {
        /// The number of attempts that have already finished.
        completed_attempts: u32,
        /// The maximum number of attempts allowed for the job.
        max_attempts: u32,
    },
    /// The requested operation is not valid from the job's current state.
    InvalidTransition {
        /// The operation that was requested.
        operation: JobOperation,
        /// The state from which the operation was requested.
        from: JobStateKind,
    },
    /// No registered job has the requested identifier.
    JobNotFound {
        /// The identifier supplied by the caller.
        job_id: u64,
    },
}
