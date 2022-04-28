#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    Drawable,
};

pub mod roms;

const NUM_REGISTERS: usize = 16;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0xFFF;

type Nibble = u8;
type Opcode = (u8, u8);
type OpcodeDecoded = (u8, Nibble, Nibble, Nibble);

pub struct Chip8<D>
where
    D: DrawTarget,
{
    display: D,
    memory: [u8; RAM_SIZE],
    stack: [u16; STACK_SIZE],
    registers: [u8; NUM_REGISTERS],
    index: u16,
    program_counter: u16,
    sp_register: u8,
    delay_timer: u8,
    sound_timer: u8,
}

impl<D: OriginDimensions + DrawTarget<Color = Rgb565>> Chip8<D> {
    pub fn new(display: D) -> Self {
        Self {
            display,
            memory: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            registers: [0; NUM_REGISTERS],
            index: 0,
            program_counter: 0,
            sp_register: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    // copy program into memory
    pub fn load_program<const S: usize>(&mut self, program: [u16; S]) {
        let mut current = PROGRAM_START;
        for op in program {
            for byte in op.to_be_bytes() {
                self.memory[current] = byte;
                current += 1;
            }
        }
    }

    /// This should be called within a loop
    /// in the main function of the hardware
    pub fn tick(&mut self) {
        let opcode = self.fetch_opcode();
        let opcode_decoded = self.decode(opcode);
        self.execute(opcode_decoded);
    }

    fn fetch_opcode(&mut self) -> Opcode {
        let opcode: Opcode = (
            self.memory[self.program_counter as usize],
            self.memory[(self.program_counter + 1) as usize],
        );
        self.program_counter += 2;
        opcode
    }
    
    fn decode(&self, opcode: Opcode) -> OpcodeDecoded {
        (
            opcode.0 >> 4,
            opcode.0.rotate_left(4) >> 4,
            opcode.1 >> 4,
            opcode.1.rotate_left(4) >> 4,
        )
    }

    fn execute(&mut self, opcode: OpcodeDecoded) {
        if let Some(op_group) = char::from_digit(opcode.0.into(), 16) {
            match op_group {
                '0' => {
                    //self._cls();
                },
                '1' => {
                    //self._jp();
                },
                '2' => {
                },
                '3' => {
                },
                '4' => {
                },
                '5' => {
                },
                '6' => {
                    //self._ld_byte();
                },
                '7' => {
                    //self._add_byte();
                },
                '8' => {
                },
                '9' => {
                },
                'a' => {
                    //self._ld_i_address();
                },
                'b' => {
                },
                'c' => {
                },
                'd' => {
                    //self._drw();
                },
                'e' => {
                },
                'f' => {
                },
                _ => {
                }
            }
        }
    }

    // 0nnn
    fn _sys(&self, addr: u8) {}

    /// 00e0 Clear screen
    fn _cls(&mut self) {
        Rectangle::new(Point::new(0, 0), self.display.size())
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(&mut self.display);
    }

    // 00ee
    fn _ret(&self) {}

    /// 1nnn jump
    fn _jp(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    // 2nnn
    fn _call(&self, addr: u8) {}

    // 3xkk
    fn _se_byte(&self, x: u8, byte: u8) {}

    // 4xkk
    fn _sne_byte(&self, x: u8, byte: u8) {}

    // 5xy0
    fn _se_register(&self, x: u8, y: u8) {}

    /// 6xkk Set register vx
    fn _ld_byte(&mut self, x: u8, byte: u8) {
        self.registers[x as usize] = byte;
    }

    /// 7xkk Add value to register vx
    fn _add_byte(&mut self, x: u8, byte: u8) {
        self.registers[x as usize] += byte;
    }

    // 8xy0
    fn _ld_register(&self, x: u8, y: u8) {}

    // 8xy1
    fn _or(&self, x: u8, y: u8) {}

    // 8xy2
    fn _and(&self, x: u8, y: u8) {}

    // 8xy3
    fn _xor(&self, x: u8, y: u8) {}

    // 8xy4
    fn _add_register(&self, x: u8, y: u8) {}

    // 8xy5
    fn _sub(&self, x: u8, y: u8) {}

    // 8xy6
    fn _shr(&self, x: u8) {}

    // 8xy7
    fn _subn(&self, x: u8, y: u8) {}

    // 8xye
    fn _shl(&self, x: u8) {}

    // 9xy0
    fn _sne_register(&self, x: u8, y: u8) {}

    /// annn set index register i
    fn _ld_i_address(&mut self, addr: u16) {
        self.index = addr;
    }

    // bnnn
    fn _jp_addr(&self, addr: u8) {}

    // cxkk
    fn _rnd(&self, x: u8, byte: u8) {}

    /// dxyn draw screen
    fn _drw(&self, x: u8, y: u8, z: u8) {}

    // ex9e
    fn _skp(&self, x: u8) {}

    // exa1
    fn _sknp(&self, x: u8) {}

    // fx07
    fn _delay_timer_to_register(&self, x: u8) {}

    // fx0a
    fn _keypress_to_register(&self, x: u8) {}

    // fx15
    fn _delay_timer_from_register(&self, x: u8) {}

    // fx18
    fn _sound_timer_from_register(&self, x: u8) {}

    // fx1e
    fn _add_i(&self, x: u8) {}

    // fx29
    fn _ld_i(&self, x: u8) {}

    // fx33
    fn _ld_b(&self, x: u8) {}

    // fx55
    fn _store_registers_at_i(&self, x: u8) {}

    // fx65
    fn _read_registers_at_i(&self, x: u8) {}
}
