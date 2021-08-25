#[macro_use]
extern crate lazy_static;

use hex::decode;
use std::time::Instant;
mod ecvrf;

fn main() {
    let samples = &[
        (
            &decode("fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025").unwrap(),
            &decode("926e895d308f5e328e7aa159c06eddbe56d06846abf5d98c2512235eaa57fdce6187befa109606682503b3a1424f0f729ca0418099fbd86a48093e6a8de26307b8d93e02da927e6dd5b73c8f119aee0f").unwrap(),
            &vec![175, 130]
        ),
        (
            &decode("b78bfbbd68ca4915c854a4cc04afa79ab35a393931a5388db306da94a9d0d2c3").unwrap(),
            &decode("8057fc57942da97027ea37353d22c6e63c81961574424e1f60e406a0791d6a460700700bf2926d16872a7e8240898db4f239e0f68473503c61f74f19a27c182373ec99ab5c871b2305f5d7bd1c95da08").unwrap(),
            &decode("34a11e19fd3650e9b7818fc33a1e0fc02c44557ac8").unwrap()
        ),
        (
            &decode("a02bfb0cfb12309df90e9526ae47a24b124a4ffaed81bf6b07c753b6b82f6bf9").unwrap(),
            &decode("d54af185e270e6a13c9f7c81b51f4d965de24b0f3f671b6796a4ad79052a6e21692a495bc2e2ff5af70146d29256b374b9a1ea30d3cd65d618eed127ce425243a7b140310e4885e9f1fe87c7804dfa07").unwrap(),
            &decode("e7b284553c289e5a337389396071facc33").unwrap()
        ),
        (
            &decode("ea1fd48f1dc23a497d7b1d92312147a339411068d30b55fadc9b8c3445cb7d5e").unwrap(),
            &decode("693e2a5a3622555946c2524d17b23aae8943d3d838a9290c2515f43064563ca766efcb2471b40be3c5cc576f5aed0326936b9d62b1e47c8dfae3593ebb52d76152418107a28a89d619f5ca42afc87401").unwrap(),
            &decode("e7d98d7f3657f5cbfdeaf1e4ec6e46f863ddb65b0029513504df135d81d6").unwrap()
        ),
        (
            &decode("9819835648509c20d360efb16f46d35867453666eca8480577c4f352886bb804").unwrap(),
            &decode("f448b5e3cedb74a497c41dd604ade9a41d21b85e27784e9338dcaf53d92c88b72fb5ff8a569dfe6acea5041b6b885814960e1d54086cb3179f5f6bed647dd226593167908e087075b6f52814073a4a0e").unwrap(),
            &decode("ac1bf71f01ebd78aa078116ce31f44913934e2deced0992ed3b9b2693426e2e70b551daef3f1d278e0d4f65df64d").unwrap()
        ),
        (
            &decode("99f1df8e5d3446c34db84237b93e68822b167e03df95085e543a106ec7f58838").unwrap(),
            &decode("52c8bd4d987d0691c8ebec363d9a3a7117dd14afdc3118e357f17e57694d9a8cc4d1458bafda86a7603860fc690a04b1a3d60f290754c68d16d24a7f900fa00ddef512b52ee2c2ca30fc953a8516120c").unwrap(),
            &decode("8b79cb91750e20640b9543777765a62b16f2e68253b3d1a991f3fdc4390f80").unwrap()
        ),
        (
            &decode("57fd770e847f2fa8b57cf2ea302705920d4f9528acdb2b2a021db879d2a564c4").unwrap(),
            &decode("ed501d5303e8ef47844333794b90ffead4dcc4bf5c70a30c550aa9865b12e78dadf3cdbf9d196b958185dab5ca205ed99b0ddd121e2a35651691e8088c39f4b8685c03dbe989f8bda84e2359da383e0b").unwrap(),
            &decode("e7006081935240bbef5f6a672c1a4ab19a7ed3620f9c1d15c7f39a809a24e2279a0edb12ebab52").unwrap()
        ),
        (
            &decode("a4caf3d2490964a005aac0ce72b48d3b0b16ed71642dd1924cfd9ced791f20ae").unwrap(),
            &decode("3749182be83150269a0327fe78714e9b1ce69e4a67cb4cbe60cbd81bebcdf805cef8d4b78fb5660d220bc6a78c46fc575ca801cbcc6ff4980b86020b57de8f9fc068988474fd41e27b589fbaea75330f").unwrap(),
            &decode("0b73de15a3268f198f98d93d498952515588152f74c557b37d4bd221e432e4e05bcf6b92e563").unwrap()
        ),
        (
            &decode("b6fbdd5afef4468045378ffb3f0eb4d612e0adb73e33bd7cb7c960f6b456993a").unwrap(),
            &decode("c746027645816449ad98d50fb82c17549ff190de68f98d1f8004d57bb61e33347752ecdcb0392ab4a75f2e57b4203e779e4c81b37af250342ff7419717dd5198e55fbf0eaa3c25dfdf4d8e1d63e3ff01").unwrap(),
            &decode("0c6627668e5e5222bbcab544d5fcf0").unwrap()
        )
    ];

    for s in samples.iter() {
        let start = Instant::now();
        let verify_result = ecvrf::ecvrf_verify(s.0, s.1, s.2);
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?} ", duration);
        assert_eq!(verify_result, true)
    }
}
