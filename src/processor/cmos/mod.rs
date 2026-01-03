use crate::memory::address::AddressMode;
use crate::processor::instructions::Instruction;
use crate::memory::Memory;
use crate::processor::{Register16, Register8};
use crate::processor::status::Status;

mod instructions;

struct CmosProcessor<'m, M: Memory> {
    program_counter: Register16,
    accumulator: Register8,
    x: Register8,
    y: Register8,
    status: Status,
    stack_pointer: Register8,
    memory: &'m M,
    cycles: u64,
}

impl<'m, M: Memory> CmosProcessor<'m, M> {
    fn with_memory(memory: &'m mut M) -> Self {
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
        let metrics = match instruction {
            Instruction::ADC => match address_mode {
                AddressMode::Immediate(value) => self.adc_immediate(value),
                AddressMode::ZeroPage(zp_address) => self.adc_zeropage(zp_address),
                AddressMode::ZeroPageX(zp_address) => self.adc_zeropage_x(zp_address),
                AddressMode::Absolute(address) => self.adc_absolute(address),
                AddressMode::AbsoluteX(address) => self.adc_absolute_x(address),
                AddressMode::AbsoluteY(address) => self.adc_absolute_y(address),
                AddressMode::PreIndexedIndirectX(zp_address) => {
                    self.adc_preindexed_indirect_x(zp_address)
                }
                AddressMode::PostIndexedIndirectY(zp_address) => {
                    self.adc_postindexed_indirect_y(zp_address)
                }
                _ => self.address_mode_not_defined(instruction, address_mode),
            },
            Instruction::AND => match address_mode {
                AddressMode::Immediate(value) => self.and_immediate(value),
                AddressMode::ZeroPage(_) => unimplemented!(),
                AddressMode::ZeroPageX(_) => unimplemented!(),
                AddressMode::Absolute(_) => unimplemented!(),
                AddressMode::AbsoluteX(_) => unimplemented!(),
                AddressMode::AbsoluteY(_) => unimplemented!(),
                AddressMode::PreIndexedIndirectX(_) => unimplemented!(),
                AddressMode::PostIndexedIndirectY(_) => unimplemented!(),
                _ => self.address_mode_not_defined(instruction, address_mode),

            },
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
        };

        self.program_counter += metrics.bytes as u16;
        self.cycles += metrics.cycles as u64;
    }

    fn address_mode_not_defined(
        &self,
        instruction: &Instruction,
        address_mode: &AddressMode,
    ) -> crate::processor::ExecutionMetrics {
        panic!(
            "Address mode {:?} not defined for instruction {:?}",
            address_mode, instruction
        );
    }
}
