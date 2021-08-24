use std::str;
use hex::{decode, encode};
use rug::{Assign, Integer};

lazy_static! {
    static ref SUITE_STRING: Vec<u8> = decode("04").unwrap();
    static ref DST: Vec<u8> =
        decode("45435652465f6564776172647332353531395f584d443a5348412d3531325f454c4c325f4e555f04")
            .unwrap();
    static ref BITS: usize = 256;
    static ref PRIME: Integer =
        "57896044618658097711785492504343953926634992332820282019728792003956564819949"
            .parse::<Integer>()
            .unwrap();
    static ref ORDER: Integer =
        "7237005577332262213973186563042994240857116359379907606001950938285454250989"
            .parse::<Integer>()
            .unwrap();
    static ref COFACTOR: Integer = "8".parse::<Integer>().unwrap();
    static ref TWO_INV: Integer =
        "28948022309329048855892746252171976963317496166410141009864396001978282409975"
            .parse::<Integer>()
            .unwrap();
    static ref II: Integer =
        "19681161376707505956807079304988542015446066515923890162744021073123829784752"
            .parse::<Integer>()
            .unwrap();
    static ref A: Integer = "486662".parse::<Integer>().unwrap();
    static ref D: Integer =
        "37095705934669439343138083508754565189542113879843219016388785533085940283555"
            .parse::<Integer>()
            .unwrap();
    static ref SQRT_MINUS_A_PLUS_2: Integer =
        "6853475219497561581579357271197624642482790079785650197046958215289687604742"
            .parse::<Integer>()
            .unwrap();
    static ref BASE_X: Integer =
        "15112221349535400772501151409588531511454012693041857206046113283949847762202"
            .parse::<Integer>()
            .unwrap();
    static ref BASE_Y: Integer =
        "46316835694926478169428394003475163141307993866256225615783033603165251855960"
            .parse::<Integer>()
            .unwrap();
    static ref BASE: (Integer, Integer) = (
        "15112221349535400772501151409588531511454012693041857206046113283949847762202"
            .parse::<Integer>()
            .unwrap(),
        "46316835694926478169428394003475163141307993866256225615783033603165251855960"
            .parse::<Integer>()
            .unwrap()
    );
}

pub fn modulus(a: &Integer, b: &Integer) -> Integer {
    <(Integer, Integer)>::from(a.div_rem_euc_ref(b)).1
}

pub fn inverse(a: &Integer) -> Integer {
    a.clone().invert(&*PRIME).unwrap_or(Integer::from(1))
}

pub fn pow(a: &Integer, b: &Integer, c: &Integer) -> Integer {
    let mut base = a.clone();
    let mut exp = b.clone();
    let m = c.clone();
    if m == 1 {
        return Integer::from(0);
    }
    let mut result = Integer::from(1);
    base = modulus(&base, &m);
    while exp > 0 {
        if Integer::from((&exp) & 1) == 1 {
            result = modulus(&Integer::from(&result * &base), &m);
        }
        exp >>= 1;
        base = modulus(&Integer::from(&base * &base), &m);
    }
    result
}

pub fn edwards_add(a: &(Integer, Integer), b: &(Integer, Integer)) -> (Integer, Integer) {
    let x1_y2 = Integer::from(&a.0 * &b.1);
    let x2_y1 = Integer::from(&a.1 * &b.0);
    let all = D.clone() * &x1_y2 * &x2_y1;
    let x3 = (x1_y2 + x2_y1) * inverse(&(Integer::from(1) + &all));
    let y3 = (Integer::from(&a.0 * &b.0) + Integer::from(&a.1 * &b.1))
        * inverse(&(Integer::from(1) - &all));
    (modulus(&x3, &*PRIME), modulus(&y3, &*PRIME))
}

pub fn scalar_multiply(p: &(Integer, Integer), scalar: &Integer) -> Option<(Integer, Integer)> {
    if *scalar == 0 {
        return Some((Integer::from(0), Integer::from(1)));
    }
    let scalar_bin = &format!("{:b}", &scalar)[1..];
    let mut q = p.clone();
    for i in scalar_bin.chars() {
        q = edwards_add(&q, &q);
        if i == '1' {
            q = edwards_add(&q, &p);
        }
    }
    Some(q)
}

pub fn x_recover(y: &Integer) -> Integer {
    let xx =
        (y * y - Integer::from(1)) * inverse(&((&*D) * Integer::from(y * y) + Integer::from(1)));
    let mut x = pow(
        &xx,
        &((&*PRIME + Integer::from(3)) / Integer::from(8)),
        &*PRIME,
    );
    if modulus(&(&x * &x - xx), &*PRIME) != Integer::from(0) {
        x = modulus(&Integer::from(&x * (&*II)), &*PRIME);
    }
    if &x & Integer::from(1) != 0 {
        &*PRIME - x
    } else {
        x
    }
}

pub fn decode_point(s: &[u8]) -> Option<(Integer,Integer)> {
    s.reverse();
    let y = (&encode(s).parse::<Integer>().ok()?) & ((Integer::from(1) << 255) - 1);
    let mut x = x_recover(y);
    if x & 1 != s[s.len()-1] & 1 {
        x = &*PRIME - x;
    }
    let p = (x, *y);
    if not _is_on_curve(p):
        return "INVALID"
    return p
}

pub fn ecvrf_decode_proof(pi: Vec<u8>) -> Option<((Integer,Integer),Integer,Integer)> {
    if pi.len() != 80 {
        return None
    }

    let gamma_string = &pi[0..32];
    let c_string = &pi[32..48];
    let s_string = &pi[48..];

    # 4. Gamma = string_to_point(gamma_string)
    gamma = _decode_point(gamma_string)

    # 5. if Gamma = "INVALID" output "INVALID" and stop.
    if gamma == "INVALID":
        return "INVALID"

    # 6. c = string_to_int(c_string)
    c = int.from_bytes(c_string, 'little')

    # 7. s = string_to_int(s_string)
    s = int.from_bytes(s_string, 'little')

    # 8. Output Gamma, c, and s
    return gamma, c, s
}

pub fn ecvrf_verify(y: Vec<u8>, pi: Vec<u8>, alpha: Vec<u8>) -> bool {
    let d = ecvrf_decode_proof(pi);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use sha3::Keccak256;
    use std::str::FromStr;

    #[test]
    fn modulus_test() {
        assert_eq!(
            modulus(&Integer::from(-4i8), &Integer::from(7i8)),
            Integer::from(3i8)
        );
    }

    #[test]
    fn inverse_test() {
        let a = "115792089237316195423570234324123"
            .parse::<Integer>()
            .unwrap();
        let b = "50185070121833820750509717279311425478202465867786279873084127885179732477785"
            .parse::<Integer>()
            .unwrap();
        assert_eq!(b, inverse(&a));
    }

    #[test]
    fn ecc_sqrt_test() {
        assert_eq!(
            "35634419551235720116798594689937697774970528779494777598852457192116356634056"
                .parse::<Integer>()
                .unwrap(),
            x_recover(
                &"50185070121833820750509717279311425478202465867786279873084127885179732477785"
                    .parse::<Integer>()
                    .unwrap()
            )
        );
        assert_eq!(
            "53301587420761876222207658879710286820900298918325969647217375986994648841896"
                .parse::<Integer>()
                .unwrap(),
            x_recover(&"3185713857305035135".parse::<Integer>().unwrap())
        );
        assert_eq!(
            "46177144718970195273346399805952030171392250782719158809116863111243864153332"
                .parse::<Integer>()
                .unwrap(),
            x_recover(
                &"87305764600495522745247520759120714246727049616"
                    .parse::<Integer>()
                    .unwrap()
            )
        );
    }

    #[test]
    fn edwards_add_test() {
        assert_eq!(
            edwards_add(
                &(Integer::from(1), Integer::from(2)),
                &(Integer::from(3), Integer::from(4)),
            ),
            (
                "30669472807527669052310166413469871322722837873560156671152128699509420332835"
                    .parse::<Integer>()
                    .unwrap(),
                "32803760088457211740806219601341938367891502708272204402052114923463521408048"
                    .parse::<Integer>()
                    .unwrap()
            )
        );
        assert_eq!(
            edwards_add(
                &(
                    "105245200036929210524520003692921052452000369292"
                        .parse::<Integer>()
                        .unwrap(),
                    "636368388952114463636838895211446363683889521144"
                        .parse::<Integer>()
                        .unwrap()
                ),
                &(
                    "365761262312465236576126231246523657612623124652"
                        .parse::<Integer>()
                        .unwrap(),
                    "599638831716981459963883171698145996388317169814"
                        .parse::<Integer>()
                        .unwrap()
                ),
            ),
            (
                "16094028690776613779404630311380383789228303041060010793878272985304591730114"
                    .parse::<Integer>()
                    .unwrap(),
                "56509461539446191492739335780640787740284013129997346250692191322113562145891"
                    .parse::<Integer>()
                    .unwrap()
            )
        );
    }

    #[test]
    fn scalar_multiply_test() {
        assert_eq!(
            scalar_multiply(
                &(
                    "2504841017466682250484101746668225048410174666822504841017466682"
                        .parse::<Integer>()
                        .unwrap(),
                    "1956113754237990195611375423799019561137542379901956113754237990"
                        .parse::<Integer>()
                        .unwrap()
                ),
                &"7126414032541130712641403254113071264140325411307126414032541130"
                    .parse::<Integer>()
                    .unwrap()
            ),
            Some((
                "3717741300534171586596133929728979624065571837388221471827653882295568582734"
                    .parse::<Integer>()
                    .unwrap(),
                "1221637037450835314506423104277906057339963056664048728491680523116867554868"
                    .parse::<Integer>()
                    .unwrap()
            ))
        );
        assert_eq!(
            scalar_multiply(
                &(
                    "2504841017466682250484101746668225048410174666822504841017466682"
                        .parse::<Integer>()
                        .unwrap(),
                    "1513453546461956113754237990195611375423799019561137542379901956113754237990"
                        .parse::<Integer>()
                        .unwrap(),
                ),
                &"74830380039917927238342598863222899552394587271096264578218486964046080567388"
                    .parse::<Integer>()
                    .unwrap(),
            ),
            Some((
                "10451491913815505047931853002078552559328154600536681248542806488509264630860"
                    .parse::<Integer>()
                    .unwrap(),
                "1891777415277742323394479244063570290330034114551949119047672059968424552778"
                    .parse::<Integer>()
                    .unwrap(),
            ))
        );

        assert_eq!(
            scalar_multiply(
                &(
                    "41580769168035012703902357280663015773275161554063216603182338549261711251193"
                        .parse::<Integer>()
                        .unwrap(),
                    "24911656077204456209601399282188369610223880089588176348139024489849710828841"
                        .parse::<Integer>()
                        .unwrap(),
                ),
                &"112366451224199189657043841110239819447199235354327421131306119208159432979989"
                    .parse::<Integer>()
                    .unwrap(),
            ),
            Some((
                "8072112576901302001883587473420904198649999849925609514862948818584399467310"
                    .parse::<Integer>()
                    .unwrap(),
                "35299203632341130723598861202244935989969207066742744119141421954087584890438"
                    .parse::<Integer>()
                    .unwrap(),
            ))
        );
    }
}
