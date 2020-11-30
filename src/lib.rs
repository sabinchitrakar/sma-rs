#![feature(external_doc)]

use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;

#[doc(include = "../README.md")]
pub struct SMA {
    accumulator: f64,
    period: u32,
    history: FixedQueue<f64>,
}

impl SMA {
    pub fn new(period: u32) -> SMA {
        Self {
            accumulator: 0_f64,
            period,
            history: FixedQueue::new(period),
        }
    }

    pub fn get_total(&self) -> f64 {
        return self.accumulator;
    }
}

impl Indicator<f64, Option<f64>> for SMA {
    fn next(&mut self, input: f64) -> Option<f64> {
        return if self.history.is_full() {
            let out = self.history.at(0).unwrap();
            self.history.add(input);
            self.accumulator = self.accumulator - out + input;
            Some(self.accumulator / self.period as f64)
        } else {
            self.history.add(input);
            self.accumulator = self.accumulator + input;
            None
        }


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
        let mut sma = SMA::new(5);
        assert_eq!(sma.next(81.59), None);
        assert_eq!(sma.next(81.06), None);
        assert_eq!(sma.next(82.87), None);
        assert_eq!(sma.next(83.00), None);
        assert_eq!(sma.next(83.61), None);
        assert_eq!(sma.next(83.15), Some(82.73799999999999));
        assert_eq!(sma.next(82.84), Some(83.09399999999998));
        assert_eq!(sma.next(83.99), Some(83.31799999999998));
        assert_eq!(sma.next(84.55), Some(83.62799999999999));
        assert_eq!(sma.next(84.36), Some(83.77799999999999));
        assert_eq!(sma.next(85.53), Some(84.25399999999998));
        assert_eq!(sma.next(86.54), Some(84.99399999999997));
        assert_eq!(sma.next(86.89), Some(85.57399999999997));
        assert_eq!(sma.next(87.77), Some(86.21799999999996));
        assert_eq!(sma.next(87.29), Some(86.80399999999996));
    }
}
