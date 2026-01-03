use crate::memory::address::AddressMode;
use crate::processor::ExecutionMetrics;

#[allow(nonstandard_style, unused)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    ADC, // add with carry (immediate)
    AND, // and (with accumulator)
    ASL, // arithmetic shift left
    BCC, // branch on carry clear
    BCS, // branch on carry set
    BEQ, // branch on equal (zero set)
    BIT, // bit test
    BMI, // branch on minus (negative set)
    BNE, // branch on not equal (zero clear)
    BPL, // branch on plus (negative clear)
    BRK, // break / interrupt
    BVC, // branch on overflow clear
    BVS, // branch on overflow set
    CLC, // clear carry
    CLD, // clear decimal
    CLI, // clear interrupt disable
    CLV, // clear overflow
    CMP, // compare (with accumulator)
    CPX, // compare with X
    CPY, // compare with Y
    DEC, // decrement
    DEX, // decrement X
    DEY, // decrement Y
    EOR, // exclusive or (with accumulator)
    INC, // increment
    INX, // increment X
    INY, // increment Y
    JMP, // jump
    JSR, // jump subroutine
    LDA, // load accumulator
    LDX, // load X
    LDY, // load Y
    LSR, // logical shift right
    NOP, // no operation
    ORA, // or with accumulator
    PHA, // push accumulator
    PHP, // push processor status (SR)
    PLA, // pull accumulator
    PLP, // pull processor status (SR)
    ROL, // rotate left
    ROR, // rotate right
    RTI, // return from interrupt
    RTS, // return from subroutine
    SBC, // subtract with carry
    SEC, // set carry
    SED, // set decimal
    SEI, // set interrupt disable
    STA, // store accumulator
    STX, // store X
    STY, // store Y
    TAX, // transfer accumulator to X
    TAY, // transfer accumulator to Y
    TSX, // transfer stack pointer to X
    TXA, // transfer X to accumulator
    TXS, // transfer X to stack pointer
    TYA, // transfer Y to accumulator
}

impl Instruction {
    pub(crate) fn execution_metrics(&self, address_mode: &AddressMode) -> Option<ExecutionMetrics> {
        match self {
            Instruction::AND |
            Instruction::ADC => match address_mode {
                AddressMode::Immediate(_) => Some(ExecutionMetrics::new(0x69, 2, 2)),
                AddressMode::ZeroPage(_) => Some(ExecutionMetrics::new(0x65, 2, 3)),
                AddressMode::ZeroPageX(_) => Some(ExecutionMetrics::new(0x75, 2, 4)),
                AddressMode::ZeroPageY(_) => None,
                AddressMode::Absolute(_) => Some(ExecutionMetrics::new(0x6d, 3, 4)),
                AddressMode::AbsoluteX(_) => Some(ExecutionMetrics::new(0x7d, 3, 4)),
                AddressMode::AbsoluteY(_) => Some(ExecutionMetrics::new(0x79, 3, 4)),
                AddressMode::Indirect(_) => None,
                AddressMode::PreIndexedIndirectX(_) => Some(ExecutionMetrics::new(0x61, 2, 6)),
                AddressMode::PostIndexedIndirectY(_) => Some(ExecutionMetrics::new(0x71, 2, 5)),
                AddressMode::Relative(_) => None,
            }
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
    }
}
