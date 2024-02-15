use std::{
    fs::File,
    io::{Read, Write},
};

use uksirius::modem::*;

fn main() {
    let mut inp_f = File::open("rx.ulaw").unwrap();
    let mut inp_data = Vec::new();
    inp_f.read_to_end(&mut inp_data).unwrap();
    let mut fsk0_f = File::create("fsk0.f32").unwrap();
    let mut fsk1_f = File::create("fsk1.f32").unwrap();
    let mut fskdiff_f = File::create("fskdiff.f32").unwrap();
    let mut fsk_bits_f = File::create("fsk.txt").unwrap();

    let mut fsk = FskDemod::new(300.0, 1180.0, 980.0);
    let mut uart = UartDecoder::new(300.0);

    for inp_u in inp_data {
        let (corr_0, corr_1) = fsk.process(inp_u);

        fsk0_f.write_all(&(corr_0 * 10.0).to_le_bytes()).unwrap();
        fsk1_f.write_all(&(corr_1 * 10.0).to_le_bytes()).unwrap();
        fskdiff_f
            .write_all(&(corr_1 - corr_0).to_le_bytes())
            .unwrap();

        // -40 dBm0 is a symbol of 52.15 / 8192
        let bit = if corr_0 > corr_1 && corr_0 >= (52.15 / 8192.0) {
            fsk_bits_f.write(&[b'0']).unwrap();
            0
        } else if corr_1 > corr_0 && corr_1 >= (52.15 / 8192.0) {
            fsk_bits_f.write(&[b'1']).unwrap();
            1
        } else {
            fsk_bits_f.write(&[b'x']).unwrap();
            -1
        };

        let maybe_byte = uart.process(bit);
        if let Some(b) = maybe_byte {
            println!("char done! 0x{:02X} 0b{:08b}", b, b);
        }
    }
}
