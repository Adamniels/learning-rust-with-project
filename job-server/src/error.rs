use crate::job::{JobOperation, JobStateKind};
use std::fmt;

/// An expected failure returned by the job server or a job transition.
#[derive(Debug)]
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

impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueueEmpty => write!(f, "queue empty, nothing to process"),
            Self::AttemptsExhausted {
                completed_attempts,
                max_attempts,
            } => write!(
                f,
                "job cannot start: {} of {} attempts already completed",
                completed_attempts, max_attempts
            ),
            Self::InvalidTransition { operation, from } => write!(
                f,
                "invalid transition: cannot {} from {} state",
                operation, from
            ),
            Self::JobNotFound { job_id } => write!(f, "job not found, job id: {}", job_id),
        }
    }
}

impl std::error::Error for JobError {}

#[cfg(test)]
mod tests {
    use super::JobError;
    use crate::job::{JobOperation, JobStateKind};

    #[test]
    fn job_error_implements_error_and_displays_human_readable_messages() {
        fn assert_error<T: std::error::Error>() {}

        assert_error::<JobError>();

        let cases = [
            (JobError::QueueEmpty, "queue empty, nothing to process"),
            (
                JobError::AttemptsExhausted {
                    completed_attempts: 2,
                    max_attempts: 2,
                },
                "job cannot start: 2 of 2 attempts already completed",
            ),
            (
                JobError::InvalidTransition {
                    operation: JobOperation::Cancel,
                    from: JobStateKind::Succeeded,
                },
                "invalid transition: cannot cancel from succeeded state",
            ),
            (
                JobError::JobNotFound { job_id: 42 },
                "job not found, job id: 42",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }
}
