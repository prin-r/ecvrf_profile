#[macro_use]
extern crate lazy_static;

use rug::Integer;
use std::time::Instant;
mod ecvrf;

fn main() {
    for i in 0..10 {
        let start = Instant::now();
        let x = ecvrf::scalar_multiply(
            &(
                format!(
                    "{}{}",
                    "2504841017466682250484101746668225048410174666822504841017466682", i
                )
                .to_string()
                .parse::<Integer>()
                .unwrap(),
                "1513453546461956113754237990195611375423799019561137542379901956113754237990"
                    .parse::<Integer>()
                    .unwrap(),
            ),
            &"74830380039917927238342598863222899552394587271096264578218486964046080567388"
                .parse::<Integer>()
                .unwrap(),
        )
        .is_some();
        let duration = start.elapsed();
        println!(
            "Time elapsed in expensive_function() is: {:?} {:?}",
            duration, x
        );
    }
}
