use std::{fs::File, io::Read};

use uksirius::modem::*;

fn main() {
    let mut inp_f = File::open("rx.ulaw").unwrap();
    let mut inp_data = Vec::new();
    inp_f.read_to_end(&mut inp_data).unwrap();

    let mut fsk = FskDemod::new(300.0, 1180.0, 980.0);

    for inp_u in inp_data {
        let maybe_byte = fsk.process(inp_u);

        if let Some(b) = maybe_byte {
            println!("char done! 0x{:02X} 0b{:08b}", b, b);
        }
    }
}
