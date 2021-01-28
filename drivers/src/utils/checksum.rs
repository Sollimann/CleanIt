/// checksum calculation for roomba serial stream
pub struct Checksum {
    current: u16,
}

impl Checksum {
    /// create a new `Checksum`
    pub fn new() -> Checksum {
        Checksum { current: 0 }
    }

    /// reset the calculation of `Checksum`
    pub fn reset(&mut self) {
        self.current = 0;
    }

    /// push data into the `Checksum`
    pub fn push(&mut self, data: u8) {
        self.current += data as u16;
    }

    /// push slice into the `Checksum`
    pub fn push_slice(&mut self, data: &[u8]) {
        for i in 0..data.len() {
            self.current += data[i] as u16;
        }
    }

    /// output the calculated checksum
    pub fn calculate(&self) -> u16 {
        self.current as u16
    }

    /// “& 0xFF” effectively masks the variable so it leaves only the
    /// value in the last 8 bits, and ignores all the rest of the bits.
    pub fn calculate_low_byte_sum(&self) -> u16 {
        self.current & 0xFF
    }
}
