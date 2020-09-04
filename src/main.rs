use std::time::SystemTime;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const WHEEL_SIZE: usize = 48;
const WHEEL: [usize; WHEEL_SIZE] = [2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4, 2, 4, 8, 6, 4, 6, 2, 4, 6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10]; 
const SMALL_PRIMES: [usize; 5] = [2, 3, 5, 7, 11];

struct Current {
    value: usize,
    wheel_index: usize,
}

struct Primes {
    current: Current,
    limit: Option<usize>,
    count: Option<usize>,
    composites: BinaryHeap<Reverse<[usize; 3]>>,
    yielded: usize,
    small_primes: Box<dyn Iterator<Item=&'static usize>>,
}

impl Primes {
    fn all() -> Primes {
        let small_primes = SMALL_PRIMES.iter();
        Primes { 
            current: Current { value: 2, wheel_index: 0 }, 
            yielded: 0, 
            count: None, 
            limit: None, 
            composites: BinaryHeap::new(), 
            small_primes: Box::new(small_primes)
        }
    }
    fn up_to(limit: usize) -> Primes {
        let mut p = Primes::all();
        p.count = Some(limit);
        return p
    }
    fn less_than(limit: usize) -> Primes {
        let mut p = Primes::all();
        p.limit = Some(limit);
        return p
    }
}

impl Iterator for Primes {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if let Some(limit) = self.limit {
            if limit < self.current.value { return None }
        }
        if let Some(limit) = self.count {
            if limit <= self.yielded { return None }
        }
        let mut is_prime = true;
        if let Some(prime) = self.small_primes.next() {
            self.current.value = *prime;
        } else {
            self.current.value += WHEEL[self.current.wheel_index];
            self.current.wheel_index = ( self.current.wheel_index + 1 ) % WHEEL_SIZE;
        }
        while let Some(mut head) = self.composites.peek_mut() {
            let Reverse([composite, prime, old_wheel]) = *head;
            if composite > self.current.value {
                break;
            } else {
                is_prime = false;
                *head = Reverse([composite + WHEEL[old_wheel % 48] * prime, prime, (old_wheel + 1) % WHEEL_SIZE]);
            }
        } 
        if is_prime {
            if self.current.value > 7 {
                self.composites.push(Reverse([self.current.value.pow(2), self.current.value, self.current.wheel_index]));
            }
            self.yielded += 1;
            return Some(self.current.value)
        } else { 
            return self.next()
         }
    }
}

fn main() {
    let mut now = SystemTime::now();
    for (j, p) in Primes::all().enumerate() {
        if j % 1_000_000 == 999_999 {
            let elapsed = now.elapsed().unwrap().as_millis();
            println!("{} millionth prime is {}, time elapsed: {} ms", (j + 1) / 1_000_000 as usize, p, elapsed);
            now = SystemTime::now();
        }
    }
}
