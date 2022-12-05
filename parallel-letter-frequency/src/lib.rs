use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    /* unimplemented!(
            "Count the frequency of letters in the given input '{:?}'. Ensure that you are using {} to process the input.",
            input,
            match worker_count {
                1 => "1 worker".to_string(),
                _ => format!("{} workers", worker_count),
            }
        );
        SEE:
        Multithreading in Rust | Nicky blogs
    https://nickymeuleman.netlify.app/blog/multithreading-rust#final-code
        */

    let mut result = HashMap::new();
    let jobs = input.iter().map(|s| s.to_string());
    for job in map_jobs(worker_count, frequency_job, jobs.collect()) {
        result.merge(job);
    }
    result
}

fn frequency_job(line: String) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for c in line.to_lowercase().chars() {
        if c.is_alphabetic() {
            *result.entry(c).or_insert(0) += 1
        }
    }
    result
}

trait Merge<T> {
    fn merge(&mut self, other: T);
}

impl<K, V> Merge<HashMap<K, V>> for HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
    V: std::ops::AddAssign,
{
    fn merge(&mut self, other: HashMap<K, V>) {
        for (k, v) in other {
            match self.entry(k) {
                Entry::Occupied(o) => *o.into_mut() += v,
                Entry::Vacant(o) => {
                    o.insert(v);
                }
            };
        }
    }
}

fn map_jobs<S, T>(worker_count: usize, map_job: fn(S) -> T, jobs: Vec<S>) -> mpsc::Receiver<T>
where
    S: Debug + Send + 'static,
    T: Debug + Send + 'static,
{
    let (result_tx, result_rx) = mpsc::channel::<T>();
    let jobs = Arc::new(Mutex::new(jobs));
    for _ in 0..worker_count {
        let thread_results = result_tx.clone();
        let queue_lock = jobs.clone();
        thread::spawn(move || {
            while let Some(item) = queue_lock.lock().unwrap().pop() {
                thread_results.send(map_job(item)).unwrap();
            }
        });
    }
    result_rx
}
