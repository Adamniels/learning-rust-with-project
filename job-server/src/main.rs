// Structs
struct Job {
    payload: String,
    max_attempts: u32,
    completed_attempts: u32,
}

impl Job {
    fn new(payload: String, max_attempts: u32) -> Self {
        Job {
            payload,
            max_attempts,
            completed_attempts: 0,
        }
    }
    fn payload(&self) -> &str {
        &self.payload
    }
    fn completed_attempts(&self) -> u32 {
        self.completed_attempts
    }
    fn can_attempt(&self) -> bool {
        self.completed_attempts < self.max_attempts
    }

    fn record_attempt(&mut self) {
        self.completed_attempts += 1;
    }
}

struct SimulationResult {
    succeeded: bool,
    total_retry_delay_sec: u32,
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

fn simulate_job(job: &mut Job, succeeds_on_attempt: u32) -> SimulationResult {
    let mut retry_sec = 0;

    while job.can_attempt() {
        // Wait according to the previous failed attempt meaning the completed attempts so far.
        retry_sec += retry_delay_seconds(job.completed_attempts());

        // Do the attempt
        job.record_attempt();

        // See if we succeed on this attempt
        if job.completed_attempts() == succeeds_on_attempt {
            return SimulationResult {
                succeeded: true,
                total_retry_delay_sec: retry_sec,
            };
        }
    }

    SimulationResult {
        succeeded: false,
        total_retry_delay_sec: retry_sec,
    }
}

// Main
fn main() {
    println!("Hello, world!");
    let mut job = Job::new(String::from("send-email"), 3);
    let result = simulate_job(&mut job, 4);

    println!(
        "{}, {}, {}, {}",
        job.payload(),
        job.completed_attempts(),
        result.succeeded,
        result.total_retry_delay_sec
    );
}

// Tests
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
            let mut job = Job::new(String::from("test-payload"), max_attempts);

            let result = simulate_job(&mut job, succeeds_on_attempt);

            let actual = (
                job.completed_attempts(),
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
        let mut job = Job::new(String::from("test-payload"), 3);

        simulate_job(&mut job, 1);

        assert_eq!(job.max_attempts, 3);
        assert_eq!(job.payload(), "test-payload");
    }
}
