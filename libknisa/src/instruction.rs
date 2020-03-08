use crate::error::DecodeError;

/// Represents an instruction.
pub enum Instruction {
    /// `0000000` No operation - Does nothing.
    Nop,

    /// `0000100` Load immediate - Loads immediate value to register.
    Ldi { destination: u8, value: u8 },

    /// `0000101` Load memory - Loads the value where the address regisger points to register.
    Ldm { destination: u8 },

    /// `0000110` Load register - Loads immediate value to register.
    Ldr { destination: u8, source: u8 },

    /// `0001000` Store to memory - Stores register value to memory.
    Stm { source: u8 },
}

impl Instruction {
    /// Decodes the instruction bits into `Instruction`.
    /// Although it receives 'u32', only lower 20 bits are available.
    pub fn decode(inst: u32) -> Result<Instruction, DecodeError> {
        if inst >= 0x00100000 {
            return Err(DecodeError::InvalidLength);
        }
        let code = ((inst >> 13) & 0x7f) as u8;
        let address = ((inst >> 8) & 0x1f) as u8;
        let data = (inst & 0xff) as u8;

        let decoded = match code {
            0 => Instruction::Nop,
            4 => Instruction::Ldi {
                destination: address,
                value: data,
            },
            5 => Instruction::Ldm {
                destination: address,
            },
            6 => Instruction::Ldr {
                destination: address,
                source: data & 0x1f,
            },
            8 => Instruction::Stm { source: address },
            _ => return Err(DecodeError::UnknownInstruction(code)),
        };

        Ok(decoded)
    }
}
