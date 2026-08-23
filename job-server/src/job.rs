use crate::error::JobError;
use std::fmt;

/// The kind of work represented by a [`Job`].
pub enum JobKind {
    /// Work that sends an email.
    Email,
    /// Work that performs cleanup.
    Cleanup,
}

/// The current state of a [`Job`], including data specific to that state.
pub enum JobState {
    /// The job is waiting to start an attempt.
    Queued,
    /// The job is executing the given one-based attempt number.
    Running {
        /// The one-based number of the running attempt.
        attempt: u32,
    },
    /// The job completed successfully.
    Succeeded {
        /// The output produced by the job.
        output: String,
    },
    /// The job used all available attempts without succeeding.
    Failed {
        /// The failure reported by the final attempt.
        error: String,
    },
    /// The job was cancelled before processing.
    Cancelled {
        /// The reason supplied when the job was cancelled.
        reason: String,
    },
}

impl JobState {
    pub(crate) fn kind(&self) -> JobStateKind {
        match self {
            JobState::Queued => JobStateKind::Queued,
            JobState::Running { .. } => JobStateKind::Running,
            JobState::Succeeded { .. } => JobStateKind::Succeeded,
            JobState::Failed { .. } => JobStateKind::Failed,
            JobState::Cancelled { .. } => JobStateKind::Cancelled,
        }
    }
}

/// An operation that can participate in a job state transition.
#[derive(Debug)]
pub enum JobOperation {
    /// Begin the next processing attempt.
    BeginAttempt,
    /// Complete the running attempt successfully.
    CompleteSuccess,
    /// Complete the running attempt with a failure.
    CompleteFailure,
    /// Cancel a queued job.
    Cancel,
}

impl fmt::Display for JobOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobOperation::BeginAttempt => write!(f, "begin attempt"),
            JobOperation::CompleteSuccess => write!(f, "complete success"),
            JobOperation::CompleteFailure => write!(f, "complete failure"),
            JobOperation::Cancel => write!(f, "cancel"),
        }
    }
}

impl JobOperation {}

/// A payload-free classification of [`JobState`].
///
/// This is useful when a caller needs to identify a state without owning or
/// exposing the state-specific output, error, or cancellation reason.
#[derive(Debug)]
pub enum JobStateKind {
    /// The job is queued.
    Queued,
    /// The job is running.
    Running,
    /// The job succeeded.
    Succeeded,
    /// The job failed.
    Failed,
    /// The job was cancelled.
    Cancelled,
}

impl fmt::Display for JobStateKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobStateKind::Queued => write!(f, "queued"),
            JobStateKind::Running => write!(f, "running"),
            JobStateKind::Succeeded => write!(f, "succeeded"),
            JobStateKind::Failed => write!(f, "failed"),
            JobStateKind::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl JobStateKind {}

/// A registered unit of work and its processing state.
///
/// The representation is private so state changes can only occur through the
/// library's validated transitions.
pub struct Job {
    kind: JobKind,
    payload: String,
    max_attempts: u32,
    completed_attempts: u32,
    job_state: JobState,
}

impl Job {
    pub(crate) fn new(kind: JobKind, payload: String, max_attempts: u32) -> Self {
        Job {
            kind,
            payload,
            max_attempts,
            completed_attempts: 0,
            job_state: JobState::Queued,
        }
    }

    /// Returns the job payload.
    pub fn payload(&self) -> &str {
        &self.payload
    }

    /// Returns the number of attempts that have finished.
    pub fn completed_attempts(&self) -> u32 {
        self.completed_attempts
    }

    /// Returns the job's work kind.
    pub fn kind(&self) -> &JobKind {
        &self.kind
    }

    /// Returns the job's current state.
    pub fn state(&self) -> &JobState {
        &self.job_state
    }

    fn can_attempt(&self) -> bool {
        let valid_state = matches!(&self.job_state, JobState::Queued | JobState::Running { .. });
        self.completed_attempts < self.max_attempts && valid_state
    }

    pub(crate) fn try_begin_attempt(&mut self) -> Result<u32, JobError> {
        if matches!(&self.job_state, JobState::Queued) && self.can_attempt() {
            let attempt = self.completed_attempts + 1;
            self.job_state = JobState::Running { attempt };
            Ok(attempt)
        } else {
            if matches!(&self.job_state, JobState::Queued) {
                return Err(JobError::AttemptsExhausted {
                    completed_attempts: self.completed_attempts,
                    max_attempts: self.max_attempts,
                });
            }
            Err(JobError::InvalidTransition {
                operation: JobOperation::BeginAttempt,
                from: self.job_state.kind(),
            })
        }
    }

    pub(crate) fn complete_successful_attempt(&mut self, output: String) -> Result<(), JobError> {
        if let JobState::Running { attempt } = self.job_state {
            self.completed_attempts = attempt;
            self.job_state = JobState::Succeeded { output };
            Ok(())
        } else {
            Err(JobError::InvalidTransition {
                operation: JobOperation::CompleteSuccess,
                from: self.job_state.kind(),
            })
        }
    }

    pub(crate) fn complete_failed_attempt(&mut self, error: String) -> Result<(), JobError> {
        if let JobState::Running { attempt } = self.job_state {
            self.completed_attempts = attempt;
            if self.can_attempt() {
                self.job_state = JobState::Queued;
            } else {
                self.job_state = JobState::Failed { error };
            }
            Ok(())
        } else {
            Err(JobError::InvalidTransition {
                operation: JobOperation::CompleteFailure,
                from: self.job_state.kind(),
            })
        }
    }

    pub(crate) fn cancel(&mut self, reason: String) -> Result<(), JobError> {
        if let JobState::Queued = self.job_state {
            self.job_state = JobState::Cancelled { reason };
            Ok(())
        } else {
            Err(JobError::InvalidTransition {
                operation: JobOperation::Cancel,
                from: self.job_state.kind(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Job, JobKind, JobOperation, JobState, JobStateKind};
    use crate::error::JobError;

    #[test]
    fn job_operation_display_uses_human_readable_names() {
        let cases = [
            (JobOperation::BeginAttempt, "begin attempt"),
            (JobOperation::CompleteSuccess, "complete success"),
            (JobOperation::CompleteFailure, "complete failure"),
            (JobOperation::Cancel, "cancel"),
        ];

        for (operation, expected) in cases {
            assert_eq!(operation.to_string(), expected);
        }
    }

    #[test]
    fn job_state_kind_display_uses_human_readable_names() {
        let cases = [
            (JobStateKind::Queued, "queued"),
            (JobStateKind::Running, "running"),
            (JobStateKind::Succeeded, "succeeded"),
            (JobStateKind::Failed, "failed"),
            (JobStateKind::Cancelled, "cancelled"),
        ];

        for (state_kind, expected) in cases {
            assert_eq!(state_kind.to_string(), expected);
        }
    }

    #[test]
    fn job_transitions_from_queued_to_running_and_back() {
        // Skapa ett jobb med max attempts 3.
        let mut job: Job = Job::new(JobKind::Email, String::from("send-email"), 3);
        // Kontrollera initialt:
        // state är Queued,
        assert!(matches!(&job.job_state, JobState::Queued));
        // completed_attempts är 0.
        assert_eq!(job.completed_attempts(), 0);
        // Anropa try_begin_attempt.
        // Kontrollera:
        // returvärdet är Ok(1),
        assert!(matches!(job.try_begin_attempt(), Ok(1)));
        // state är Running med attempt number 1,
        assert!(matches!(&job.job_state, JobState::Running { attempt: 1 }));
        // completed_attempts är fortfarande 0, eftersom attemptet bara har börjat.
        assert_eq!(job.completed_attempts(), 0);
        // Anropa sedan complete_failed_attempt med ett ägt error message.
        assert!(matches!(
            job.complete_failed_attempt(String::from("failed")),
            Ok(())
        ));
        // Kontrollera:
        // state har gått tillbaka till Queued, eftersom två attempts återstår,
        assert!(matches!(&job.job_state, JobState::Queued));
        // completed_attempts är nu 1.
        assert_eq!(job.completed_attempts(), 1);
    }

    #[test]
    fn queued_job_with_zero_max_attempts_returns_attempts_exhausted_without_mutation() {
        let mut job = Job::new(JobKind::Email, String::from("send-email"), 0);

        let res = job.try_begin_attempt();

        assert!(matches!(
            res,
            Err(JobError::AttemptsExhausted {
                completed_attempts: 0,
                max_attempts: 0
            })
        ));

        assert!(matches!(job.state(), JobState::Queued));

        assert_eq!(job.completed_attempts(), 0);
    }

    #[test]
    fn running_job_cannot_begin_another_attempt_and_remains_unchanged() {
        let mut job = Job::new(JobKind::Email, String::from("send-email"), 3);
        assert!(matches!(job.try_begin_attempt(), Ok(1)));

        let result = job.try_begin_attempt();

        assert!(matches!(
            result,
            Err(JobError::InvalidTransition {
                operation: JobOperation::BeginAttempt,
                from: JobStateKind::Running,
            })
        ));
        assert!(matches!(job.state(), JobState::Running { attempt: 1 }));
        assert_eq!(job.completed_attempts(), 0);
    }

    #[test]
    fn queued_job_cannot_complete_successfully_and_remains_unchanged() {
        let mut job = Job::new(JobKind::Email, String::from("send-email"), 3);

        let result = job.complete_successful_attempt(String::from("completed"));

        assert!(matches!(
            result,
            Err(JobError::InvalidTransition {
                operation: JobOperation::CompleteSuccess,
                from: JobStateKind::Queued,
            })
        ));
        assert!(matches!(job.state(), JobState::Queued));
        assert_eq!(job.completed_attempts(), 0);
    }

    #[test]
    fn queued_job_cannot_complete_with_failure_and_remains_unchanged() {
        let mut job = Job::new(JobKind::Email, String::from("send-email"), 3);

        let result = job.complete_failed_attempt(String::from("failed"));

        assert!(matches!(
            result,
            Err(JobError::InvalidTransition {
                operation: JobOperation::CompleteFailure,
                from: JobStateKind::Queued,
            })
        ));
        assert!(matches!(job.state(), JobState::Queued));
        assert_eq!(job.completed_attempts(), 0);
    }

    #[test]
    fn cancelled_job_rejects_attempt_transitions_without_mutation() {
        let mut job = Job::new(JobKind::Email, String::from("send-email"), 3);
        assert!(matches!(
            job.cancel(String::from("operator request")),
            Ok(())
        ));

        assert!(matches!(
            job.try_begin_attempt(),
            Err(JobError::InvalidTransition {
                operation: JobOperation::BeginAttempt,
                from: JobStateKind::Cancelled,
            })
        ));
        assert!(matches!(
            job.complete_successful_attempt(String::from("completed")),
            Err(JobError::InvalidTransition {
                operation: JobOperation::CompleteSuccess,
                from: JobStateKind::Cancelled,
            })
        ));
        assert!(matches!(
            job.complete_failed_attempt(String::from("failed")),
            Err(JobError::InvalidTransition {
                operation: JobOperation::CompleteFailure,
                from: JobStateKind::Cancelled,
            })
        ));
        assert_eq!(job.completed_attempts(), 0);
        assert!(matches!(
            job.state(),
            JobState::Cancelled { reason } if reason == "operator request"
        ));
    }
}
