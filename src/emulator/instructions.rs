use crate::emulator::cpu::Cpu;
use crate::emulator::memory::Mem;
use crate::emulator::memory::Stack;
use crate::emulator::cpu::Status;
use crate::emulator::opcodes::OPCODES;
use crate::emulator::opcodes::Code;
use crate::emulator::addressing_modes::AddressingMode;

impl Cpu {
    fn next(&mut self) -> u8 {
        let value = self.mem_read(self.program_counter);
        self.program_counter += 1;
        value
    }

    pub fn step(&mut self) -> bool  {
        let code = OPCODES.get(&self.next()).expect("Invalid opcode");
        let (addr, bytes_used) = self.get_operand_address(&code.mode);
        self.program_counter = self.program_counter.wrapping_add(bytes_used);

        match code.code {
            Code::LDA => { /* LDA */
                let addr = addr.unwrap();
                let value = self.mem_read(addr);

                self.register_a = value;
                self.update_zero_and_negative_flag(self.register_a);
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
                self.mem_write(addr, self.register_a)
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
                
                let res = self.addition(self.mem_read(addr));
                self.register_a = res;
                self.update_zero_and_negative_flag(res);
            },
            Code::SBC => { /* SBC */
                let addr = addr.unwrap();

                let res = self.addition(self.mem_read(addr).wrapping_neg().wrapping_sub(1) as u8);
                self.register_a = res;
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
                self.register_a &= value;
                self.update_zero_and_negative_flag(self.register_a);
            },
            Code::ORA => { /* ORA */
                let addr = addr.unwrap();

                let value = self.mem_read(addr);
                self.register_a |= value;
                self.update_zero_and_negative_flag(self.register_a);
            },
            Code::EOR => { /* EOR */
                let addr = addr.unwrap();

                let value = self.mem_read(addr);
                self.register_a ^= value;
                self.update_zero_and_negative_flag(self.register_a);
            },
            Code::JMP => { /* JMP */
                let addr = addr.unwrap();
                self.program_counter = addr;
            }, 


            Code::BCS => { /* BCS */
                let addr = addr.unwrap();

                if self.status.contains(Status::CARRY) {
                    self.program_counter = addr;
                }
            },
            Code::BCC => { /* BCC */
                let addr = addr.unwrap();

                if !self.status.contains(Status::CARRY) {
                    self.program_counter = addr;
                }
            },
            Code::BEQ => { /* BEQ */
                let addr = addr.unwrap();

                if self.status.contains(Status::ZERO) {
                    self.program_counter = addr;
                }
            },
            Code::BNE => { /* BNE */
                let addr = addr.unwrap();

                if !self.status.contains(Status::ZERO) {
                    self.program_counter = addr;
                }
            },
            Code::BMI => { /* BMI */
                let addr = addr.unwrap();

                if self.status.contains(Status::NEGATIVE) {
                    self.program_counter = addr;
                }
            },
            Code::BPL => { /* BPL */
                let addr = addr.unwrap();

                if !self.status.contains(Status::NEGATIVE) {
                    self.program_counter = addr;
                }
            },
            Code::BVS => { /* BVS */
                let addr = addr.unwrap();

                if self.status.contains(Status::OVERFLOW) {
                    self.program_counter = addr;
                }
            },
            Code::BVC => { /* BVC */
                let addr = addr.unwrap();

                if !self.status.contains(Status::OVERFLOW) {
                    self.program_counter = addr;
                }
            },


            Code::CMP => { /* CMP */
                let addr = addr.unwrap();

                let value = self.mem_read(addr);
                let result = self.register_a.wrapping_sub(value);
        
                self.update_zero_and_negative_flag(result);
                self.update_carry_flag(self.register_a, value);
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
                self.status.set(Status::ZERO, (self.register_a & value) == 0);
                self.status.set(Status::NEGATIVE, value & 0b10000000 != 0);
                self.status.set(Status::OVERFLOW, value & 0b01000000 != 0);
            },


            Code::ASL => { /* ASL */
                if code.mode == AddressingMode::Accumulator {
                    let new_val = self.asl(self.register_a);
                    self.register_a = new_val;
                } else {
                    let addr = addr.unwrap();
                    let val = self.mem_read(addr);

                    let new_val = self.asl(val);
                    self.mem_write(addr, new_val);
                }
            },
            Code::LSR => { /* LSR */
                if code.mode == AddressingMode::Accumulator {
                    let val = self.lsr(self.register_a);
                    self.register_a = val;
                } else {
                    let addr = addr.unwrap();

                    let val = self.lsr(self.mem_read(addr));
                    self.mem_write(addr, val);
                }
            },
            Code::ROL => { /* ROL */
                if code.mode == AddressingMode::Accumulator {
                    let val = self.rol(self.register_a);
                    self.register_a = val;
                } else {
                    let addr = addr.unwrap();

                    let val = self.rol(self.mem_read(addr));
                    self.mem_write(addr, val);
                }
            },
            Code::ROR => { /* ROR */
                if code.mode == AddressingMode::Accumulator {
                    let val = self.ror(self.register_a);
                    self.register_a = val;
                } else {
                    let addr = addr.unwrap();

                    let val = self.ror(self.mem_read(addr));
                    self.mem_write(addr, val);
                }
            },


            Code::TAX => { /* TAX */
                self.register_x = self.register_a;
                self.update_zero_and_negative_flag(self.register_x);
            }, 
            Code::TAY => { /* TAY */
                self.register_y = self.register_a;
                self.update_zero_and_negative_flag(self.register_y);
            }, 
            Code::TXA => { /* TXA */
                self.register_a = self.register_x;
                self.update_zero_and_negative_flag(self.register_a);
            }, 
            Code::TYA => { /* TYA */
                self.register_a = self.register_y;
                self.update_zero_and_negative_flag(self.register_a);
            }, 
            Code::TSX => {
                self.register_x = self.stack_pointer;
                self.update_zero_and_negative_flag(self.register_x);
            },
            Code::TXS => { /* TXS */
                self.stack_pointer = self.register_x;
            }, 


            Code::PHA => { /* PHA */
                self.stack_push_u8(self.register_a);
            }, 
            Code::PHP => { /* PHP */
                let mut p = self.status.clone();
                p.insert(Status::BREAKONE);
                p.insert(Status::BREAKTWO);
                self.stack_push_u8(p.bits());
            }, 
            Code::PLA => { /* PLA */
                self.register_a = self.stack_pop_u8();
                self.update_zero_and_negative_flag(self.register_a);
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

            Code::BRK => { /* BRK */
                self.status.insert(Status::BREAKONE);
                return false; // Change later
            }, 

            Code::NOP => (), /* NOP */
        }

        true
    }

    fn addition(&mut self, val: u8) -> u8 {
        let mut sum = self.register_a as u16 + val as u16;
        
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
        if (res ^ self.register_a) & (res ^ val) & 0b10000000 != 0 {
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