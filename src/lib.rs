#![feature(external_doc)]
use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;

#[doc(include = "../README.md")]
pub struct SMA {

    accumulator: f64,
    history: FixedQueue<f64>,
}

impl SMA {
    pub fn new(period: u32) -> SMA {
        Self {
            accumulator: 0_f64,
            history: FixedQueue::new(period),
        }
    }
}

impl Indicator<f64, Option<f64>> for SMA {
    fn next(&mut self, input: f64) -> Option<f64> {
        if self.history.is_full() {
            let out = self.history.at(0).unwrap();
            self.history.add(input);
            self.accumulator = self.accumulator - out + input;
        } else {
            self.history.add(input);
            self.accumulator = self.accumulator + input;
        }
        let n = self.history.size() as f64;
        return Some(self.accumulator / n);
    }

    fn reset(&mut self) {
        self.history.clear();
        self.accumulator = 0_f64;
    }
}

#[cfg(test)]
mod tests {
    use crate::SMA;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut sma = SMA::new(4);
        assert_eq!(sma.next(4.0), Some(4.0));
        assert_eq!(sma.next(5.0), Some(4.5));
        assert_eq!(sma.next(6.0), Some(5.0));
        assert_eq!(sma.next(6.0), Some(5.25));
        assert_eq!(sma.next(6.0), Some(5.75));
        assert_eq!(sma.next(6.0), Some(6.0));
        assert_eq!(sma.next(2.0), Some(5.0));
    }
}
