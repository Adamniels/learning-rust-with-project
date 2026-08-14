// Unit 1
fn can_attempt(completed_attempts: u32, max_attempts: u32) -> bool {
    completed_attempts < max_attempts
}
fn retry_delay_seconds(failed_attempt: u32) -> u32 {
    match failed_attempt {
        0 => 0,
        1 => 1,
        2 => 5,
        _ => 15,
    }
}

fn simulate_job(job: Job, succeeds_on_attempt: u32) -> SimulationResult {
    let mut completed_attempts = 0;
    let mut retry_sec = 0;

    while can_attempt(completed_attempts, job.max_attempts) {
        // Wait according to the previous failed attempt meaning the completed attempts so far.
        retry_sec += retry_delay_seconds(completed_attempts);

        // Do the attempt
        completed_attempts += 1;

        // See if we succeed on this attempt
        if completed_attempts == succeeds_on_attempt {
            return SimulationResult {
                job,
                completed_attempts,
                succeeded: true,
                total_retry_delay_sec: retry_sec,
            };
        }
    }

    SimulationResult {
        job,
        completed_attempts,
        succeeded: false,
        total_retry_delay_sec: retry_sec,
    }
}

// Unit 2
struct Job {
    payload: String,
    max_attempts: u32,
}

struct SimulationResult {
    job: Job,
    completed_attempts: u32,
    succeeded: bool,
    total_retry_delay_sec: u32,
}

fn main() {
    println!("Hello, world!");
    let job = Job {
        payload: String::from("send-email"),
        max_attempts: 3,
    };
    let result = simulate_job(job, 4);

    println!(
        "{}, {}, {}, {}",
        result.job.payload,
        result.completed_attempts,
        result.succeeded,
        result.total_retry_delay_sec
    );
}

// Test from unit 1
#[cfg(test)]
mod tests_unit1and2 {
    use super::*;

    #[test]
    fn simulate_job_handles_representative_outcomes() {
        let test_cases = [
            ((0, 1), (0, false, 0)),
            ((3, 1), (1, true, 0)),
            ((3, 3), (3, true, 6)),
            ((3, 4), (3, false, 6)),
        ];

        for ((max_attempts, succeeds_on_attempt), expected) in test_cases {
            let job = Job {
                payload: String::from("test-payload"),
                max_attempts,
            };

            let result = simulate_job(job, succeeds_on_attempt);

            let actual = (
                result.completed_attempts,
                result.succeeded,
                result.total_retry_delay_sec,
            );

            assert_eq!(
                actual, expected,
                "failed with max_attempts={max_attempts}, \
                 succeeds_on_attempt={succeeds_on_attempt}"
            );
        }
    }

    #[test]
    fn simulate_job_preserves_job_data() {
        let job = Job {
            payload: String::from("test-payload"),
            max_attempts: 3,
        };

        let result = simulate_job(job, 1);

        assert_eq!(result.job.max_attempts, 3);
        assert_eq!(result.job.payload, "test-payload");
    }
}
// test from unit 0
#[cfg(test)]
mod tests {
    #[test]
    fn test_harness_runs() {
        assert!(true);
    }
}
