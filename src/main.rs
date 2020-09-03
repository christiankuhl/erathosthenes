use std::time::SystemTime;

struct Primes {
    current: usize,
    limit: Option<usize>,
    yielded: Box<Vec<usize>>
}

impl Primes {
    fn new(limit: Option<usize>) -> Primes {
        Primes { current: 2, yielded: Box::new(Vec::new()), limit: limit }
    }
}

impl Iterator for Primes {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if let Some(limit) = self.limit {
            if limit < self.yielded.len() { return None }
        }
        let result = self.current;
        let mut composite = false;
        for k in &*self.yielded {
            if (*k as f64) > (self.current as f64).sqrt() { break; }
            if self.current % k == 0 {
                composite = true;
                break;
            }
        }
        self.current += 1;
        if !composite {
            self.yielded.push(result);
            Some(result)
        } else {
            self.next()
        }
    }
}

fn main() {
    let mut now = SystemTime::now();
    for (j, p) in Primes::new(None).enumerate() {
        if j % 1_000_000 == 999_999 {
            let elapsed = now.elapsed().unwrap().as_millis();
            println!("{} millionth prime is {}, time elapsed: {} ms", (j + 1) / 1_000_000 as usize, p, elapsed);
            now = SystemTime::now();
        }
    }
}
