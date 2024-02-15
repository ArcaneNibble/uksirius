use std::{
    fs::File,
    io::{Read, Write},
};

use uksirius::modem::*;

fn main() {
    // input test
    let mut inp_f = File::open("rx.ulaw").unwrap();
    let mut inp_data = Vec::new();
    inp_f.read_to_end(&mut inp_data).unwrap();

    let mut fsk_demod = FskDemod::new(300.0, 1180.0, 980.0);

    for inp_u in inp_data {
        let maybe_byte = fsk_demod.process(inp_u);

        if let Some(b) = maybe_byte {
            println!("char done! 0x{:02X} 0b{:08b}", b, b);
        }
    }

    // totally independent output test
    let mut out_f = File::create("v21-test.ulaw").unwrap();
    let mut fsk_mod = FskEncoder::new(300.0, 1180.0, 980.0);
    fsk_mod.add_specials(&[0x3ff]);
    fsk_mod.add_bytes(&[0xe0, 0xc1]);
    for _block in 0..1000 {
        let mut buf = [0u8; 8]; // 1 ms
        let needs_more = fsk_mod.run(&mut buf);
        // dbg!(needs_more);
        if needs_more {
            fsk_mod.add_specials(&[0x3ff]);
            fsk_mod.add_bytes(&[0xe0, 0xc1]);
        }
        out_f.write_all(&buf).unwrap();
    }
}
