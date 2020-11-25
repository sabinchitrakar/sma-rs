[![Build Status](https://travis-ci.com/immortalinfidel/sma-rs.svg?branch=master)](https://travis-ci.com/immortalinfidel/sma-rs)

#SMA(Simple Moving Average)
```
use ta_common::traits::Indicator;
use sma_rs::SMA;
let mut sma = SMA::new(4);
assert_eq!(sma.next(4.0), Some(4.0));
assert_eq!(sma.next(5.0), Some(4.5));
assert_eq!(sma.next(6.0), Some(5.0));
assert_eq!(sma.next(6.0), Some(5.25));
assert_eq!(sma.next(6.0), Some(5.75));
assert_eq!(sma.next(6.0), Some(6.0));
assert_eq!(sma.next(2.0), Some(5.0));
```