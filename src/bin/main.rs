use std::cmp::Reverse;
use std::collections::BinaryHeap;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    ///sort by #pixels set
    s: bool,
    #[arg(short, long, default_value_t = String::from("latin"))]
    ///range all/ascii/latin/greek/hiragana/box/sga
    r: String,
    #[arg(short, long, default_value_t = String::from(""))]
    ///each char in string
    u: String,
}

fn main() {
    let args = Args::parse();

    let mut heap = BinaryHeap::new();

    if args.u.len() > 0 {
        for c in args.u.chars() {
            let u = c as u16;
            let b = font8x8::unicode2bitmap(u);
            heap.push(Reverse((b.count_ones(), c, u)));
        }
    } else {
        args.r
            .split(",")
            .into_iter()
            .map(|s| match s {
                "latin" => font8x8::UNICODE_LATIN,
                "greek" => font8x8::UNICODE_GREEK,
                "hiragana" => font8x8::UNICODE_HIRAGANA,
                "sga" => font8x8::UNICODE_SGA,
                "ascii" => font8x8::UNICODE_ASCII,
                "box" => font8x8::UNICODE_BOX,
                "all" => font8x8::UNICODE_ALL,
                _ => 1..0,
            })
            .for_each(|range| {
                for u in range {
                    let b = font8x8::unicode2bitmap(u);
                    if u == 0x20 || b != 0x0 {
                        let c = char::from_u32(u.into()).unwrap();
                        heap.push(Reverse(if args.s {
                            (b.count_ones(), c, u)
                        } else {
                            (u.into(), c, u)
                        }));
                    }
                }
            })
    }

    let mut n = 0;
    let n_all = heap.len();
    let mut s = String::new();
    while !heap.is_empty() {
        if let Some(Reverse((_, c, u))) = heap.pop() {
            n += 1;
            let b = font8x8::unicode2bitmap(u);
            println!(
                "{n}/{n_all}: Unicode 0x{u:x}: '{c}'; bits: {}/64",
                b.count_ones()
            );
            font8x8::display(b);
            s.push(c);
        }
    }
    println!("{s}");
}
