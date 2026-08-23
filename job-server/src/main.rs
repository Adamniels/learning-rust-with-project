use job_server::{JobError, JobKind, JobServer, JobState};

fn report_job_error(error: JobError) {
    eprintln!("{}", error);
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
