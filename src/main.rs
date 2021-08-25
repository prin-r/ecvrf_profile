#[macro_use]
extern crate lazy_static;

use hex::decode;
use rug::Integer;
use std::time::Instant;
mod ecvrf;

fn main() {
    for i in 0..1 {
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
        // assert_eq!(
        //     ecvrf::encode_point(&(
        //         "11765910627670138205555954470128887569457785139558335884609577674421928602465"
        //             .parse::<Integer>()
        //             .unwrap(),
        //         "18209892540234382838474494422429649302902580183111935078055540371838462697257"
        //             .parse::<Integer>()
        //             .unwrap()
        //     )),
        //     decode("299f6d20010556799ff82f2ad721bd15732f7533cfc6ad8bf333cd22166f42a8").unwrap()
        // );
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?} ", duration,);
    }
}
