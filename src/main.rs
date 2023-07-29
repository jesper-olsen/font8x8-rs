pub mod font8x8;
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
    ///all/latin/greek/hiragana
    u: String,
}

fn main() {
    let args = Args::parse();

    let mut heap = BinaryHeap::new();

    let range = match args.u.as_str() {
        "latin" => font8x8::UNICODE_LATIN,
        "greek" => font8x8::UNICODE_GREEK,
        "hiragana" => font8x8::UNICODE_HIRAGANA,
        "sga" => font8x8::UNICODE_SGA,
        "ascii" => font8x8::UNICODE_ASCII,
        "box" => font8x8::UNICODE_BOX,
        _ => font8x8::UNICODE_ALL,
    };
    for u in range {
        let b = font8x8::unicode2bitmap(u);
        if u == 0x20 || b != 0x0 {
            let c = char::from_u32(u.into()).unwrap();
            if args.s {
                heap.push(Reverse((b.count_ones(), c, u)));
            } else {
                heap.push(Reverse((u.into(), c, u)));
            }
        }
    }

    let mut n = 0;
    let n_all = heap.len();
    while !heap.is_empty() {
        if let Some(Reverse((_, c, u))) = heap.pop() {
            n += 1;
            let b = font8x8::unicode2bitmap(u);
            println!(
                "{n}/{n_all}: Unicode 0x{u:x}: '{c}'; bits: {}/64",
                b.count_ones()
            );
            font8x8::display(b);
        }
    }
}
