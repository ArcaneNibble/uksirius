use std::{fs::File, io::Read};

use uksirius::modem::*;

fn main() {
    let mut inp_f = File::open("rx.ulaw").unwrap();
    let mut inp_data = Vec::new();
    inp_f.read_to_end(&mut inp_data).unwrap();

    let mut fsk = SlidingGoertzelDFT::new(200, &[22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
    for inp_u in inp_data {
        let inp_lin = ulaw_to_f32(inp_u);
        let mut fskout = [0.0; 20];
        fsk.run(inp_lin, &mut fskout);
        dbg!(fskout);
    }
}
