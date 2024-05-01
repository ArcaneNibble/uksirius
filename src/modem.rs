// important stuff
// f_s = 8000 Hz
// 0 dBm0 = 5215/8192 signal value

use std::{collections::VecDeque, f32::consts::PI, mem};

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
    /// Frequency of the tone encoding a 0 bit (Hz)
    freq_0: f32,
    /// Frequency of the tone encoding a 1 bit (Hz)
    freq_1: f32,
    /// Window of past values for computing a cross-correlation of the input with a "0" bit sinusoid, cosine phase
    prev_corr_terms_0_cos: Vec<f32>,
    /// Window of past values for computing a cross-correlation of the input with a "0" bit sinusoid, sine phase
    prev_corr_terms_0_sin: Vec<f32>,
    /// Window of past values for computing a cross-correlation of the input with a "1" bit sinusoid, cosine phase
    prev_corr_terms_1_cos: Vec<f32>,
    /// Window of past values for computing a cross-correlation of the input with a "1" bit sinusoid, sine phase
    prev_corr_terms_1_sin: Vec<f32>,
    /// Resulting computed cross-correlation of input with "0" sinusoid, cosine phase
    corr_0_cos: f32,
    /// Resulting computed cross-correlation of input with "0" sinusoid, sine phase
    corr_0_sin: f32,
    /// Resulting computed cross-correlation of input with "1" sinusoid, cosine phase
    corr_1_cos: f32,
    /// Resulting computed cross-correlation of input with "1" sinusoid, sine phase
    corr_1_sin: f32,
    /// Phase accumulator, [0-8000)
    phase: u32,
    /// Write offset into past values arrays
    prev_wptr: usize,
    /// State machine for decoding start/stop bits
    uart_decode: UartDecoder,
}
impl FskDemod {
    pub fn new(baud: f32, freq_0: f32, freq_1: f32) -> Self {
        // Always look at a sliding (rotating) window containing enough samples for a full symbol
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

        // convert to linear
        let inp_lin = ulaw_to_f32(inp_u);

        // generate tone sinusoids
        let (sin_0, cos_0) = (2.0 * PI * self.freq_0 / 8000.0 * self.phase as f32).sin_cos();
        let (sin_1, cos_1) = (2.0 * PI * self.freq_1 / 8000.0 * self.phase as f32).sin_cos();

        // remove the amount that is about to roll out of the window
        self.corr_0_cos -= self.prev_corr_terms_0_cos[self.prev_wptr];
        self.corr_0_sin -= self.prev_corr_terms_0_sin[self.prev_wptr];
        self.corr_1_cos -= self.prev_corr_terms_1_cos[self.prev_wptr];
        self.corr_1_sin -= self.prev_corr_terms_1_sin[self.prev_wptr];
        // correlation at this sample
        let corr_0_cos_term = inp_lin * cos_0;
        let corr_0_sin_term = inp_lin * sin_0;
        let corr_1_cos_term = inp_lin * cos_1;
        let corr_1_sin_term = inp_lin * sin_1;
        // accumulate this back into the total
        self.corr_0_cos += corr_0_cos_term;
        self.corr_0_sin += corr_0_sin_term;
        self.corr_1_cos += corr_1_cos_term;
        self.corr_1_sin += corr_1_sin_term;

        // update states
        self.phase = (self.phase + 1) % 8000;
        self.prev_corr_terms_0_cos[self.prev_wptr] = corr_0_cos_term;
        self.prev_corr_terms_0_sin[self.prev_wptr] = corr_0_sin_term;
        self.prev_corr_terms_1_cos[self.prev_wptr] = corr_1_cos_term;
        self.prev_corr_terms_1_sin[self.prev_wptr] = corr_1_sin_term;
        self.prev_wptr = (self.prev_wptr + 1) % self.prev_corr_terms_0_cos.len();

        // compute magnitude/energy of correlation
        let corr_0 = (self.corr_0_cos * self.corr_0_cos + self.corr_0_sin * self.corr_0_sin).sqrt()
            / self.prev_corr_terms_0_cos.len() as f32;
        let corr_1 = (self.corr_1_cos * self.corr_1_cos + self.corr_1_sin * self.corr_1_sin).sqrt()
            / self.prev_corr_terms_1_cos.len() as f32;

        // output 0/1 with a hard decision, but only if there is enough energy at all
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
    /// FSM state
    state: UartFSM,
    /// Baud rate (symbols / second)
    baud: f32,
    /// Stashing the previously processed bit (to compare against)
    prev_bit: i8,
    /// Timer for deciding when to sample the input (unit: samples @ 8 kHz)
    timer: u32,
    /// Bit currently being sampled
    inprogress_bit: i8,
    /// Byte currently being decoded
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
            // want to sample all the bits between 40% and 60% of timing window, make sure they all agree
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
                    UartFSM::WaitStartTransition => {
                        // ignore, bit error at this exact sample
                    }
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
pub struct FskEncoder {
    /// Baud rate (symbols / second)
    baud: f32,
    /// Frequency of the tone encoding a 0 bit (Hz)
    freq_0: f32,
    /// Frequency of the tone encoding a 1 bit (Hz)
    freq_1: f32,
    /// Currently accumulated sinusoid phase (radians)
    phase: f32,
    /// Current sample
    sample_num: u64,
    /// Current symbol
    sym_num: u64,
    /// Bytes/symbols to send
    bytes: VecDeque<u16>,
    /// Current byte/symbol being sent
    cur_byte: u16,
    /// Current bit index being sent
    cur_byte_biti: i8,
    /// Current bit value being sent
    working_bit: bool,
}
impl FskEncoder {
    const AMPLITUDE: f32 = 1165.0 / 8192.0;

    pub fn new(baud: f32, freq_0: f32, freq_1: f32) -> Self {
        Self {
            baud,
            freq_0,
            freq_1,
            phase: 0.0,
            sample_num: 0,
            sym_num: 0,
            bytes: VecDeque::new(),
            cur_byte: 0,
            cur_byte_biti: -1,
            working_bit: true,
        }
    }
    pub fn add_bytes(&mut self, bs: &[u8]) {
        for b in bs {
            self.bytes.push_back((u8::reverse_bits(*b) as u16) << 1 | 1);
        }
    }
    pub fn add_specials(&mut self, syms: &[u16]) {
        for sym in syms {
            self.bytes.push_back(sym & 0x3FF);
        }
    }
    pub fn run(&mut self, out: &mut [u8]) -> bool {
        let samples_per_symbol = 8000.0 / self.baud;
        let mut needs_more_data = self.bytes.len() == 0;
        for i in 0..out.len() {
            if self.sample_num >= ((self.sym_num + 1) as f32 * samples_per_symbol).floor() as u64 {
                // println!("new bit time! {}", self.sample_num);
                self.sym_num += 1;

                let bit;
                if self.cur_byte_biti == -1 {
                    if self.bytes.len() == 0 {
                        // idle
                        // println!("idle!");
                        bit = true;
                    } else {
                        self.cur_byte = self.bytes.pop_front().unwrap();
                        // println!("new byte! {:010b}", self.cur_byte);
                        if self.bytes.len() == 0 {
                            // println!("emptied buf!");
                            needs_more_data = true;
                        }
                        bit = self.cur_byte & (1 << 9) != 0;
                        self.cur_byte_biti = 8;
                    }
                } else {
                    bit = self.cur_byte & (1 << self.cur_byte_biti) != 0;
                    self.cur_byte_biti -= 1;
                }
                self.working_bit = bit;
            }

            // println!("bit {}", self.working_bit);
            let freq = if self.working_bit {
                self.freq_1
            } else {
                self.freq_0
            };
            // synthesize sinusoid, with this particular form
            let sinusoid = Self::AMPLITUDE * (2.0 * PI * self.phase).sin();
            let wave_u = f32_to_ulaw(sinusoid);
            out[i] = wave_u;
            // doing the computation this way ensures that there is no sharp discontinuity
            // when switching between 0 and 1 bits
            self.phase += freq / 8000.0;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
            self.sample_num += 1;
        }
        needs_more_data
    }
}

#[derive(Debug)]
pub struct AnsAmGen {
    sample_num: u32,
}
impl AnsAmGen {
    const AMPLITUDE: f32 = 1000.0 / 8192.0;
    const ANSAM_FREQ: f32 = 2100.0; // 2100 Hz carrier
    const MOD_FREQ: f32 = 15.0; // 15 Hz envelope modulation
    const PHASE_REVERSAL_SAMPLES: u32 = 3600; // 450 ms
    const TIME_LIMIT: u32 = 5 * 8000; // 5 sec

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
    SendingV8JM,
    V8Silence,
    V21Data,
    V23Data,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum V8FSM {
    WaitCM0,
    GettingCM0,
    GettingCM1,
}

#[derive(Debug)]
pub struct ModemState {
    // xxx
    timestamp: u64,
    fsm: ModemFSM,
    ansam: AnsAmGen,
    v21thing: FskDemod,
    v8_cm_buf0: Vec<u8>,
    v8_cm_buf1: Vec<u8>,
    v8_state: V8FSM,
    v8_jm: Vec<u8>,
    v21modthing: FskEncoder,
    v8_cj_zeros_count: u8,
    v8_waited_samples: u32,

    v21_another_demod: FskDemod,
    v21_another_mod: FskEncoder,

    v23_demod: FskDemod,
    v23_mod: FskEncoder,
}
impl ModemState {
    pub fn new() -> Self {
        Self {
            timestamp: 0,
            fsm: ModemFSM::AnswerWait,
            ansam: AnsAmGen::new(),
            v21thing: FskDemod::new(300.0, 1180.0, 980.0),
            v8_cm_buf0: Vec::new(),
            v8_cm_buf1: Vec::new(),
            v8_state: V8FSM::WaitCM0,
            v8_jm: Vec::new(),
            v21modthing: FskEncoder::new(300.0, 1850.0, 1650.0),
            v8_cj_zeros_count: 0,
            v8_waited_samples: 0,
            v21_another_demod: FskDemod::new(300.0, 1180.0, 980.0),
            v21_another_mod: FskEncoder::new(300.0, 1850.0, 1650.0),
            v23_demod: FskDemod::new(75.0, 450.0, 390.0),
            v23_mod: FskEncoder::new(1200.0, 2100.0, 1300.0),
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
                let mut switch_state = false;
                for inp_u in inp {
                    let maybe_byte = self.v21thing.process(*inp_u);
                    if let Some(v8b) = maybe_byte {
                        let v8b = u8::reverse_bits(v8b);
                        println!("V.8 get 0{:08b}1", v8b);

                        match self.v8_state {
                            V8FSM::WaitCM0 => {
                                if v8b == 0b00000111 {
                                    self.v8_cm_buf0.push(v8b);
                                    self.v8_state = V8FSM::GettingCM0;
                                }
                            }
                            V8FSM::GettingCM0 => {
                                if v8b == 0b00000111 {
                                    // 1st copy done, this is the start of the second copy
                                    println!("got one copy of CM {:x?}", self.v8_cm_buf0);
                                    self.v8_cm_buf1.push(v8b);
                                    self.v8_state = V8FSM::GettingCM1;
                                } else {
                                    self.v8_cm_buf0.push(v8b);
                                }
                            }
                            V8FSM::GettingCM1 => {
                                if v8b == 0b00000111 {
                                    // 2nd copy done, somewhere a third copy is coming in
                                    println!("got second copy of CM {:x?}", self.v8_cm_buf1);

                                    if self.v8_cm_buf0 == self.v8_cm_buf1 {
                                        println!("they match!");

                                        let mut _callf0 = None;
                                        let mut _modn0 = None;
                                        let mut _modn1 = None;
                                        let mut _modn2 = None;

                                        let mut i = 0;
                                        while i < self.v8_cm_buf0.len() {
                                            let b = self.v8_cm_buf0[i];

                                            match b & 0b11111_000 {
                                                0b10000_000 => {
                                                    println!(
                                                        "got call function: {}",
                                                        [
                                                            "TBD",
                                                            "Fax TX",
                                                            "V.18 Textphone",
                                                            "data",
                                                            "H.324 multimedia",
                                                            "Fax RX",
                                                            "T.101 Videotext",
                                                            "extension"
                                                        ]
                                                            [(b & 0b111) as usize]
                                                    );
                                                    _callf0 = Some(b);
                                                }
                                                0b10100_000 => {
                                                    _modn0 = Some(b);
                                                    println!("got modulation:");
                                                    if b & 0b100 != 0 {
                                                        println!("* PCM")
                                                    }
                                                    if b & 0b010 != 0 {
                                                        println!("* V.34 duplex")
                                                    }
                                                    if b & 0b001 != 0 {
                                                        println!("* V.34 half-duplex")
                                                    }

                                                    if i + 1 < self.v8_cm_buf0.len() {
                                                        let b = self.v8_cm_buf0[i + 1];
                                                        if b & 0b000_111_00 == 0b000_010_00 {
                                                            // modn1
                                                            _modn1 = Some(b);
                                                            // println!("modn1");
                                                            i += 1;
                                                            if b & 0b100_000_00 != 0 {
                                                                println!("* V.32")
                                                            }
                                                            if b & 0b010_000_00 != 0 {
                                                                println!("* V.22")
                                                            }
                                                            if b & 0b001_000_00 != 0 {
                                                                println!("* V.17")
                                                            }
                                                            if b & 0b000_000_10 != 0 {
                                                                println!("* V.29 half-duplex")
                                                            }
                                                            if b & 0b000_000_01 != 0 {
                                                                println!("* V.27ter")
                                                            }
                                                        }
                                                        if i + 1 < self.v8_cm_buf0.len() {
                                                            let b = self.v8_cm_buf0[i + 1];
                                                            if b & 0b000_111_00 == 0b000_010_00 {
                                                                // modn2
                                                                _modn2 = Some(b);
                                                                // println!("modn2");
                                                                i += 1;
                                                                if b & 0b100_000_00 != 0 {
                                                                    println!("* V.26ter")
                                                                }
                                                                if b & 0b010_000_00 != 0 {
                                                                    println!("* V.26bis")
                                                                }
                                                                if b & 0b001_000_00 != 0 {
                                                                    println!("* V.23 duplex")
                                                                }
                                                                if b & 0b000_000_10 != 0 {
                                                                    println!("* V.23 half-duplex")
                                                                }
                                                                if b & 0b000_000_01 != 0 {
                                                                    println!("* V.21")
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                0b01010_000 => {
                                                    if b & 0b111 == 0b100 {
                                                        println!("Protocols: V.42 LAPM")
                                                    } else if b & 0b111 == 0b111 {
                                                        println!("Protocols: extension")
                                                    } else {
                                                        println!(
                                                            "Protocols: UNKNOWN {:03b}",
                                                            b & 0b111
                                                        );
                                                    }
                                                }
                                                0b10110_000 => {
                                                    println!("got pstn access:");
                                                    if b & 0b100 != 0 {
                                                        println!("* caller DCE is cellular")
                                                    }
                                                    if b & 0b010 != 0 {
                                                        println!("* answer DCE is cellular")
                                                    }
                                                    if b & 0b001 != 0 {
                                                        println!("* DCE is on a digital network connection")
                                                    }
                                                }
                                                0b11100_000 => {
                                                    println!("got pcm availability:");
                                                    if b & 0b100 != 0 {
                                                        println!("* V.90 analog")
                                                    }
                                                    if b & 0b010 != 0 {
                                                        println!("* V.90 digital")
                                                    }
                                                    if b & 0b001 != 0 {
                                                        println!("* V.91")
                                                    }
                                                }
                                                _ => {
                                                    println!("don't know what this is {:08b}", b);
                                                }
                                            }

                                            i += 1;
                                        }

                                        switch_state = true;
                                    } else {
                                        println!("they DON'T MATCH!");
                                        mem::swap(&mut self.v8_cm_buf0, &mut self.v8_cm_buf1);
                                        self.v8_cm_buf1.clear();
                                        self.v8_cm_buf1.push(v8b);
                                    }
                                } else {
                                    self.v8_cm_buf1.push(v8b);
                                }
                            }
                        }
                    }
                }
                self.timestamp += inp.len() as u64;
                if switch_state {
                    println!("starting to send JM");
                    self.v8_jm = [0xe0, 0xc1, 0x05, 0x10, 0x94].to_vec();
                    self.v21modthing.add_specials(&[0x3ff]);
                    self.v21modthing.add_bytes(&self.v8_jm);
                    let _needs_more = self.v21modthing.run(outp);
                    // xxx loop around if very long packets?
                    self.fsm = ModemFSM::SendingV8JM;
                } else {
                    let timeout = self.ansam.run(outp);
                    if timeout {
                        println!("V.8 timed out!");
                        // self.fsm = ModemFSM::V21Data;
                        self.fsm = ModemFSM::V23Data;
                    }
                }
            }
            ModemFSM::SendingV8JM => {
                let mut switch_state = false;
                // detect CJ
                for inp_u in inp {
                    let maybe_byte = self.v21thing.process(*inp_u);
                    if let Some(v8b) = maybe_byte {
                        let v8b = u8::reverse_bits(v8b);
                        println!("V.8 get 0{:08b}1", v8b);
                        if v8b != 0 {
                            self.v8_cj_zeros_count = 0;
                        } else {
                            self.v8_cj_zeros_count += 1;
                            if self.v8_cj_zeros_count == 3 {
                                println!("V.8 got CJ");
                                switch_state = true;
                            }
                        }
                    }
                }

                self.timestamp += inp.len() as u64;
                if switch_state {
                    // xxx switch state
                    self.v8_waited_samples = outp.len() as u32;
                    println!(
                        "need to wait 75 ms 600 samples, already have {} ms {} samples",
                        1.0 / 8000.0 * self.v8_waited_samples as f32,
                        self.v8_waited_samples
                    );
                    outp.fill(ULAW_0);
                    self.fsm = ModemFSM::V8Silence;
                } else {
                    let needs_more = self.v21modthing.run(outp);
                    // xxx loop around multiple times if very long packets?
                    if needs_more {
                        self.v21modthing.add_specials(&[0x3ff]);
                        self.v21modthing.add_bytes(&self.v8_jm);
                    }
                }
            }
            ModemFSM::V8Silence => {
                outp.fill(ULAW_0);
                if self.v8_waited_samples + outp.len() as u32 >= 600 {
                    let start_at = 600 - self.v8_waited_samples;
                    println!("done waiting, start offset {} of {}", start_at, outp.len());
                    // xxx whatever, waste some time here
                    self.v21_another_mod.run(&mut outp[start_at as usize..]);
                    self.fsm = ModemFSM::V21Data;
                }
                self.v8_waited_samples += outp.len() as u32;
            }
            ModemFSM::V21Data => {
                for inp_u in inp {
                    let maybe_byte = self.v21_another_demod.process(*inp_u);
                    if let Some(b) = maybe_byte {
                        println!("got 0x{:02X}", b);
                        if b.is_ascii_alphanumeric() {
                            self.v21_another_mod.add_bytes(&[b + 1]);
                        } else {
                            self.v21_another_mod.add_bytes(&[b]);
                        }
                    }
                }
                self.v21_another_mod.run(outp);
            }
            ModemFSM::V23Data => {
                for inp_u in inp {
                    let maybe_byte = self.v23_demod.process(*inp_u);
                    if let Some(b) = maybe_byte {
                        println!("got 0x{:02X}", b);
                        if b.is_ascii_alphanumeric() {
                            self.v23_mod.add_bytes(&[b + 1]);
                        } else {
                            self.v23_mod.add_bytes(&[b]);
                        }
                    }
                }
                self.v23_mod.run(outp);
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
