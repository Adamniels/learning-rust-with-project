// Structs
enum JobKind {
    Email,
    Cleanup,
}

enum JobState {
    Queued,
    Running { attempt: u32 },
    Succeeded { output: String },
    Failed { error: String },
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

    fn try_begin_attempt(&mut self) -> Option<u32> {
        if matches!(&self.job_state, JobState::Queued) && self.can_attempt() {
            let attempt = self.completed_attempts + 1;
            self.job_state = JobState::Running { attempt };
            Some(attempt)
        } else {
            None
        }
    }
    fn complete_successful_attempt(&mut self, output: String) {
        let attempt = match self.job_state {
            JobState::Running { attempt } => attempt,
            _ => panic!("Job must be running"),
        };

        self.completed_attempts = attempt;
        self.job_state = JobState::Succeeded { output };
    }

    fn complete_failed_attempt(&mut self, error: String) {
        let attempt = match self.job_state {
            JobState::Running { attempt } => attempt,
            _ => panic!("Job must be running"),
        };

        self.completed_attempts = attempt;
        if self.can_attempt() {
            self.job_state = JobState::Queued;
        } else {
            self.job_state = JobState::Failed { error };
        }
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

fn simulate_job(job: &mut Job, succeeds_on_attempt: Option<u32>) -> u32 {
    let mut retry_sec = 0;

    loop {
        let attempt = match job.try_begin_attempt() {
            Some(attempt) => attempt,
            None => return retry_sec,
        };

        retry_sec += retry_delay_seconds(job.completed_attempts());

        if succeeds_on_attempt == Some(attempt) {
            job.complete_successful_attempt(String::from("completed"));
            return retry_sec;
        }
        job.complete_failed_attempt(String::from("maximum attempts reached"));
    }
}

// Main
fn main() {
    let mut job = Job::new(JobKind::Email, String::from("send-email"), 3);
    let total_retry_delay_sec = simulate_job(&mut job, None);

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
        // returvärdet är Some(1),
        assert!(matches!(job.try_begin_attempt(), Some(1)));
        // state är Running med attempt number 1,
        assert!(matches!(&job.job_state, JobState::Running { attempt: 1 }));
        // completed_attempts är fortfarande 0, eftersom attemptet bara har börjat.
        assert_eq!(job.completed_attempts(), 0);
        // Anropa sedan complete_failed_attempt med ett ägt error message.
        job.complete_failed_attempt(String::from("failed"));
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
        let tot_retry = simulate_job(&mut job, Some(2));

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
        let tot_retry = simulate_job(&mut job, None);

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
}
