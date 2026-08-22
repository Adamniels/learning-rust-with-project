use crate::error::JobError;
use crate::job::{Job, JobState};

fn retry_delay_seconds(failed_attempt: u32) -> u32 {
    match failed_attempt {
        0 => 0,
        1 => 1,
        2 => 5,
        _ => 15,
    }
}

pub(super) fn simulate_job(
    job: &mut Job,
    succeeds_on_attempt: Option<u32>,
) -> Result<u32, JobError> {
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

#[cfg(test)]
mod tests {
    use super::simulate_job;
    use crate::job::{Job, JobKind, JobState};

    #[test]
    fn simulation_succeeds_on_second_attempt() {
        // Skapa ett nytt jobb med max attempts 3.
        let mut job = Job::new(JobKind::Email, String::from("send-email"), 3);
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
        let mut job = Job::new(JobKind::Email, String::from("send-email"), 3);
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
}
