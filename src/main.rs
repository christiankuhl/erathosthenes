use std::time::SystemTime;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Primes {
    current: usize,
    limit: Option<usize>,
    count: Option<usize>,
    composites: BinaryHeap<Reverse<[usize; 2]>>,
    yielded: usize
}

impl Primes {
    fn up_to(limit: usize) -> Primes {
        Primes { current: 2, yielded: 0, count: Some(limit), limit: None, composites: BinaryHeap::new() }
    }
    fn all() -> Primes {
        Primes { current: 2, yielded: 0, count: None, limit: None, composites: BinaryHeap::new() }
    }
    fn less_than(limit: usize) -> Primes {
        Primes { current: 2, yielded: 0, count: None, limit: Some(limit), composites: BinaryHeap::new() }
    }
}

impl Iterator for Primes {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if let Some(limit) = self.limit {
            if limit < self.current { return None }
        }
        if let Some(limit) = self.count {
            if limit < self.yielded { return None }
        }
        if self.current == 2 {
            self.current += 1; 
            return Some(2) 
        }
        let mut is_prime = true;
        while let Some(mut head) = self.composites.peek_mut() {
            let Reverse([composite, prime]) = *head;
            if composite > self.current {
                break;
            } else {
                is_prime = false;
                *head = Reverse([composite + 2 * prime, prime]);
            }
        } 
        if is_prime {
            self.composites.push(Reverse([self.current.pow(2), self.current]));
            self.current += 2;
            // println!("{}: {:?}", self.current, self.composites);
            self.yielded += 1;
            return Some(self.current - 2)
        } else { 
            self.current += 2;
            // println!("{}: {:?}", self.current, self.composites);
            return self.next()
         }
    }
}

fn main() {
    let mut now = SystemTime::now();
    for (j, p) in Primes::all().enumerate() {
        if j % 1_000_000 == 999_999 {
            let elapsed = now.elapsed().unwrap().as_millis();
            // println!("{}: {}", j+1, p);
            println!("{} millionth prime is {}, time elapsed: {} ms", (j + 1) / 1_000_000 as usize, p, elapsed);
            now = SystemTime::now();
        }
    }
}
