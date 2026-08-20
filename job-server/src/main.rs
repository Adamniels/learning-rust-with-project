use std::collections::{HashMap, VecDeque};

// Structs
enum JobOperation {
    BeginAttempt,
    CompleteSuccess,
    CompleteFailure,
    Cancel,
}

impl JobOperation {
    fn as_str(&self) -> &'static str {
        match self {
            JobOperation::BeginAttempt => "begin attempt",
            JobOperation::CompleteSuccess => "complete success",
            JobOperation::CompleteFailure => "complete failure",
            JobOperation::Cancel => "cancel",
        }
    }
}

enum JobStateKind {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl JobStateKind {
    fn as_str(&self) -> &'static str {
        match self {
            JobStateKind::Queued => "queued",
            JobStateKind::Running => "running",
            JobStateKind::Succeeded => "succeeded",
            JobStateKind::Failed => "failed",
            JobStateKind::Cancelled => "cancelled",
        }
    }
}

enum JobError {
    QueueEmpty,
    AttemptsExhausted {
        completed_attempts: u32,
        max_attempts: u32,
    },
    InvalidTransition {
        operation: JobOperation,
        from: JobStateKind,
    },
    JobNotFound {
        job_id: u64,
    },
}

enum JobKind {
    Email,
    Cleanup,
}

enum JobState {
    Queued,
    Running { attempt: u32 },
    Succeeded { output: String },
    Failed { error: String },
    Cancelled { reason: String },
}

impl JobState {
    fn kind(&self) -> JobStateKind {
        match self {
            JobState::Queued => JobStateKind::Queued,
            JobState::Running { .. } => JobStateKind::Running,
            JobState::Succeeded { .. } => JobStateKind::Succeeded,
            JobState::Failed { .. } => JobStateKind::Failed,
            JobState::Cancelled { .. } => JobStateKind::Cancelled,
        }
    }
}

struct Job {
    kind: JobKind,
    payload: String,
    max_attempts: u32,
    completed_attempts: u32,
    job_state: JobState,
}

impl Job {
    fn new(kind: JobKind, payload: String, max_attempts: u32) -> Self {
        Job {
            kind,
            payload,
            max_attempts,
            completed_attempts: 0,
            job_state: JobState::Queued,
        }
    }

    fn payload(&self) -> &str {
        &self.payload
    }

    fn completed_attempts(&self) -> u32 {
        self.completed_attempts
    }

    fn kind(&self) -> &JobKind {
        &self.kind
    }

    fn state(&self) -> &JobState {
        &self.job_state
    }

    fn can_attempt(&self) -> bool {
        let valid_state = matches!(&self.job_state, JobState::Queued | JobState::Running { .. });
        self.completed_attempts < self.max_attempts && valid_state
    }

    fn try_begin_attempt(&mut self) -> Result<u32, JobError> {
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
    fn complete_successful_attempt(&mut self, output: String) -> Result<(), JobError> {
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

    fn complete_failed_attempt(&mut self, error: String) -> Result<(), JobError> {
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
    fn cancel(&mut self, reason: String) -> Result<(), JobError> {
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

struct JobServer {
    next_job_id: u64,
    jobs: HashMap<u64, Job>,
    queue: VecDeque<u64>,
}

impl JobServer {
    fn new() -> Self {
        JobServer {
            next_job_id: 1,
            jobs: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    fn submit(&mut self, kind: JobKind, payload: String, max_attempts: u32) -> u64 {
        let job_id = self.next_job_id;
        let new_job = Job::new(kind, payload, max_attempts);
        self.jobs.insert(job_id, new_job);
        self.queue.push_back(job_id);
        self.next_job_id += 1;
        job_id
    }

    fn get(&self, job_id: u64) -> Option<&Job> {
        self.jobs.get(&job_id)
    }

    fn next_queued(&mut self) -> Result<(u64, &mut Job), JobError> {
        let job_id = match self.queue.front().copied() {
            Some(job_id) => job_id,
            None => return Err(JobError::QueueEmpty),
        };
        let job = match self.jobs.get_mut(&job_id) {
            Some(job) => job,
            None => panic!("Couldnt find job in queue with that job id"),
        };

        Ok((job_id, job))
    }

    fn process_next(&mut self, succeeds_on_attempt: Option<u32>) -> Result<(u64, u32), JobError> {
        let (job_id, next_job) = self.next_queued()?;

        let tot_retry_delay = simulate_job(next_job, succeeds_on_attempt)?;

        match self.queue.pop_front() {
            Some(dequeued_job_id) if dequeued_job_id == job_id => {}
            _ => panic!("Queue front changed while processing job"),
        }

        Ok((job_id, tot_retry_delay))
    }
    fn cancel(&mut self, job_id: u64, reason: String) -> Result<(), JobError> {
        let matching_indices: Vec<usize> = self
            .queue
            .iter()
            .enumerate()
            .filter(|entry| *entry.1 == job_id)
            .map(|entry| entry.0)
            .collect();

        let state_kind = match self.jobs.get(&job_id) {
            Some(job) => job.state().kind(),
            None if matching_indices.is_empty() => {
                return Err(JobError::JobNotFound { job_id });
            }
            None => panic!("Queue references missing registry job"),
        };

        let queue_index = match state_kind {
            JobStateKind::Queued => {
                if matching_indices.len() != 1 {
                    panic!("Queued job must appear exactly once in queue");
                }
                matching_indices[0]
            }
            from => {
                if !matching_indices.is_empty() {
                    panic!("Non-queued job must not appear in queue");
                }
                return Err(JobError::InvalidTransition {
                    operation: JobOperation::Cancel,
                    from,
                });
            }
        };

        let job_to_cancel = self
            .jobs
            .get_mut(&job_id)
            .expect("Validated job must remain in registry");

        job_to_cancel.cancel(reason)?;

        match self.queue.remove(queue_index) {
            Some(removed_job_id) if removed_job_id == job_id => {}
            _ => panic!("Queue changed while cancelling job"),
        }

        Ok(())
    }
}

// Functions
fn retry_delay_seconds(failed_attempt: u32) -> u32 {
    match failed_attempt {
        0 => 0,
        1 => 1,
        2 => 5,
        _ => 15,
    }
}

fn simulate_job(job: &mut Job, succeeds_on_attempt: Option<u32>) -> Result<u32, JobError> {
    let mut retry_sec = 0;

    loop {
        let attempt = job.try_begin_attempt()?;

        retry_sec += retry_delay_seconds(job.completed_attempts());

        if succeeds_on_attempt == Some(attempt) {
            job.complete_successful_attempt(String::from("completed"))?;
            return Ok(retry_sec);
        }
        job.complete_failed_attempt(String::from("maximum attempts reached"))?;

        if matches!(job.state(), JobState::Failed { .. }) {
            return Ok(retry_sec);
        }
    }
}

fn report_job_error(error: JobError) {
    match error {
        JobError::AttemptsExhausted {
            completed_attempts,
            max_attempts,
        } => {
            eprintln!(
                "Job cannot start: {completed_attempts} of {max_attempts} attempts already completed"
            );
        }
        JobError::QueueEmpty => {
            eprintln!("Queue empty, nothing to process");
        }
        JobError::InvalidTransition { operation, from } => {
            eprintln!(
                "Invalid transition: cannot {} from {} state",
                operation.as_str(),
                from.as_str()
            );
        }
        JobError::JobNotFound { job_id } => {
            eprintln!("Job not found, job id: {job_id}");
        }
    }
}

// Main
fn main() {
    let mut job_server = JobServer::new();

    job_server.submit(JobKind::Email, String::from("send-email"), 3);
    let cleanup_id = job_server.submit(JobKind::Cleanup, String::from("cleanup"), 1);

    if let Err(error) = job_server.cancel(cleanup_id, String::from("operator request")) {
        report_job_error(error);
        return;
    }

    let (job_id, total_retry_delay_sec) = match job_server.process_next(None) {
        Ok((job_id, tot_retry)) => (job_id, tot_retry),
        Err(error) => {
            report_job_error(error);
            return;
        }
    };

    let job = match job_server.get(job_id) {
        Some(job) => job,
        None => panic!("job id didnt give a job in main"),
    };

    let kind = match job.kind() {
        JobKind::Email => "email",
        JobKind::Cleanup => "cleanup",
    };

    match job.state() {
        JobState::Queued => {
            println!(
                "kind: {kind}, payload: {}, completed attempts: {}, state: queued, total retry delay: {total_retry_delay_sec}",
                job.payload(),
                job.completed_attempts(),
            );
        }
        JobState::Running { attempt } => {
            println!(
                "kind: {kind}, payload: {}, completed attempts: {}, state: running attempt {attempt}, total retry delay: {total_retry_delay_sec}",
                job.payload(),
                job.completed_attempts(),
            );
        }
        JobState::Succeeded { output } => {
            println!(
                "kind: {kind}, payload: {}, completed attempts: {}, state: succeeded, output: {output}, total retry delay: {total_retry_delay_sec}",
                job.payload(),
                job.completed_attempts(),
            );
        }
        JobState::Failed { error } => {
            println!(
                "kind: {kind}, payload: {}, completed attempts: {}, state: failed, error: {error}, total retry delay: {total_retry_delay_sec}",
                job.payload(),
                job.completed_attempts(),
            );
        }
        JobState::Cancelled { reason } => {
            println!(
                "kind: {kind}, payload: {}, completed attempts: {}, state: cancelled, reason: {}, total retry delay: {total_retry_delay_sec}",
                job.payload(),
                job.completed_attempts(),
                reason
            );
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

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
    fn simulation_succeeds_on_second_attempt() {
        // Skapa ett nytt jobb med max attempts 3.
        let mut job: Job = Job::new(JobKind::Email, String::from("send-email"), 3);
        // Kör simulatorn med Some(2).
        let Ok(tot_retry) = simulate_job(&mut job, Some(2)) else {
            panic!("Expected simulation to succeed")
        };

        // Kontrollera:
        // total retry delay är 1,
        assert_eq!(tot_retry, 1);
        // completed_attempts är 2,
        assert_eq!(job.completed_attempts(), 2);
        // state är Succeeded,
        // associated output är "completed".
        // Om state är Queued, Running eller Failed ska testet ge panic!.
        let JobState::Succeeded { output } = job.state() else {
            panic!("Expected job state to be succeeded")
        };
        assert_eq!(output.as_str(), "completed");
    }

    #[test]
    fn simulation_fails_when_no_attempt_succeeds() {
        // Skapa ett nytt jobb med max attempts 3.
        let mut job: Job = Job::new(JobKind::Email, String::from("send-email"), 3);
        // Kör simulatorn med None.
        let Ok(tot_retry) = simulate_job(&mut job, None) else {
            panic!("Expected simulation to complete")
        };

        // Kontrollera:
        // total retry delay är 6,
        assert_eq!(tot_retry, 6);
        // completed_attempts är 3,
        assert_eq!(job.completed_attempts(), 3);
        // state är Failed,
        // associated error är "maximum attempts reached".
        // Om state är Queued, Running eller Succeeded ska testet ge panic!
        if let JobState::Failed { error } = job.state() {
            assert_eq!(error.as_str(), "maximum attempts reached");
        } else {
            panic!("Expected job state to be failed")
        };
    }

    #[test]
    fn new_empty_jobserver_returns_none_for_get_and_queue_empty_for_next_queued() {
        let mut jobserver = JobServer::new();
        assert!(jobserver.get(1).is_none());
        assert!(matches!(jobserver.next_queued(), Err(JobError::QueueEmpty)));
    }

    #[test]
    fn submissions_are_registered_and_queued_in_order() {
        let mut jobserver = JobServer::new();

        let first_id = jobserver.submit(JobKind::Email, String::from("send-email"), 3);

        let second_id = jobserver.submit(JobKind::Cleanup, String::from("cleanup"), 1);

        assert_eq!(first_id, 1);
        assert_eq!(second_id, 2);

        let first_job = jobserver.get(1).expect("job 1 should exist");

        assert!(matches!(first_job.kind(), JobKind::Email));
        assert_eq!(first_job.payload(), "send-email");
        assert_eq!(first_job.max_attempts, 3);
        assert_eq!(first_job.completed_attempts(), 0);
        assert!(matches!(first_job.state(), JobState::Queued));

        let second_job = jobserver.get(2).expect("job 2 should exist");

        assert!(matches!(second_job.kind(), JobKind::Cleanup));
        assert_eq!(second_job.payload(), "cleanup");
        assert_eq!(second_job.max_attempts, 1);
        assert_eq!(second_job.completed_attempts(), 0);
        assert!(matches!(second_job.state(), JobState::Queued));

        assert!(jobserver.get(3).is_none());

        let queued_ids: Vec<u64> = jobserver.queue.iter().copied().collect();

        assert_eq!(queued_ids, vec![1, 2]);

        // iter() lånade bara kön och konsumerade den inte.
        assert_eq!(jobserver.queue.len(), 2);
    }

    #[test]
    fn next_queued_peeks_at_front_job_without_removing_it() {
        let mut jobserver = JobServer::new();

        let first_id = jobserver.submit(JobKind::Email, String::from("send-email"), 3);

        let second_id = jobserver.submit(JobKind::Cleanup, String::from("cleanup"), 1);

        assert_eq!(first_id, 1);
        assert_eq!(second_id, 2);

        {
            let Ok((job_id, job)) = jobserver.next_queued() else {
                panic!("Expected first queued job to exist")
            };

            assert_eq!(job_id, first_id);
            assert_eq!(job.payload(), "send-email");
        }

        let queued_ids: Vec<u64> = jobserver.queue.iter().copied().collect();
        assert_eq!(queued_ids, vec![first_id, second_id]);

        let Ok((peeked_again_id, peeked_again_job)) = jobserver.next_queued() else {
            panic!("Expected front job to remain queued")
        };
        assert_eq!(peeked_again_id, first_id);
        assert_eq!(peeked_again_job.payload(), "send-email");
        assert!(jobserver.get(first_id).is_some());
        assert!(jobserver.get(second_id).is_some());
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
    fn process_next_processes_jobs_in_fifo_order() {
        let mut jobserver = JobServer::new();
        let first_id = jobserver.submit(JobKind::Email, String::from("send-email"), 3);
        let second_id = jobserver.submit(JobKind::Cleanup, String::from("cleanup"), 1);

        let Ok((processed_first_id, first_delay)) = jobserver.process_next(Some(1)) else {
            panic!("Expected first queued job to be processed")
        };

        assert_eq!(processed_first_id, first_id);
        assert_eq!(first_delay, 0);
        {
            let first_job = jobserver
                .get(first_id)
                .expect("first processed job should remain registered");
            assert_eq!(first_job.completed_attempts(), 1);
            assert!(matches!(
                first_job.state(),
                JobState::Succeeded { output } if output == "completed"
            ));
        }

        let Ok((processed_second_id, second_delay)) = jobserver.process_next(None) else {
            panic!("Expected second queued job to be processed")
        };

        assert_eq!(processed_second_id, second_id);
        assert_eq!(second_delay, 0);
        let second_job = jobserver
            .get(second_id)
            .expect("second processed job should remain registered");
        assert_eq!(second_job.completed_attempts(), 1);
        assert!(matches!(
            second_job.state(),
            JobState::Failed { error } if error == "maximum attempts reached"
        ));
    }

    #[test]
    fn process_next_returns_queue_empty_when_no_job_is_queued() {
        let mut jobserver = JobServer::new();

        let result = jobserver.process_next(None);

        assert!(matches!(result, Err(JobError::QueueEmpty)));
        assert!(jobserver.get(1).is_none());
    }

    #[test]
    fn process_next_keeps_front_job_queued_when_simulation_returns_error() {
        let mut jobserver = JobServer::new();
        let first_id = jobserver.submit(JobKind::Email, String::from("send-email"), 0);
        let second_id = jobserver.submit(JobKind::Cleanup, String::from("cleanup"), 1);

        let result = jobserver.process_next(None);

        assert!(matches!(
            result,
            Err(JobError::AttemptsExhausted {
                completed_attempts: 0,
                max_attempts: 0,
            })
        ));
        let queued_ids: Vec<u64> = jobserver.queue.iter().copied().collect();
        assert_eq!(queued_ids, vec![first_id, second_id]);
        let first_job = jobserver
            .get(first_id)
            .expect("failed processing job should remain registered");
        assert!(matches!(first_job.state(), JobState::Queued));
        assert_eq!(first_job.completed_attempts(), 0);
    }

    #[test]
    #[should_panic(expected = "Couldnt find job in queue with that job id")]
    fn next_queued_panics_when_queue_references_missing_registry_job() {
        let mut jobserver = JobServer::new();
        jobserver.queue.push_back(7);

        let _ = jobserver.next_queued();
    }
    #[test]
    fn cancelling_middle_queued_job_preserves_fifo_and_keeps_job_registered() {
        let mut jobserver = JobServer::new();

        let first_id = jobserver.submit(JobKind::Email, String::from("send-email"), 0);
        let second_id = jobserver.submit(JobKind::Cleanup, String::from("cleanup1"), 1);
        let third_id = jobserver.submit(JobKind::Cleanup, String::from("cleanup2"), 1);

        assert_eq!(first_id, 1);
        assert_eq!(second_id, 2);
        assert_eq!(third_id, 3);

        let res = jobserver.cancel(second_id, String::from("operator request"));

        assert!(res.is_ok());

        assert_eq!(jobserver.queue, VecDeque::from([first_id, third_id]));

        let job = jobserver.get(second_id).expect("job 2 should still exist");

        assert_eq!(job.completed_attempts(), 0);
        assert!(matches!(
            job.state(),
            JobState::Cancelled { reason } if reason == "operator request"
        ));

        for job_id in [first_id, third_id] {
            let job = jobserver.get(job_id).expect("Job should exist");

            assert!(
                matches!(job.state(), JobState::Queued),
                "Job {job_id} should be Queued"
            );
        }
    }

    #[test]
    fn cancel_returns_job_not_found_without_mutating_server() {
        let mut jobserver = JobServer::new();
        let existing_id = jobserver.submit(JobKind::Email, String::from("send-email"), 3);

        let result = jobserver.cancel(99, String::from("operator request"));

        assert!(matches!(result, Err(JobError::JobNotFound { job_id: 99 })));
        assert_eq!(jobserver.queue, VecDeque::from([existing_id]));
        let existing_job = jobserver
            .get(existing_id)
            .expect("existing job should remain registered");
        assert!(matches!(existing_job.state(), JobState::Queued));
        assert_eq!(existing_job.completed_attempts(), 0);
    }

    #[test]
    fn cancel_rejects_succeeded_job_without_mutation() {
        let mut jobserver = JobServer::new();
        let job_id = jobserver.submit(JobKind::Email, String::from("send-email"), 1);
        assert!(matches!(
            jobserver.process_next(Some(1)),
            Ok((processed_id, 0)) if processed_id == job_id
        ));

        let result = jobserver.cancel(job_id, String::from("too late"));

        assert!(matches!(
            result,
            Err(JobError::InvalidTransition {
                operation: JobOperation::Cancel,
                from: JobStateKind::Succeeded,
            })
        ));
        assert!(jobserver.queue.is_empty());
        let job = jobserver
            .get(job_id)
            .expect("succeeded job should remain registered");
        assert_eq!(job.completed_attempts(), 1);
        assert!(matches!(
            job.state(),
            JobState::Succeeded { output } if output == "completed"
        ));
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

    #[test]
    #[should_panic(expected = "Queue references missing registry job")]
    fn cancel_panics_when_queue_references_missing_registry_job() {
        let mut jobserver = JobServer::new();
        jobserver.queue.push_back(7);

        let _ = jobserver.cancel(7, String::from("operator request"));
    }

    #[test]
    #[should_panic(expected = "Queued job must appear exactly once in queue")]
    fn cancel_panics_when_queued_job_is_missing_from_queue() {
        let mut jobserver = JobServer::new();
        let job_id = jobserver.submit(JobKind::Email, String::from("send-email"), 3);
        assert_eq!(jobserver.queue.pop_front(), Some(job_id));

        let _ = jobserver.cancel(job_id, String::from("operator request"));
    }

    #[test]
    #[should_panic(expected = "Queued job must appear exactly once in queue")]
    fn cancel_panics_when_queued_job_appears_multiple_times() {
        let mut jobserver = JobServer::new();
        let job_id = jobserver.submit(JobKind::Email, String::from("send-email"), 3);
        jobserver.queue.push_back(job_id);

        let _ = jobserver.cancel(job_id, String::from("operator request"));
    }

    #[test]
    #[should_panic(expected = "Non-queued job must not appear in queue")]
    fn cancel_panics_when_non_queued_job_appears_in_queue() {
        let mut jobserver = JobServer::new();
        let job_id = jobserver.submit(JobKind::Email, String::from("send-email"), 1);
        assert!(matches!(
            jobserver.process_next(Some(1)),
            Ok((processed_id, 0)) if processed_id == job_id
        ));
        jobserver.queue.push_back(job_id);

        let _ = jobserver.cancel(job_id, String::from("too late"));
    }
}
