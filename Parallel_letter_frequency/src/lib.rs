use std::collections::HashMap;

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();

    if input.is_empty() {
        return result;
    }

    let mut vworkers = vec![];

    for i in 0..worker_count {
        let start = i * input.len() / worker_count;
        let end = if i < worker_count - 1 {
            (i + 1) * input.len() / worker_count
        } else {
            input.len()
        };

        // this is possible only if the input has 'static lifetime
        let mut chunk = vec![];
        for j in start..end {
            chunk.push(input[j]);
        }

        // this copy is necessary if we don't force a 'static lifetime to input
        //let schunk = chunk.join("");

        vworkers.push(std::thread::spawn(move || {
            let mut hm = HashMap::new();
            for schunk in chunk {
                for c in schunk.chars() {
                    if c.is_alphabetic() {
                        let cl = c.to_lowercase().next().unwrap();
                        *hm.entry(cl).or_insert(0) += 1;
                    }
                }
            }
            hm
        }));
    }

    for w in vworkers {
        let hm = w.join().unwrap();
        for (k, v) in hm {
            *result.entry(k).or_insert(0) += v;
        }
    }

    result
}
