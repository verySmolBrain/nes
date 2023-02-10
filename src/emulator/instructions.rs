use crate::emulator::cpu::Cpu;
use crate::emulator::memory::Mem;
use crate::emulator::memory::Stack;
use crate::emulator::cpu::Status;
use crate::emulator::opcodes::OPCODES;
use crate::emulator::opcodes::Code;
use crate::emulator::addressing_modes::AddressingMode;

use super::interrupts::Interrupt;

impl Cpu {
    fn next(&mut self) -> u8 {
        let value = self.mem_read(self.program_counter);
        self.program_counter += 1;
        value
    }

    pub fn step(&mut self) -> bool  {
        let code = OPCODES.get(&self.next()).expect("Invalid opcode");
        let (addr, bytes_used, crossed_page) = self.get_operand_address(&code.mode);
        self.program_counter = self.program_counter.wrapping_add(bytes_used);
        
        let mut branch_taken = false;

        match code.code {
            Code::LDA => { /* LDA */
                let addr = addr.unwrap();
                let value = self.mem_read(addr);

                self.accumulator = value;
                self.update_zero_and_negative_flag(self.accumulator);
            },
            Code::LDX => { /* LDX */
                let addr = addr.unwrap();
                let value = self.mem_read(addr);

                self.register_x = value;
                self.update_zero_and_negative_flag(self.register_x);
            },
            Code::LDY => { /* LDY */
                let addr = addr.unwrap();
                let value = self.mem_read(addr);

                self.register_y = value;
                self.update_zero_and_negative_flag(self.register_y);
            },
            

            Code::STA => { /* STA */
                let addr = addr.unwrap();
                self.mem_write(addr, self.accumulator);
            },
            Code::STX => { /* STX */
                let addr = addr.unwrap();
                self.mem_write(addr, self.register_x)
            },
            Code::STY => { /* STY */
                let addr = addr.unwrap();
                self.mem_write(addr, self.register_y)
            },
            

            Code::ADC => { /* ADC */
                let addr = addr.unwrap();
                let val = self.mem_read(addr);
                
                let res = self.addition(val);
                self.accumulator = res;
                self.update_zero_and_negative_flag(res);
            },
            Code::SBC | Code::SBC_U => { /* SBC */
                let addr = addr.unwrap();
                let val = self.mem_read(addr);

                let res = self.addition(val.wrapping_neg().wrapping_sub(1) as u8);
                self.accumulator = res;
                self.update_zero_and_negative_flag(res);
            },
            

            Code::INC => { /* INC */
                let addr = addr.unwrap();

                let value = self.mem_read(addr).wrapping_add(1);
                self.mem_write(addr, value);
                self.update_zero_and_negative_flag(value);
            },
            Code::INX => { /* INX */
                self.register_x = self.register_x.wrapping_add(1);
                self.update_zero_and_negative_flag(self.register_x);
            }, 
            Code::INY => { /* INY */
                self.register_y = self.register_y.wrapping_add(1);
                self.update_zero_and_negative_flag(self.register_y);
            } 


            Code::DEC => { /* DEC */
                let addr = addr.unwrap();

                let value = self.mem_read(addr).wrapping_sub(1);
                self.mem_write(addr, value);
                self.update_zero_and_negative_flag(value);
            },
            Code::DEX => { /* DEX */
                self.register_x = self.register_x.wrapping_sub(1);
                self.update_zero_and_negative_flag(self.register_x);
            } 
            Code::DEY => { /* DEY */
                self.register_y = self.register_y.wrapping_sub(1);
                self.update_zero_and_negative_flag(self.register_y);
            }, 


            Code::AND => { /* AND */
                let addr = addr.unwrap();

                let value = self.mem_read(addr);
                self.accumulator &= value;
                self.update_zero_and_negative_flag(self.accumulator);
            },
            Code::ORA => { /* ORA */
                let addr = addr.unwrap();

                let value = self.mem_read(addr);
                self.accumulator |= value;
                self.update_zero_and_negative_flag(self.accumulator);
            },
            Code::EOR => { /* EOR */
                let addr = addr.unwrap();

                let value = self.mem_read(addr);
                self.accumulator ^= value;
                self.update_zero_and_negative_flag(self.accumulator);
            },
            Code::JMP => { /* JMP */
                let addr = addr.unwrap();
                self.program_counter = addr;
            }, 


            Code::BCS => { /* BCS */
                let addr = addr.unwrap();

                if self.status.contains(Status::CARRY) {
                    branch_taken = true;
                    self.program_counter = addr;
                }
            },
            Code::BCC => { /* BCC */
                let addr = addr.unwrap();

                if !self.status.contains(Status::CARRY) {
                    branch_taken = true;
                    self.program_counter = addr;
                }
            },
            Code::BEQ => { /* BEQ */
                let addr = addr.unwrap();

                if self.status.contains(Status::ZERO) {
                    branch_taken = true;
                    self.program_counter = addr;
                }
            },
            Code::BNE => { /* BNE */
                let addr = addr.unwrap();

                if !self.status.contains(Status::ZERO) {
                    branch_taken = true;
                    self.program_counter = addr;
                }
            },
            Code::BMI => { /* BMI */
                let addr = addr.unwrap();

                if self.status.contains(Status::NEGATIVE) {
                    branch_taken = true;
                    self.program_counter = addr;
                }
            },
            Code::BPL => { /* BPL */
                let addr = addr.unwrap();

                if !self.status.contains(Status::NEGATIVE) {
                    branch_taken = true;
                    self.program_counter = addr;
                }
            },
            Code::BVS => { /* BVS */
                let addr = addr.unwrap();

                if self.status.contains(Status::OVERFLOW) {
                    branch_taken = true;
                    self.program_counter = addr;
                }
            },
            Code::BVC => { /* BVC */
                let addr = addr.unwrap();

                if !self.status.contains(Status::OVERFLOW) {
                    branch_taken = true;
                    self.program_counter = addr;
                }
            },


            Code::CMP => { /* CMP */
                let addr = addr.unwrap();

                let value = self.mem_read(addr);
                let result = self.accumulator.wrapping_sub(value);
        
                self.update_zero_and_negative_flag(result);
                self.update_carry_flag(self.accumulator, value);
            },
            Code::CPX => { /* CPX */
                let addr = addr.unwrap();

                let value = self.mem_read(addr);
                let result = self.register_x.wrapping_sub(value);
        
                self.update_zero_and_negative_flag(result);
                self.update_carry_flag(self.register_x, value);
            },
            Code::CPY => { /* CPY */
                let addr = addr.unwrap();
                
                let value = self.mem_read(addr);
                let result = self.register_y.wrapping_sub(value);

                self.update_zero_and_negative_flag(result);
                self.update_carry_flag(self.register_y, value);
            },
            Code::BIT => { /* BIT */
                let addr = addr.unwrap();

                let value = self.mem_read(addr);
                self.status.set(Status::ZERO, (self.accumulator & value) == 0);
                self.status.set(Status::NEGATIVE, value & 0b10000000 != 0);
                self.status.set(Status::OVERFLOW, value & 0b01000000 != 0);
            },


            Code::ASL => { /* ASL */
                if code.mode == AddressingMode::Accumulator {
                    let new_val = self.asl(self.accumulator);
                    self.accumulator = new_val;
                } else {
                    let addr = addr.unwrap();
                    let val = self.mem_read(addr);

                    let new_val = self.asl(val);
                    self.mem_write(addr, new_val);
                }
            },
            Code::LSR => { /* LSR */
                if code.mode == AddressingMode::Accumulator {
                    let val = self.lsr(self.accumulator);
                    self.accumulator = val;
                } else {
                    let addr = addr.unwrap();
                    let val = self.mem_read(addr);

                    let val = self.lsr(val);
                    self.mem_write(addr, val);
                }
            },
            Code::ROL => { /* ROL */
                if code.mode == AddressingMode::Accumulator {
                    let val = self.rol(self.accumulator);
                    self.accumulator = val;
                } else {
                    let addr = addr.unwrap();
                    let val = self.mem_read(addr);

                    let val = self.rol(val);
                    self.mem_write(addr, val);
                }
            },
            Code::ROR => { /* ROR */
                if code.mode == AddressingMode::Accumulator {
                    let val = self.ror(self.accumulator);
                    self.accumulator = val;
                } else {
                    let addr = addr.unwrap();
                    let val = self.mem_read(addr);

                    let val = self.ror(val);
                    self.mem_write(addr, val);
                }
            },


            Code::TAX => { /* TAX */
                self.register_x = self.accumulator;
                self.update_zero_and_negative_flag(self.register_x);
            }, 
            Code::TAY => { /* TAY */
                self.register_y = self.accumulator;
                self.update_zero_and_negative_flag(self.register_y);
            }, 
            Code::TXA => { /* TXA */
                self.accumulator = self.register_x;
                self.update_zero_and_negative_flag(self.accumulator);
            }, 
            Code::TYA => { /* TYA */
                self.accumulator = self.register_y;
                self.update_zero_and_negative_flag(self.accumulator);
            }, 
            Code::TSX => {
                self.register_x = self.stack_pointer;
                self.update_zero_and_negative_flag(self.register_x);
            },
            Code::TXS => { /* TXS */
                self.stack_pointer = self.register_x;
            }, 


            Code::PHA => { /* PHA */
                self.stack_push_u8(self.accumulator);
            }, 
            Code::PHP => { /* PHP */
                let mut p = self.status.clone();
                p.insert(Status::BREAKONE);
                p.insert(Status::BREAKTWO);
                self.stack_push_u8(p.bits());
            }, 
            Code::PLA => { /* PLA */
                self.accumulator = self.stack_pop_u8();
                self.update_zero_and_negative_flag(self.accumulator);
            }, 
            Code::PLP => { /* PLP */
                self.status = Status::from_bits_truncate(self.stack_pop_u8());
                self.status.remove(Status::BREAKTWO);
                self.status.insert(Status::BREAKONE);
            }, 


            Code::JSR => { /* JSR */
                let addr = addr.unwrap();

                self.stack_push_u16(self.program_counter.wrapping_sub(1));
                self.program_counter = addr;
            }, 
            Code::RTS => { /* RTS */
                self.program_counter = self.stack_pop_u16().wrapping_add(1);
            }, 
            Code::RTI => { /* RTI */
                self.status = Status::from_bits_truncate(self.stack_pop_u8());
                self.program_counter = self.stack_pop_u16();
        
                self.status.remove(Status::BREAKTWO);
                self.status.insert(Status::BREAKONE);
            }, 


            Code::CLC => { /* CLC */
                self.status.remove(Status::CARRY)
            }, 
            Code::CLD => { /* CLD */
                self.status.remove(Status::DECIMAL)
            }, 
            Code::CLI => { /* CLI */
                self.status.remove(Status::INTERDIS)
            }, 
            Code::CLV => { /* CLV */
                self.status.remove(Status::OVERFLOW)
            }, 


            Code::SEC => { /* SEC */
                self.status.insert(Status::CARRY);
            }, 
            Code::SED => { /* SED */
                self.status.insert(Status::DECIMAL);
            }, 
            Code::SEI => { /* SEI */
                self.status.insert(Status::INTERDIS);
            }, 

            /* Unofficial Opcodes */
            Code::AAC_U => { /* AAC */
                let addr = addr.unwrap();

                self.accumulator &= self.mem_read(addr);
                self.update_zero_and_negative_flag(self.accumulator);

                if self.status.contains(Status::NEGATIVE) {
                    self.status.insert(Status::CARRY);
                } else {
                    self.status.remove(Status::CARRY);
                }
            },

            Code::AAX_U => { /* AAX */
                let addr = addr.unwrap();
                let val = self.accumulator & self.register_x;

                self.mem_write(addr, val);
            },

            Code::ARR_U => { /* ARR */
                let addr = addr.unwrap();

                let val = self.mem_read(addr);
                self.accumulator &= val;
                self.accumulator >>= 1;

                let bit_5_set = self.accumulator & 0b100 != 0;
                let bit_6_set = self.accumulator & 0b10 != 0;

                if bit_5_set && bit_6_set {
                    self.status.insert(Status::CARRY);
                    self.status.remove(Status::OVERFLOW);
                } else if bit_5_set && !bit_6_set {
                    self.status.remove(Status::CARRY);
                    self.status.insert(Status::OVERFLOW);
                } else if !bit_5_set && bit_6_set {
                    self.status.insert(Status::CARRY);
                    self.status.insert(Status::OVERFLOW);
                } else {
                    self.status.remove(Status::CARRY);
                    self.status.remove(Status::OVERFLOW);
                }

                self.update_zero_and_negative_flag(self.accumulator);
            },

            Code::ASR_U => { /* ASR */
                let addr = addr.unwrap();

                let val = self.mem_read(addr);
                self.accumulator &= val;
                self.accumulator = self.lsr(self.accumulator);
            },

            Code::ATX_U => { /* ATX */
                let addr = addr.unwrap();

                let val = self.mem_read(addr);
                self.accumulator &= val;
                self.register_x = self.accumulator;

                self.update_zero_and_negative_flag(self.accumulator);
            }

            Code::AXA_U => { /* AXA */
                let addr = addr.unwrap();

                let val = self.accumulator & self.register_x & 7;
                self.mem_write(addr, val);
            },

            Code::AXS_U => { /* AXS */
                let addr = addr.unwrap();
                let val = self.mem_read(addr);

                let res = self.accumulator & self.register_x;

                self.update_carry_flag(res, val);
                self.register_x = res.wrapping_sub(val);
                self.update_zero_and_negative_flag(self.register_x);
            },


            Code::DCP_U => { /* DCP */
                let addr = addr.unwrap();
                let val = self.mem_read(addr).wrapping_sub(1);

                self.mem_write(addr, val);
                self.update_carry_flag(self.accumulator, val);
                // DCP is a CMP with the result of the subtraction
                self.update_zero_and_negative_flag(self.accumulator.wrapping_sub(val));
            },

            Code::ISB_U => { /* ISC */
                let addr = addr.unwrap();

                let val = self.mem_read(addr).wrapping_add(1);
                self.mem_write(addr, val);

                let res = self.addition(val.wrapping_neg().wrapping_sub(1) as u8);
                self.accumulator = res;
                self.update_zero_and_negative_flag(self.accumulator);
            }

            Code::LAR_U => { /* LAR */
                let addr = addr.unwrap();

                let val = self.mem_read(addr);
                self.stack_pointer &= val;
                self.accumulator = self.stack_pointer;
                self.register_x = self.stack_pointer;
                self.update_zero_and_negative_flag(self.stack_pointer);
            },

            Code::LAX_U => { /* LAX */
                let addr = addr.unwrap();

                let val = self.mem_read(addr);
                self.accumulator = val;
                self.register_x = val;
                self.update_zero_and_negative_flag(val);
            },

            Code::RLA_U => { /* RLA */
                let addr = addr.unwrap();
                let val = self.mem_read(addr);

                let res = self.rol(val);
                self.mem_write(addr, res);

                self.accumulator &= res;
                self.update_zero_and_negative_flag(self.accumulator);
            },

            Code::RRA_U => { /* RRA */
                let addr = addr.unwrap();
                let val = self.mem_read(addr);

                let res = self.ror(val);
                self.mem_write(addr, res);

                let res = self.addition(res);
                self.accumulator = res;
                self.update_zero_and_negative_flag(self.accumulator);
            },

            Code::SLO_U => { /* SLO */
                let addr = addr.unwrap();
                let val = self.mem_read(addr);

                let res = self.asl(val);
                self.mem_write(addr, res);

                self.accumulator |= res;
                self.update_zero_and_negative_flag(self.accumulator);
            },

            Code::SRE_U => { /* SRE */
                let addr = addr.unwrap();
                let val = self.mem_read(addr);

                let res = self.lsr(val);
                self.mem_write(addr, res);

                self.accumulator ^= res;
                self.update_zero_and_negative_flag(self.accumulator);
            },

            Code::SXA_U => { /* SXA */
                let addr = addr.unwrap();
                let val = self.register_x & ((addr >> 8) as u8 + 1);
                self.mem_write(addr, val);
            }

            Code::SYA_U => { /* SYA */
                let addr = addr.unwrap();
                let val = self.register_y & ((addr >> 8) as u8 + 1);
                self.mem_write(addr, val);
            },

            Code::XAS_U => { /* XAS */
                let addr = addr.unwrap();
                
                self.stack_pointer = self.accumulator & self.register_x;
                let val = self.stack_pointer & ((addr >> 8) as u8 + 1);
                self.mem_write(addr, val);
            },

            Code::XAA_U => { /* XAA */
                let addr = addr.unwrap();

                let val = self.mem_read(addr);
                self.accumulator = self.register_x & val;
                self.update_zero_and_negative_flag(self.accumulator);
            },

            Code::BRK => { /* BRK */
                self.status.insert(Status::BREAKONE);
                return false; // Change later
            }, 
            
            Code::NOP_U => (), /* NOP */
            Code::KIL_U => (), /* KIL */
            Code::DOP_U => (), /* DOP */
            Code::NOP => (), /* NOP */
        }

        
        let mut cycle_inc: usize = code.cycles;
        match code.code {
            /* 
            STA, ROR, ROL, LSR, ASL, INC, DEC, DCP, ISC, RLA, RRA, SLO, SRE, SXA, SYA, XAS 
            don't care about page crossing (Due to LE) 
            */
            Code::STA | Code::ROR | Code::ROL | Code::LSR | Code::ASL | Code::INC 
            | Code::DEC | Code::DCP_U | Code::ISB_U | Code::RRA_U | Code::RLA_U 
            | Code::SLO_U | Code::SRE_U | Code::SYA_U | Code::SXA_U | Code::XAS_U => (),

            Code::BCC | Code::BCS | Code::BEQ | Code::BMI | Code::BNE 
            | Code::BPL | Code::BVC | Code::BVS => {
                if branch_taken && crossed_page {
                    cycle_inc += 2;
                } else if branch_taken {
                    cycle_inc += 1;
                }
            },
            _ => {
                if crossed_page {
                    cycle_inc += 1;
                }
            },
        }
        
        self.cycles = self.cycles.wrapping_add(cycle_inc);
        self.bus.tick(cycle_inc);

        true // Change later
    }

    fn addition(&mut self, val: u8) -> u8 {
        let mut sum = self.accumulator as u16 + val as u16;
        
        if self.status.contains(Status::CARRY) {
            sum += 1;
        }

        if sum > 0xff {
            self.status.insert(Status::CARRY);
        } else {
            self.status.remove(Status::CARRY);
        }

        let res = sum as u8; 
        // http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        if (res ^ self.accumulator) & (res ^ val) & 0b10000000 != 0 {
            self.status.insert(Status::OVERFLOW);
        } else { 
            self.status.remove(Status::OVERFLOW);
        }

        res
    }

    fn asl(&mut self, val: u8) -> u8 {
        self.status.set(Status::CARRY, val & 0b10000000 != 0);
        let new_val = val << 1;
        self.update_zero_and_negative_flag(new_val);

        new_val
    }

    fn lsr(&mut self, val: u8) -> u8 {
        self.status.set(Status::CARRY, val & 0b1 != 0);
        let new_val = val >> 1;
        self.update_zero_and_negative_flag(new_val);

        new_val
    }

    fn rol(&mut self, val: u8) -> u8 {
        let old_carry = self.status.contains(Status::CARRY) as u8;
        self.status.set(Status::CARRY, val & 0b10000000 != 0);

        let new_val = (val << 1) | old_carry;
        self.update_zero_and_negative_flag(new_val);
        new_val
    }

    fn ror(&mut self, val: u8) -> u8 {
        let old_carry = self.status.contains(Status::CARRY) as u8;
        self.status.set(Status::CARRY, val & 0b1 != 0);

        let new_val = (val >> 1) | (old_carry << 7);
        self.update_zero_and_negative_flag(new_val);
        new_val
    }
}