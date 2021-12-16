use anyhow::Result;
use itertools::Itertools;

enum Packet{
    LITERAL {
        version: u8,
        type_id: u8,
        value: u64,
    },
    OPERATOR {
        version: u8,
        type_id: u8,
        packets: Vec<Packet>
    },
}

fn bit_index(i: u8, b: u8) -> bool {
    let selector = 1 << (7-i);
    b & selector != 0
}

fn number(it: &mut dyn Iterator<Item=bool>, n: u64) -> Option<u64> {
    let taken: Vec<_> = it.take(n as usize).collect();
    if taken.len() == n as usize {
        Some(
            taken.into_iter().zip(0..n).map(|(b, i)| if b {
                1 << (n - 1 - i)
            } else {
                0
            }).fold(0, |acc, n| {
                acc | n
            })
        )
    } else {
        None
    }
}

fn literal(it: &mut dyn Iterator<Item=bool>) -> u64 {
    let mut ret: u64 = 0;
    let mut last = false;
    while !last {
        let word = number(it, 5).unwrap();
        last = word & 0x10 == 0;
        ret = (ret << 4) | (word & 0xf);
    }
    ret
}

fn operator(it: &mut dyn Iterator<Item=bool>) -> Option<Vec<Packet>> {
    if let Some(length_type_id) = number(it, 1) {
        Some(
            if length_type_id == 0 {
                let length = number(it, 15).unwrap() as usize;
                let mut sub_it = it.take(length);
                packets(&mut sub_it)
            } else {
                let sub_packets = number(it, 11).unwrap() as usize;
                packets_limit(it, sub_packets)
            }
        )
    } else {
        None
    }
}

fn packet(it: &mut dyn Iterator<Item=bool>) -> Option<Packet> {
    if let Some(version) = number(it, 3) {
        let version = version as u8;
        let type_id = number(it, 3).unwrap() as u8;
        match type_id {
            4 => {
                let value = literal(it);
                Some(
                    Packet::LITERAL {
                        version,
                        type_id,
                        value
                    }
                )
            },
            _ => {
                if let Some(packets) = operator(it) {
                    Some(Packet::OPERATOR {
                        version,
                        type_id,
                        packets
                    })
                } else {
                    None
                }

            }
        }
    } else {
        None
    }
}

fn packets(it: &mut dyn Iterator<Item=bool>) -> Vec<Packet> {
    let mut ret = Vec::new();
    while let Some(p) = packet(it) {
        ret.push(p)
    }
    ret
}

fn packets_limit(it: &mut dyn Iterator<Item=bool>, n: usize) -> Vec<Packet> {
    let mut ret = Vec::new();
    while ret.len() < n {
        ret.push(packet(it).unwrap())
    }
    ret
}

fn version_sum(packets: &[Packet]) -> u64 {
    packets.iter().map(|p| match p {
        Packet::LITERAL { version, .. } => { *version as u64 }
        Packet::OPERATOR { version, packets: sub_packets, .. } => {
            *version as u64 + version_sum(&sub_packets)
        }
    }).sum()
}

fn main() -> Result<()> {
    let input = INPUT;
    let mut bs = Vec::new();
    for (ch0,ch1) in input.chars().tuples() {
        let mut b: u8 = ch0.to_digit(16).unwrap() as u8;
        b = b << 4 | ch1.to_digit(16).unwrap() as u8;
        bs.push(b);
    }

    let bits: Vec<bool> = bs.iter().flat_map(|b| {
        (0..8).map(move |i| bit_index(i, *b))
    }).collect();

    let mut it = bits.into_iter();

    let ps = packets(&mut it);

    let ver_sum = version_sum(&ps);
    println!("Part 1 {}", ver_sum);

    Ok(())
}

const INPUT: &str = r#"005410C99A9802DA00B43887138F72F4F652CC0159FE05E802B3A572DBBE5AA5F56F6B6A4600FCCAACEA9CE0E1002013A55389B064C0269813952F983595234002DA394615002A47E06C0125CF7B74FE00E6FC470D4C0129260B005E73FCDFC3A5B77BF2FB4E0009C27ECEF293824CC76902B3004F8017A999EC22770412BE2A1004E3DCDFA146D00020670B9C0129A8D79BB7E88926BA401BAD004892BBDEF20D253BE70C53CA5399AB648EBBAAF0BD402B95349201938264C7699C5A0592AF8001E3C09972A949AD4AE2CB3230AC37FC919801F2A7A402978002150E60BC6700043A23C618E20008644782F10C80262F005679A679BE733C3F3005BC01496F60865B39AF8A2478A04017DCBEAB32FA0055E6286D31430300AE7C7E79AE55324CA679F9002239992BC689A8D6FE084012AE73BDFE39EBF186738B33BD9FA91B14CB7785EC01CE4DCE1AE2DCFD7D23098A98411973E30052C012978F7DD089689ACD4A7A80CCEFEB9EC56880485951DB00400010D8A30CA1500021B0D625450700227A30A774B2600ACD56F981E580272AA3319ACC04C015C00AFA4616C63D4DFF289319A9DC401008650927B2232F70784AE0124D65A25FD3A34CC61A6449246986E300425AF873A00CD4401C8A90D60E8803D08A0DC673005E692B000DA85B268E4021D4E41C6802E49AB57D1ED1166AD5F47B4433005F401496867C2B3E7112C0050C20043A17C208B240087425871180C01985D07A22980273247801988803B08A2DC191006A2141289640133E80212C3D2C3F377B09900A53E00900021109623425100723DC6884D3B7CFE1D2C6036D180D053002880BC530025C00F700308096110021C00C001E44C00F001955805A62013D0400B400ED500307400949C00F92972B6BC3F47A96D21C5730047003770004323E44F8B80008441C8F51366F38F240"#;
