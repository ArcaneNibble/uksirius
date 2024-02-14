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
    let mut fsk_bits_f = File::create("fsk.txt").unwrap();

    let mut fsk = SlidingGoertzelDFT::new(200, &[23, 24, 25, 28, 29, 30]);
    for inp_u in inp_data {
        let inp_lin = ulaw_to_f32(inp_u);
        let mut fskout = [0.0; 12];
        fsk.run(inp_lin, &mut fskout);

        let f1_windowed_re = -0.25 * fskout[0] + 0.5 * fskout[2] - 0.25 * fskout[4];
        let f1_windowed_im = -0.25 * fskout[1] + 0.5 * fskout[3] - 0.25 * fskout[5];

        let f0_windowed_re = -0.25 * fskout[6] + 0.5 * fskout[8] - 0.25 * fskout[10];
        let f0_windowed_im = -0.25 * fskout[7] + 0.5 * fskout[9] - 0.25 * fskout[11];

        let f0_mag_sq = f0_windowed_re * f0_windowed_re + f0_windowed_im * f0_windowed_im;
        let f1_mag_sq = f1_windowed_re * f1_windowed_re + f1_windowed_im * f1_windowed_im;
        // normalize, but div by N/2 because we threw away the symmetric bin
        let f0_mag = f0_mag_sq.sqrt() / 100.0;
        let f1_mag = f1_mag_sq.sqrt() / 100.0;

        // dbg!((f0_mag, f1_mag));
        fsk0_f.write_all(&(f0_mag * 10.0).to_le_bytes()).unwrap();
        fsk1_f.write_all(&(f1_mag * 10.0).to_le_bytes()).unwrap();

        // -40 dBm0 is a symbol of 52.15 / 8192
        if f0_mag > f1_mag && f0_mag >= (52.15 / 8192.0) {
            fsk_bits_f.write(&[b'0']).unwrap();
        } else if f1_mag > f0_mag && f1_mag >= (52.15 / 8192.0) {
            fsk_bits_f.write(&[b'1']).unwrap();
        } else {
            fsk_bits_f.write(&[b'x']).unwrap();
        }
    }
}
