pub mod font8x8;

fn main() {
    // let mut v = Vec::new();
    // for (i, b) in font8x8::FONT8X8.iter().enumerate() {
    //     let x = bm_brightness(b);
    //     if let Some(u) = font8x8::index2unicode(i) {
    //         if x != 64 || u == 0x20 {
    //             v.push((x, i));
    //         }
    //         //println!("bitmap#{i}");
    //         //bm_display(b)
    //     }
    // }
    // v.sort();
    // for (x, i) in v {
    //     let b = &font8x8::FONT8X8[i];
    //     if let Some(u) = font8x8::index2unicode(i) {
    //         println!(
    //             "Index {i} -  Code u+{u:x} : {}; brightness: {x}",
    //             char::from_u32(u.try_into().unwrap()).unwrap(),
    //         );
    //         bm_display(b);
    //     }
    // }

    // for (i, b) in font8x8::FONT8X8.iter().enumerate() {
    //     println!("bitmap#{i}");
    //     bm_display(b)
    // }

    //for u in font8x8::UNICODE_RANGE {
    for u in font8x8::UNICODE_LATIN {
        let b = font8x8::unicode2bitmap(u);
        if u == 0x20 || b != 0x0 {
            let c = char::from_u32(u.into()).unwrap();
            println!("Code 0x{u:x} : '{c}'; brightness: {}", b.count_zeros());
            font8x8::display(b)
        }
    }
}
