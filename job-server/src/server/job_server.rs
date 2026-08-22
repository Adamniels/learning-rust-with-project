use super::simulation::simulate_job;
use crate::error::JobError;
use crate::job::{Job, JobKind, JobOperation, JobStateKind};

use std::collections::{HashMap, VecDeque};

/// An in-memory registry and FIFO queue for jobs.
///
/// `JobServer` owns every submitted [`Job`] and exposes complete use cases so
/// callers cannot mutate job state independently of queue state.
pub struct JobServer {
    next_job_id: u64,
    jobs: HashMap<u64, Job>,
    queue: VecDeque<u64>,
}

impl JobServer {
    /// Creates an empty job server whose first submitted job receives ID `1`.
    pub fn new() -> Self {
        JobServer {
            next_job_id: 1,
            jobs: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    /// Registers and queues a new job, returning its one-based identifier.
    pub fn submit(&mut self, kind: JobKind, payload: String, max_attempts: u32) -> u64 {
        let job_id = self.next_job_id;
        let new_job = Job::new(kind, payload, max_attempts);
        self.jobs.insert(job_id, new_job);
        self.queue.push_back(job_id);
        self.next_job_id += 1;
        job_id
    }

    /// Returns the registered job with `job_id`, if it exists.
    pub fn get(&self, job_id: u64) -> Option<&Job> {
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

    /// Processes the next queued job with the current synchronous simulation.
    ///
    /// `succeeds_on_attempt` is a temporary simulation input. `Some(n)` makes
    /// the job succeed on one-based attempt `n`; `None` makes every attempt
    /// fail. The returned tuple contains the processed job ID and total retry
    /// delay in seconds.
    ///
    /// # Errors
    ///
    /// Returns [`JobError::QueueEmpty`] when no job is queued, or propagates a
    /// failed job transition without removing the job from the queue.
    pub fn process_next(
        &mut self,
        succeeds_on_attempt: Option<u32>,
    ) -> Result<(u64, u32), JobError> {
        let (job_id, next_job) = self.next_queued()?;

        let tot_retry_delay = simulate_job(next_job, succeeds_on_attempt)?;

        match self.queue.pop_front() {
            Some(dequeued_job_id) if dequeued_job_id == job_id => {}
            _ => panic!("Queue front changed while processing job"),
        }

        Ok((job_id, tot_retry_delay))
    }

    /// Cancels a queued job and removes it from the processing queue.
    ///
    /// The cancelled job remains registered and can still be observed with
    /// [`Self::get`].
    ///
    /// # Errors
    ///
    /// Returns [`JobError::JobNotFound`] for an unknown ID and
    /// [`JobError::InvalidTransition`] when the job is not queued.
    pub fn cancel(&mut self, job_id: u64, reason: String) -> Result<(), JobError> {
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

#[cfg(test)]
mod tests {
    use super::JobServer;
    use crate::error::JobError;
    use crate::job::{JobKind, JobOperation, JobState, JobStateKind};
    use std::collections::VecDeque;

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
        assert_eq!(first_job.completed_attempts(), 0);
        assert!(matches!(first_job.state(), JobState::Queued));

        let second_job = jobserver.get(2).expect("job 2 should exist");

        assert!(matches!(second_job.kind(), JobKind::Cleanup));
        assert_eq!(second_job.payload(), "cleanup");
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
