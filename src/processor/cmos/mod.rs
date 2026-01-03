use crate::memory::address::AddressMode;
use crate::processor::Instruction;
use crate::memory::Memory;
use crate::processor::{Register16, Register8};
use crate::processor::status::Status;

mod addressing;
pub mod instructions;

pub struct CmosProcessor<'m, M: Memory> {
    pub(crate) program_counter: Register16,
    pub(crate) accumulator: Register8,
    pub(crate) x: Register8,
    pub(crate) y: Register8,
    pub(crate) status: Status,
    pub(crate) stack_pointer: Register8,
    pub(crate) memory: &'m M,
    pub(crate) cycles: u64,
}

impl<'m, M: Memory> CmosProcessor<'m, M> {
    pub fn with_memory(memory: &'m mut M) -> Self {
        Self {
            program_counter: 0,
            memory,
            x: 0,
            y: 0,
            status: Default::default(),
            accumulator: 0,
            stack_pointer: 0,
            cycles: 0,
        }
    }


    fn execute(&mut self, instruction: &Instruction, address_mode: &AddressMode) {

        let Some(execution_metrics) = instruction.execution_metrics(address_mode) else {
            panic!("Instruction does not have a definition for address mode {:?}", address_mode);
        };

        let (value, additional_cycles) = self.translate_address(address_mode);

        match instruction {
            Instruction::ADC => self.execute_adc(&value),
            Instruction::AND => self.execute_and(&value),
            Instruction::ASL => unimplemented!(),
            Instruction::BCC => unimplemented!(),
            Instruction::BCS => unimplemented!(),
            Instruction::BEQ => unimplemented!(),
            Instruction::BIT => unimplemented!(),
            Instruction::BMI => unimplemented!(),
            Instruction::BNE => unimplemented!(),
            Instruction::BPL => unimplemented!(),
            Instruction::BRK => unimplemented!(),
            Instruction::BVC => unimplemented!(),
            Instruction::BVS => unimplemented!(),
            Instruction::CLC => unimplemented!(),
            Instruction::CLD => unimplemented!(),
            Instruction::CLI => unimplemented!(),
            Instruction::CLV => unimplemented!(),
            Instruction::CMP => unimplemented!(),
            Instruction::CPX => unimplemented!(),
            Instruction::CPY => unimplemented!(),
            Instruction::DEC => unimplemented!(),
            Instruction::DEX => unimplemented!(),
            Instruction::DEY => unimplemented!(),
            Instruction::EOR => unimplemented!(),
            Instruction::INC => unimplemented!(),
            Instruction::INX => unimplemented!(),
            Instruction::INY => unimplemented!(),
            Instruction::JMP => unimplemented!(),
            Instruction::JSR => unimplemented!(),
            Instruction::LDA => unimplemented!(),
            Instruction::LDX => unimplemented!(),
            Instruction::LDY => unimplemented!(),
            Instruction::LSR => unimplemented!(),
            Instruction::NOP => unimplemented!(),
            Instruction::ORA => unimplemented!(),
            Instruction::PHA => unimplemented!(),
            Instruction::PHP => unimplemented!(),
            Instruction::PLA => unimplemented!(),
            Instruction::PLP => unimplemented!(),
            Instruction::ROL => unimplemented!(),
            Instruction::ROR => unimplemented!(),
            Instruction::RTI => unimplemented!(),
            Instruction::RTS => unimplemented!(),
            Instruction::SBC => unimplemented!(),
            Instruction::SEC => unimplemented!(),
            Instruction::SED => unimplemented!(),
            Instruction::SEI => unimplemented!(),
            Instruction::STA => unimplemented!(),
            Instruction::STX => unimplemented!(),
            Instruction::STY => unimplemented!(),
            Instruction::TAX => unimplemented!(),
            Instruction::TAY => unimplemented!(),
            Instruction::TSX => unimplemented!(),
            Instruction::TXA => unimplemented!(),
            Instruction::TXS => unimplemented!(),
            Instruction::TYA => unimplemented!(),
        }

        self.program_counter += execution_metrics.bytes as u16;
        self.cycles += (execution_metrics.cycles + additional_cycles) as u64;
    }
}
