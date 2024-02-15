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

    for inp_u in inp_data {
        let (corr_0, corr_1) = fsk.process(inp_u);

        fsk0_f.write_all(&(corr_0 * 10.0).to_le_bytes()).unwrap();
        fsk1_f.write_all(&(corr_1 * 10.0).to_le_bytes()).unwrap();
        fskdiff_f
            .write_all(&(corr_1 - corr_0).to_le_bytes())
            .unwrap();

        // -40 dBm0 is a symbol of 52.15 / 8192
        if corr_0 > corr_1 && corr_0 >= (52.15 / 8192.0) {
            fsk_bits_f.write(&[b'0']).unwrap();
        } else if corr_1 > corr_0 && corr_1 >= (52.15 / 8192.0) {
            fsk_bits_f.write(&[b'1']).unwrap();
        } else {
            fsk_bits_f.write(&[b'x']).unwrap();
        }

        /*let y_980 = inp_lin * biquad_980_b0 + inp_prev[1] * biquad_980_b2
            - biquad_980_a1 * y_prev_980[0]
            - biquad_980_a2 * y_prev_980[1];
        let y_1180 = inp_lin * biquad_1180_b0 + inp_prev[1] * biquad_1180_b2
            - biquad_1180_a1 * y_prev_1180[0]
            - biquad_1180_a2 * y_prev_1180[1];
        inp_prev[1] = inp_prev[0];
        inp_prev[0] = inp_lin;

        // have tones through a biquad now, compute abs while doing dc lpf
        // *before* we shuffle prev
        let dc_980 = y_980.abs() * biquad_lpf_b0
            + y_prev_980[0].abs() * biquad_lpf_b1
            + y_prev_980[1].abs() * biquad_lpf_b2
            - dc_prev_980[0] * biquad_lpf_a1
            - dc_prev_980[1] * biquad_lpf_a2;
        let dc_1180 = y_1180.abs() * biquad_lpf_b0
            + y_prev_1180[0].abs() * biquad_lpf_b1
            + y_prev_1180[1].abs() * biquad_lpf_b2
            - dc_prev_1180[0] * biquad_lpf_a1
            - dc_prev_1180[1] * biquad_lpf_a2;
        y_prev_980[1] = y_prev_980[0];
        y_prev_980[0] = y_980;
        y_prev_1180[1] = y_prev_1180[0];
        y_prev_1180[0] = y_1180;
        dc_prev_980[1] = dc_prev_980[0];
        dc_prev_980[0] = dc_980;
        dc_prev_1180[1] = dc_prev_1180[0];
        dc_prev_1180[0] = dc_1180;

        fsk0_f.write_all(&(dc_1180 * 10.0).to_le_bytes()).unwrap();
        fsk1_f.write_all(&(dc_980 * 10.0).to_le_bytes()).unwrap();
        fskdiff_f
            .write_all(&((dc_980 - dc_1180) * 10.0).to_le_bytes())
            .unwrap();

        if dc_1180 > dc_980 && dc_1180 >= (20.0 / 8192.0) {
            fsk_bits_f.write(&[b'0']).unwrap();
        } else if dc_980 > dc_1180 && dc_980 >= (20.0 / 8192.0) {
            fsk_bits_f.write(&[b'1']).unwrap();
        } else {
            fsk_bits_f.write(&[b'x']).unwrap();
        }*/

        /*let mut fskout = [0.0; 4];
        fsk.run(inp_lin, &mut fskout);
        // dbg!(fskout);

        let f0_mag_sq = fskout[2] * fskout[2] + fskout[3] * fskout[3];
        let f1_mag_sq = fskout[0] * fskout[0] + fskout[1] * fskout[1];
        let f0_mag = f0_mag_sq.sqrt() / 12.5;
        let f1_mag = f1_mag_sq.sqrt() / 12.5;
        fsk0_f.write_all(&(f0_mag).to_le_bytes()).unwrap();
        fsk1_f.write_all(&(f1_mag).to_le_bytes()).unwrap();
        fskdiff_f
            .write_all(&(f1_mag - f0_mag).to_le_bytes())
            .unwrap();*/

        /*let f1_windowed_re = -0.25 * fskout[0] + 0.5 * fskout[2] - 0.25 * fskout[4];
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
        }*/
    }
}
