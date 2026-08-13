fn main() {
    println!("Hello, world!");
    let job = simulate_job(3, 4);

    println!("{}, {}, {}", job.0, job.1, job.2);

    let job = simulate_job2(3, 4);

    println!("{}, {}, {}", job.0, job.1, job.2);
}

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

// just a simulation of the job for unit 1
fn simulate_job(max_attempts: u32, succeeds_on_attempt: u32) -> (u32, bool, u32) {
    let mut completed_attempts = 0;
    let mut retry_sec = 0;
    loop {
        if !can_attempt(completed_attempts, max_attempts) {
            return (completed_attempts, false, retry_sec);
        }

        retry_sec += retry_delay_seconds(completed_attempts);

        // attempt a job
        completed_attempts += 1;

        // did it succeed?
        if completed_attempts == succeeds_on_attempt {
            return (completed_attempts, true, retry_sec);
        }
    }
}

fn simulate_job2(max_attempts: u32, succeeds_on_attempt: u32) -> (u32, bool, u32) {
    let mut completed_attempts = 0;
    let mut retry_sec = 0;

    while can_attempt(completed_attempts, max_attempts) {
        // Wait according to the previous failed attempt meaning the completed attempts so far.
        retry_sec += retry_delay_seconds(completed_attempts);

        // Do the attempt
        completed_attempts += 1;

        // See if we succeed on this attempt
        if completed_attempts == succeeds_on_attempt {
            return (completed_attempts, true, retry_sec);
        }
    }

    (completed_attempts, false, retry_sec)
}

// Test from unit 1
#[cfg(test)]
mod tests_unit1 {
    use super::*;

    #[test]
    fn simulate_job_handles_all_cases() {
        let test_cases = [
            ((0, 1), (0, false, 0)),
            ((1, 1), (1, true, 0)),
            ((3, 1), (1, true, 0)),
            ((3, 2), (2, true, 1)),
            ((3, 3), (3, true, 6)),
            ((3, 4), (3, false, 6)),
            ((3, 0), (3, false, 6)),
        ];

        for ((max_attempts, succeeds_on_attempt), expected) in test_cases {
            let actual = simulate_job(max_attempts, succeeds_on_attempt);

            assert_eq!(
                actual, expected,
                "failed for simulate_job({max_attempts}, {succeeds_on_attempt})"
            );
        }
    }

    #[test]
    fn simulate_job2_handles_all_cases() {
        let test_cases = [
            ((0, 1), (0, false, 0)),
            ((1, 1), (1, true, 0)),
            ((3, 1), (1, true, 0)),
            ((3, 2), (2, true, 1)),
            ((3, 3), (3, true, 6)),
            ((3, 4), (3, false, 6)),
            ((3, 0), (3, false, 6)),
        ];

        for ((max_attempts, succeeds_on_attempt), expected) in test_cases {
            let actual = simulate_job2(max_attempts, succeeds_on_attempt);

            assert_eq!(
                actual, expected,
                "failed for simulate_job({max_attempts}, {succeeds_on_attempt})"
            );
        }
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
