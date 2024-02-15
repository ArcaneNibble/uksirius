// important stuff
// f_s = 8000 Hz
// 0 dBm0 = 5215/8192 signal value

use std::f32::consts::PI;

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
pub struct FskDemod {
    freq_0: f32,
    freq_1: f32,
    prev_corr_terms_0_cos: Vec<f32>,
    prev_corr_terms_0_sin: Vec<f32>,
    prev_corr_terms_1_cos: Vec<f32>,
    prev_corr_terms_1_sin: Vec<f32>,
    corr_0_cos: f32,
    corr_0_sin: f32,
    corr_1_cos: f32,
    corr_1_sin: f32,
    phase: u32,
    prev_wptr: usize,
    uart_decode: UartDecoder,
}
impl FskDemod {
    pub fn new(baud: f32, freq_0: f32, freq_1: f32) -> Self {
        let baud_win_sz = (8000.0 / baud).floor() as usize;

        Self {
            freq_0,
            freq_1,
            prev_corr_terms_0_cos: vec![0.0; baud_win_sz],
            prev_corr_terms_0_sin: vec![0.0; baud_win_sz],
            prev_corr_terms_1_cos: vec![0.0; baud_win_sz],
            prev_corr_terms_1_sin: vec![0.0; baud_win_sz],
            corr_0_cos: 0.0,
            corr_0_sin: 0.0,
            corr_1_cos: 0.0,
            corr_1_sin: 0.0,
            phase: 0,
            prev_wptr: 0,
            uart_decode: UartDecoder::new(baud),
        }
    }
    pub fn process(&mut self, inp_u: u8) -> Option<u8> {
        // absolutely don't do *anything* fancy, just cross-correlation with sinusoids
        let inp_lin = ulaw_to_f32(inp_u);

        let (sin_0, cos_0) = (2.0 * PI * self.freq_0 / 8000.0 * self.phase as f32).sin_cos();
        let (sin_1, cos_1) = (2.0 * PI * self.freq_1 / 8000.0 * self.phase as f32).sin_cos();

        self.corr_0_cos -= self.prev_corr_terms_0_cos[self.prev_wptr];
        self.corr_0_sin -= self.prev_corr_terms_0_sin[self.prev_wptr];
        self.corr_1_cos -= self.prev_corr_terms_1_cos[self.prev_wptr];
        self.corr_1_sin -= self.prev_corr_terms_1_sin[self.prev_wptr];
        let corr_0_cos_term = inp_lin * cos_0;
        let corr_0_sin_term = inp_lin * sin_0;
        let corr_1_cos_term = inp_lin * cos_1;
        let corr_1_sin_term = inp_lin * sin_1;
        self.corr_0_cos += corr_0_cos_term;
        self.corr_0_sin += corr_0_sin_term;
        self.corr_1_cos += corr_1_cos_term;
        self.corr_1_sin += corr_1_sin_term;

        self.phase = (self.phase + 1) % 8000;
        self.prev_corr_terms_0_cos[self.prev_wptr] = corr_0_cos_term;
        self.prev_corr_terms_0_sin[self.prev_wptr] = corr_0_sin_term;
        self.prev_corr_terms_1_cos[self.prev_wptr] = corr_1_cos_term;
        self.prev_corr_terms_1_sin[self.prev_wptr] = corr_1_sin_term;
        self.prev_wptr = (self.prev_wptr + 1) % self.prev_corr_terms_0_cos.len();

        let corr_0 = (self.corr_0_cos * self.corr_0_cos + self.corr_0_sin * self.corr_0_sin).sqrt()
            / self.prev_corr_terms_0_cos.len() as f32;
        let corr_1 = (self.corr_1_cos * self.corr_1_cos + self.corr_1_sin * self.corr_1_sin).sqrt()
            / self.prev_corr_terms_1_cos.len() as f32;

        // -40 dBm0 is a symbol of 52.15 / 8192
        let bit = if corr_0 > corr_1 && corr_0 >= (52.15 / 8192.0) {
            0
        } else if corr_1 > corr_0 && corr_1 >= (52.15 / 8192.0) {
            1
        } else {
            -1
        };

        self.uart_decode.process(bit)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum UartFSM {
    WaitStartTransition,
    StartBit,
    DataBit(u8),
    StopBit,
}
#[derive(Debug)]
pub struct UartDecoder {
    state: UartFSM,
    baud: f32,
    prev_bit: i8,
    timer: u32,
    inprogress_bit: i8,
    inprogress_byte: u8,
}
impl UartDecoder {
    pub fn new(baud: f32) -> Self {
        Self {
            state: UartFSM::WaitStartTransition,
            baud,
            prev_bit: -1,
            timer: 0,
            inprogress_bit: -1,
            inprogress_byte: 0,
        }
    }
    pub fn process(&mut self, bit: i8) -> Option<u8> {
        let samples_per_symbol = 8000.0 / self.baud;
        let mut ret = None;

        if self.state == UartFSM::WaitStartTransition {
            if self.prev_bit == 1 && bit == 0 {
                // println!("Found start bit!");
                self.timer = 0;
                self.state = UartFSM::StartBit;
            }
        } else {
            let uart_bit_i = match self.state {
                UartFSM::WaitStartTransition => unreachable!(),
                UartFSM::StartBit => 0,
                UartFSM::DataBit(n) => n + 1,
                UartFSM::StopBit => 9,
            };
            let this_bit_start_sample_time =
                (samples_per_symbol * ((uart_bit_i as f32) + 0.4)).floor() as u32;
            let this_bit_stop_sample_time =
                (samples_per_symbol * ((uart_bit_i as f32) + 0.6)).floor() as u32;
            let this_bit_end_time =
                (samples_per_symbol * ((uart_bit_i as f32) + 1.0)).floor() as u32;

            if self.timer == this_bit_start_sample_time {
                // start sampling this bit at 40%
                self.inprogress_bit = bit;
            } else if self.timer >= this_bit_start_sample_time
                && self.timer < this_bit_stop_sample_time
            {
                if bit != self.inprogress_bit {
                    println!("bit error!");
                    self.state = UartFSM::WaitStartTransition;
                }
            }

            self.timer += 1;

            if self.timer == this_bit_stop_sample_time {
                // stop sampling this bit at 60%
                match self.state {
                    UartFSM::WaitStartTransition => unreachable!(),
                    UartFSM::StartBit => {
                        if self.inprogress_bit != 0 {
                            println!("Framing error (start bit!)");
                            self.state = UartFSM::WaitStartTransition;
                        } else {
                            self.inprogress_byte = 0;
                        }
                    }
                    UartFSM::DataBit(n) => {
                        debug_assert_ne!(self.inprogress_bit, -1);
                        self.inprogress_byte |= (self.inprogress_bit as u8) << n;
                    }
                    UartFSM::StopBit => {
                        if self.inprogress_bit != 1 {
                            println!("Framing error (stop bit!)");
                            self.state = UartFSM::WaitStartTransition;
                        }
                    }
                }
            }

            if self.state == UartFSM::StopBit && self.timer >= this_bit_stop_sample_time {
                // switch early to make sure we don't miss next start bit
                self.state = UartFSM::WaitStartTransition;
                // println!(
                //     "char done! 0x{:02X} 0b{:08b}",
                //     self.inprogress_byte, self.inprogress_byte
                // );
                ret = Some(self.inprogress_byte);
            } else {
                if self.timer >= this_bit_end_time {
                    match self.state {
                        UartFSM::WaitStartTransition | UartFSM::StopBit => unreachable!(),
                        UartFSM::StartBit => self.state = UartFSM::DataBit(0),
                        UartFSM::DataBit(n) => {
                            if n == 7 {
                                self.state = UartFSM::StopBit;
                            } else {
                                self.state = UartFSM::DataBit(n + 1);
                            }
                        }
                    }
                }
            }
        }

        self.prev_bit = bit;
        ret
    }
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
    v21thing: FskDemod,
}
impl ModemState {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            fsm: ModemFSM::AnswerWait,
            ansam: AnsAmGen::new(),
            v21thing: FskDemod::new(300.0, 1180.0, 980.0),
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
                    let maybe_byte = self.v21thing.process(*inp_u);
                    if let Some(v8b) = maybe_byte {
                        let v8b = u8::reverse_bits(v8b);
                        println!("V.8 get 0{:08b}1", v8b);
                    }
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
