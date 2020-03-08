use bitvec::prelude::*;

/// Represents the machine state.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct State {
    register: [u8; 32],
    status: BitVec,
}

impl State {
    /// Creates new instance.
    pub fn new() -> State {
        State {
            register: [0; 32],
            status: bitvec![0; 16],
        }
    }

    /// Gets the register value.
    pub fn get_register(&self, index: usize) -> u8 {
        if index >= 32 {
            0
        } else {
            self.register[index]
        }
    }

    /// Sets the register value.
    pub fn set_register(&mut self, index: usize, value: u8) {
        if index < 32 {
            self.register[index] = value;
        }
    }

    /// Gets the addressing register (r31:r30) value.
    pub fn get_addressing(&self) -> u16 {
        (self.register[31] << 8) as u16 | self.register[30] as u16
    }

    /// Gets the addressing register (r31:r30) value.
    pub fn set_addressing(&mut self, value: u16) {
        self.register[30] = (value & 0xff) as u8;
        self.register[31] = (value >> 8) as u8;
    }
}
