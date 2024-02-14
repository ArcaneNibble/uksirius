// important stuff
// f_s = 8000 Hz
// 0 dBm0 = 5215/8192 signal value

use std::{f32::consts::PI, fs::File, io::Write};

pub const ULAW_0: u8 = 0xff;

const ULAW_TO_F32: [f32; 256] = [
    // 0x00
    -8031.0 / 8192.0,
    -7775.0 / 8192.0,
    -7519.0 / 8192.0,
    -7263.0 / 8192.0,
    -7007.0 / 8192.0,
    -6751.0 / 8192.0,
    -6495.0 / 8192.0,
    -6239.0 / 8192.0,
    -5983.0 / 8192.0,
    -5727.0 / 8192.0,
    -5471.0 / 8192.0,
    -5215.0 / 8192.0,
    -4959.0 / 8192.0,
    -4703.0 / 8192.0,
    -4447.0 / 8192.0,
    -4191.0 / 8192.0,
    // 0x10
    -3999.0 / 8192.0,
    -3871.0 / 8192.0,
    -3743.0 / 8192.0,
    -3615.0 / 8192.0,
    -3487.0 / 8192.0,
    -3359.0 / 8192.0,
    -3231.0 / 8192.0,
    -3103.0 / 8192.0,
    -2975.0 / 8192.0,
    -2847.0 / 8192.0,
    -2719.0 / 8192.0,
    -2591.0 / 8192.0,
    -2463.0 / 8192.0,
    -2335.0 / 8192.0,
    -2207.0 / 8192.0,
    -2079.0 / 8192.0,
    // 0x20
    -1983.0 / 8192.0,
    -1919.0 / 8192.0,
    -1855.0 / 8192.0,
    -1791.0 / 8192.0,
    -1727.0 / 8192.0,
    -1663.0 / 8192.0,
    -1599.0 / 8192.0,
    -1535.0 / 8192.0,
    -1471.0 / 8192.0,
    -1407.0 / 8192.0,
    -1343.0 / 8192.0,
    -1279.0 / 8192.0,
    -1215.0 / 8192.0,
    -1151.0 / 8192.0,
    -1087.0 / 8192.0,
    -1023.0 / 8192.0,
    // 0x30
    -975.0 / 8192.0,
    -943.0 / 8192.0,
    -911.0 / 8192.0,
    -879.0 / 8192.0,
    -847.0 / 8192.0,
    -815.0 / 8192.0,
    -783.0 / 8192.0,
    -751.0 / 8192.0,
    -719.0 / 8192.0,
    -687.0 / 8192.0,
    -655.0 / 8192.0,
    -623.0 / 8192.0,
    -591.0 / 8192.0,
    -559.0 / 8192.0,
    -527.0 / 8192.0,
    -495.0 / 8192.0,
    // 0x40
    -471.0 / 8192.0,
    -455.0 / 8192.0,
    -439.0 / 8192.0,
    -423.0 / 8192.0,
    -407.0 / 8192.0,
    -391.0 / 8192.0,
    -375.0 / 8192.0,
    -359.0 / 8192.0,
    -343.0 / 8192.0,
    -327.0 / 8192.0,
    -311.0 / 8192.0,
    -295.0 / 8192.0,
    -279.0 / 8192.0,
    -263.0 / 8192.0,
    -247.0 / 8192.0,
    -231.0 / 8192.0,
    // 0x50
    -219.0 / 8192.0,
    -211.0 / 8192.0,
    -203.0 / 8192.0,
    -195.0 / 8192.0,
    -187.0 / 8192.0,
    -179.0 / 8192.0,
    -171.0 / 8192.0,
    -163.0 / 8192.0,
    -155.0 / 8192.0,
    -147.0 / 8192.0,
    -139.0 / 8192.0,
    -131.0 / 8192.0,
    -123.0 / 8192.0,
    -115.0 / 8192.0,
    -107.0 / 8192.0,
    -99.0 / 8192.0,
    // 0x60
    -93.0 / 8192.0,
    -89.0 / 8192.0,
    -85.0 / 8192.0,
    -81.0 / 8192.0,
    -77.0 / 8192.0,
    -73.0 / 8192.0,
    -69.0 / 8192.0,
    -65.0 / 8192.0,
    -61.0 / 8192.0,
    -57.0 / 8192.0,
    -53.0 / 8192.0,
    -49.0 / 8192.0,
    -45.0 / 8192.0,
    -41.0 / 8192.0,
    -37.0 / 8192.0,
    -33.0 / 8192.0,
    // 0x70
    -30.0 / 8192.0,
    -28.0 / 8192.0,
    -26.0 / 8192.0,
    -24.0 / 8192.0,
    -22.0 / 8192.0,
    -20.0 / 8192.0,
    -18.0 / 8192.0,
    -16.0 / 8192.0,
    -14.0 / 8192.0,
    -12.0 / 8192.0,
    -10.0 / 8192.0,
    -8.0 / 8192.0,
    -6.0 / 8192.0,
    -4.0 / 8192.0,
    -2.0 / 8192.0,
    -1.0 / 8192.0,
    // 0x80
    8031.0 / 8192.0,
    7775.0 / 8192.0,
    7519.0 / 8192.0,
    7263.0 / 8192.0,
    7007.0 / 8192.0,
    6751.0 / 8192.0,
    6495.0 / 8192.0,
    6239.0 / 8192.0,
    5983.0 / 8192.0,
    5727.0 / 8192.0,
    5471.0 / 8192.0,
    5215.0 / 8192.0,
    4959.0 / 8192.0,
    4703.0 / 8192.0,
    4447.0 / 8192.0,
    4191.0 / 8192.0,
    // 0x90
    3999.0 / 8192.0,
    3871.0 / 8192.0,
    3743.0 / 8192.0,
    3615.0 / 8192.0,
    3487.0 / 8192.0,
    3359.0 / 8192.0,
    3231.0 / 8192.0,
    3103.0 / 8192.0,
    2975.0 / 8192.0,
    2847.0 / 8192.0,
    2719.0 / 8192.0,
    2591.0 / 8192.0,
    2463.0 / 8192.0,
    2335.0 / 8192.0,
    2207.0 / 8192.0,
    2079.0 / 8192.0,
    // 0xA0
    1983.0 / 8192.0,
    1919.0 / 8192.0,
    1855.0 / 8192.0,
    1791.0 / 8192.0,
    1727.0 / 8192.0,
    1663.0 / 8192.0,
    1599.0 / 8192.0,
    1535.0 / 8192.0,
    1471.0 / 8192.0,
    1407.0 / 8192.0,
    1343.0 / 8192.0,
    1279.0 / 8192.0,
    1215.0 / 8192.0,
    1151.0 / 8192.0,
    1087.0 / 8192.0,
    1023.0 / 8192.0,
    // 0xB0
    975.0 / 8192.0,
    943.0 / 8192.0,
    911.0 / 8192.0,
    879.0 / 8192.0,
    847.0 / 8192.0,
    815.0 / 8192.0,
    783.0 / 8192.0,
    751.0 / 8192.0,
    719.0 / 8192.0,
    687.0 / 8192.0,
    655.0 / 8192.0,
    623.0 / 8192.0,
    591.0 / 8192.0,
    559.0 / 8192.0,
    527.0 / 8192.0,
    495.0 / 8192.0,
    // 0xC0
    471.0 / 8192.0,
    455.0 / 8192.0,
    439.0 / 8192.0,
    423.0 / 8192.0,
    407.0 / 8192.0,
    391.0 / 8192.0,
    375.0 / 8192.0,
    359.0 / 8192.0,
    343.0 / 8192.0,
    327.0 / 8192.0,
    311.0 / 8192.0,
    295.0 / 8192.0,
    279.0 / 8192.0,
    263.0 / 8192.0,
    247.0 / 8192.0,
    231.0 / 8192.0,
    // 0xD0
    219.0 / 8192.0,
    211.0 / 8192.0,
    203.0 / 8192.0,
    195.0 / 8192.0,
    187.0 / 8192.0,
    179.0 / 8192.0,
    171.0 / 8192.0,
    163.0 / 8192.0,
    155.0 / 8192.0,
    147.0 / 8192.0,
    139.0 / 8192.0,
    131.0 / 8192.0,
    123.0 / 8192.0,
    115.0 / 8192.0,
    107.0 / 8192.0,
    99.0 / 8192.0,
    // 0xE0
    93.0 / 8192.0,
    89.0 / 8192.0,
    85.0 / 8192.0,
    81.0 / 8192.0,
    77.0 / 8192.0,
    73.0 / 8192.0,
    69.0 / 8192.0,
    65.0 / 8192.0,
    61.0 / 8192.0,
    57.0 / 8192.0,
    53.0 / 8192.0,
    49.0 / 8192.0,
    45.0 / 8192.0,
    41.0 / 8192.0,
    37.0 / 8192.0,
    33.0 / 8192.0,
    // 0xF0
    30.0 / 8192.0,
    28.0 / 8192.0,
    26.0 / 8192.0,
    24.0 / 8192.0,
    22.0 / 8192.0,
    20.0 / 8192.0,
    18.0 / 8192.0,
    16.0 / 8192.0,
    14.0 / 8192.0,
    12.0 / 8192.0,
    10.0 / 8192.0,
    8.0 / 8192.0,
    6.0 / 8192.0,
    4.0 / 8192.0,
    2.0 / 8192.0,
    0.0 / 8192.0,
];

pub fn ulaw_to_f32(ulaw: u8) -> f32 {
    ULAW_TO_F32[ulaw as usize]
}

pub fn f32_to_ulaw(mut fval: f32) -> u8 {
    // https://dspguru.com/dsp/tricks/fast-floating-point-to-mu-law-conversion/
    // but modified

    let is_neg = if fval < 0.0 {
        // shift [−8159, -1] to [−8158, -0] for symmetry
        fval = -(fval + 1.0 / 8192.0);
        true
    } else {
        false
    };

    fval = fval.clamp(0.0, 8158.0 / 8192.0);

    // magic bias
    fval += 33.0 / 8192.0;
    let fbits = fval.to_bits();
    // magic subtract
    let mut mu = (0x16f - ((fbits >> 19) & 0xff)) & 0x7f;
    if !is_neg {
        mu |= 0x80;
    }

    mu as u8
}

#[derive(Debug)]
pub struct AnsAmGen {
    sample_num: u32,
}
impl AnsAmGen {
    const AMPLITUDE: f32 = 1000.0 / 8192.0;
    const ANSAM_FREQ: f32 = 2100.0;
    const MOD_FREQ: f32 = 15.0;
    const PHASE_REVERSAL_SAMPLES: u32 = 3600; // 450 ms
    const TIME_LIMIT: u32 = 5 * 8000;

    pub fn new() -> Self {
        Self { sample_num: 0 }
    }

    pub fn run(&mut self, out: &mut [u8]) -> bool {
        for i in 0..out.len() {
            if self.sample_num >= Self::TIME_LIMIT {
                out[i] = ULAW_0;
            } else {
                let modulation =
                    0.2 * (2.0 * PI * Self::MOD_FREQ * (self.sample_num as f32) / 8000.0).sin();
                // modulation is +- 0.2
                let phase_shift = (self.sample_num / Self::PHASE_REVERSAL_SAMPLES) % 2 == 1;
                // if phase_shift {
                //     dbg!(self.sample_num);
                // }
                let carrier = Self::AMPLITUDE
                    * (1.0 + modulation)
                    * (2.0 * PI * Self::ANSAM_FREQ * (self.sample_num as f32) / 8000.0
                        + if phase_shift { PI } else { 0.0 })
                    .sin();
                // dbg!(carrier);
                let wave_u = f32_to_ulaw(carrier);
                out[i] = wave_u;
                self.sample_num += 1;
            }
        }

        self.sample_num >= Self::TIME_LIMIT
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ModemFSM {
    AnswerWait,
    AnsAm,
}

#[derive(Debug)]
pub struct ModemState {
    // xxx
    timestamp: u64,
    fsm: ModemFSM,
    ansam: AnsAmGen,
}
impl ModemState {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            fsm: ModemFSM::AnswerWait,
            ansam: AnsAmGen::new(),
        }
    }

    pub fn process(&mut self, inp: &[u8], outp: &mut [u8]) {
        match self.fsm {
            ModemFSM::AnswerWait => {
                self.timestamp += inp.len() as u64;
                if self.timestamp >= 3200 {
                    println!("done waiting, will send ansam");
                    self.fsm = ModemFSM::AnsAm;
                }
            }
            ModemFSM::AnsAm => {
                for inp_u in inp {
                    let inp_lin = ulaw_to_f32(*inp_u);

                    // todo
                }
                self.timestamp += inp.len() as u64;
                let _timeout = self.ansam.run(outp);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use super::*;

    #[test]
    fn ulaw_roundtrip() {
        for x in 0..256 {
            let fout = ulaw_to_f32(x as u8);
            let rt_mu = f32_to_ulaw(fout);
            println!("0x{:02X} -> {} -> 0x{:02X}", x, fout * 8192.0, rt_mu);
            assert_eq!(x as u8, rt_mu);
        }
    }

    #[test]
    fn ulaw_decision_vals() {
        let mut decision_vals = vec![8159];
        for _ in 0..16 {
            decision_vals.push(decision_vals.last().unwrap() - 256);
        }
        for _ in 0..16 {
            decision_vals.push(decision_vals.last().unwrap() - 128);
        }
        for _ in 0..16 {
            decision_vals.push(decision_vals.last().unwrap() - 64);
        }
        for _ in 0..16 {
            decision_vals.push(decision_vals.last().unwrap() - 32);
        }
        for _ in 0..16 {
            decision_vals.push(decision_vals.last().unwrap() - 16);
        }
        for _ in 0..16 {
            decision_vals.push(decision_vals.last().unwrap() - 8);
        }
        for _ in 0..16 {
            decision_vals.push(decision_vals.last().unwrap() - 4);
        }
        for _ in 0..15 {
            decision_vals.push(decision_vals.last().unwrap() - 2);
        }
        decision_vals.push(0);
        decision_vals.reverse();
        dbg!(&decision_vals);

        for i in 0..8160 {
            println!("testing value {}", i);
            let (decision_idx, _) = decision_vals
                .windows(2)
                .enumerate()
                .find(|x| i >= x.1[0] && i < x.1[1])
                .unwrap_or((127, &[]));
            dbg!(decision_idx);

            let mu_0_pos = f32_to_ulaw(i as f32 / 8192.0);
            let mu_del_pos = f32_to_ulaw(i as f32 / 8192.0 + 1.0 / 16384.0);
            dbg!(mu_0_pos);
            assert_eq!(mu_0_pos ^ 0xff, decision_idx as u8);
            assert_eq!(mu_del_pos ^ 0xff, decision_idx as u8);

            // fixme test negatives
        }
    }

    #[test]
    #[ignore = "manually tested"]
    fn ansam_test() {
        let mut dbg_ansam_f = File::create("test_ansam.ulaw").unwrap();
        let mut ansam = AnsAmGen::new();
        let mut buf = [0u8; 8000 * 6];
        ansam.run(&mut buf);
        dbg_ansam_f.write_all(&buf).unwrap();
    }
}
