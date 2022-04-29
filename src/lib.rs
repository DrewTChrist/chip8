#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod keypad;
pub mod roms;

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    Drawable,
};
use embedded_hal::digital::v2::{InputPin, OutputPin};

use keypad::KeyPad;

const NUM_REGISTERS: usize = 16;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0xFFF;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_WIDTH: usize = 64;

type Nibble = u8;
type Opcode = (u8, u8);
type OpcodeDecoded = (char, Nibble, Nibble, Nibble);

pub struct Chip8<D, O, I>
where
    D: DrawTarget,
    O: OutputPin,
    I: InputPin,
{
    display: D,
    keypad: KeyPad<O, I>,
    memory: [u8; RAM_SIZE],
    stack: [u16; STACK_SIZE],
    registers: [u8; NUM_REGISTERS],
    index: u16,
    program_counter: u16,
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    pixels: [[bool; CHIP8_WIDTH]; CHIP8_HEIGHT],
}

impl<D, O, I> Chip8<D, O, I>
where
    D: OriginDimensions + DrawTarget<Color = Rgb565>,
    O: OutputPin,
    I: InputPin,
{
    pub fn new<E>(display: D, keypad: KeyPad<O, I>) -> Self
    where
        D: OriginDimensions + DrawTarget<Color = Rgb565>,
        O: OutputPin<Error = E>,
        I: InputPin<Error = E>,
    {
        Self {
            display,
            keypad,
            memory: [0; RAM_SIZE],
            program_counter: PROGRAM_START as u16,
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            registers: [0; NUM_REGISTERS],
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            pixels: [[false; CHIP8_WIDTH]; CHIP8_HEIGHT],
        }
    }

    pub fn get_memory(&self) -> &[u8] {
        &self.memory[PROGRAM_START..PROGRAM_END]
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn get_index(&self) -> u16 {
        self.index
    }

    pub fn get_stack(&self) -> [u16; STACK_SIZE] {
        self.stack
    }

    pub fn get_registers(&self) -> [u8; NUM_REGISTERS] {
        self.registers
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
    pub fn tick(&mut self) -> OpcodeDecoded {
        let opcode = self.fetch_opcode();
        let opcode_decoded = self.decode(opcode);
        self.execute(opcode_decoded);
        opcode_decoded
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
            char::from_digit((opcode.0 >> 4).into(), 16).unwrap(),
            opcode.0.rotate_left(4) >> 4,
            opcode.1 >> 4,
            opcode.1.rotate_left(4) >> 4,
        )
    }

    fn execute(&mut self, opcode: OpcodeDecoded) {
        match opcode.0 {
            '0' => {
                if opcode.3 == 0x0 {
                    self._cls();
                } else if opcode.3 == 0xe {
                    self._ret();
                }
            },
            '1' => {
                self.program_counter -= 2;
                let mut nnn: u16 = 0;
                nnn |= opcode.1 as u16;
                nnn <<= 4;
                nnn |= opcode.2 as u16;
                nnn <<= 4;
                nnn |= opcode.3 as u16;
                self._jp(nnn);
            },
            '2' => {
                self.program_counter -= 2;
                let mut nnn: u16 = 0;
                nnn |= opcode.1 as u16;
                nnn <<= 4;
                nnn |= opcode.2 as u16;
                nnn <<= 4;
                nnn |= opcode.3 as u16;
                self._call(nnn);
            },
            '3' => {},
            '4' => {},
            '5' => {},
            '6' => {
                self._ld_byte(opcode.1, (opcode.2 << 4) | opcode.3);
            },
            '7' => {
                self._add_byte(opcode.1, (opcode.2 << 4) | opcode.3);
            },
            '8' => {},
            '9' => {},
            'a' => {
                let mut nnn: u16 = 0;
                nnn |= opcode.1 as u16;
                nnn <<= 4;
                nnn |= opcode.2 as u16;
                nnn <<= 4;
                nnn |= opcode.3 as u16;
                self._ld_i_address(nnn);
            },
            'b' => {},
            'c' => {},
            'd' => {
                self._drw(opcode.1, opcode.2, opcode.3);
            },
            'e' => {},
            'f' => {},
            _ => {},
        }
    }

    // 0nnn
    fn _sys(&self, nnn: u8) {}

    /// 00e0 Clear screen
    fn _cls(&mut self) {
        if Rectangle::new(Point::new(0, 0), self.display.size())
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(&mut self.display)
            .is_err()
        {}
    }

    // 00ee
    fn _ret(&mut self) {
        self.program_counter = self.stack[self.stack_pointer];
        self.stack_pointer -= 1;
    }

    /// 1nnn jump
    fn _jp(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    // 2nnn
    fn _call(&mut self, nnn: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = self.program_counter;
        self.program_counter = nnn;
    }

    // 3xnn
    fn _se_byte(&self, x: u8, nn: u8) {}

    // 4xnn
    fn _sne_byte(&self, x: u8, nn: u8) {}

    // 5xy0
    fn _se_register(&self, x: u8, y: u8) {}

    /// 6xnn Set register vx
    fn _ld_byte(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn;
    }

    /// 7xnn Add value to register vx
    fn _add_byte(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] += nn;
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
    fn _ld_i_address(&mut self, nnn: u16) {
        self.index = nnn;
    }

    // bnnn
    fn _jp_addr(&self, nnn: u8) {}

    // cxnn
    fn _rnd(&self, x: u8, nn: u8) {}

    /// dxyn draw screen
    fn _drw(&mut self, mut x: u8, mut y: u8, n: u8) {
        let coords: (u8, u8) = (
            self.registers[x as usize] % 64,
            self.registers[y as usize] % 32,
        );
        self.registers[0xf] = 0;
        for i in 0..n {
            let sprite = self.memory[(self.index + i as u16) as usize].reverse_bits();
            for j in 0..u8::BITS {
                if (sprite >> j) == 1 && self.pixels[i as usize][j as usize] {
                    // turn pixel off
                    if self
                        .display
                        .fill_solid(
                            &Rectangle::new(
                                Point::new(coords.0.into(), coords.1.into()),
                                Size::new(3, 4),
                            ),
                            Rgb565::BLACK,
                        )
                        .is_err()
                    {}
                    self.pixels[i as usize][j as usize] = false;
                } else if (sprite >> j) == 1 && !self.pixels[i as usize][j as usize] {
                    // turn pixel on
                    if self
                        .display
                        .fill_solid(
                            &Rectangle::new(
                                Point::new((coords.0*3).into(), (coords.1*4).into()),
                                Size::new(3, 4),
                            ),
                            Rgb565::WHITE,
                        )
                        .is_err()
                    {}
                    self.pixels[i as usize][j as usize] = true;
                }
                x += 1;
            }
            y += 1;
        }
    }

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
