[![Build Status](https://travis-ci.com/immortalinfidel/sma-rs.svg?branch=master)](https://travis-ci.com/immortalinfidel/sma-rs)

#SMA(Simple Moving Average)
```
use ta_common::traits::Indicator;
use sma_rs::SMA;
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
```