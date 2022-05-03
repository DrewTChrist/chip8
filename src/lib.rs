#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod keypad;
pub mod roms;
pub mod fonts;

use embedded_graphics::{
    draw_target::DrawTarget, geometry::Point, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use keypad::KeyPad;
use rand::RngCore;

const NUM_REGISTERS: usize = 16;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0xFFF;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_WIDTH: usize = 64;

/// These bytes should be treated as half bytes
pub type Nibble = u8;
/// Two byte opcode
pub type Opcode = (u8, u8);
/// Opcode broken into four nibbles
pub type OpcodeDecoded = (Nibble, Nibble, Nibble, Nibble);

/// Combines the last three
/// nibbles of an opcode into a u16
fn nnn(opcode: OpcodeDecoded) -> u16 {
    let mut nnn: u16 = 0;
    nnn |= opcode.1 as u16;
    nnn <<= 4;
    nnn |= opcode.2 as u16;
    nnn <<= 4;
    nnn |= opcode.3 as u16;
    nnn
}

/// Combines the third and fourth
/// nibbles of an opcode into a single byte
fn nn(opcode: OpcodeDecoded) -> u8 {
    (opcode.2 << 4) | opcode.3
}

/// A no_std Chip8 implementation
///
/// #### Use this with your microcontroller:
/// You need:
/// * A microcontroller capable of generating random numbers
/// * A display with a driver that implements the OriginDimensions and DrawTarget traits from embedded_graphics
///     * ie st7735
/// * Enough free pins to create a button matrix (8 pins) and your display
///
/// #### Examples:
/// <https://github.com/drewtchrist/chip8-pico>
///
/// #### Timing:
/// Timing should be handled by the peripherals of
/// your hardware. This Chip8 implementation makes no attempts to manage
/// the speed of itself.
pub struct Chip8<D, O, I, R>
where
    D: DrawTarget,
    O: OutputPin,
    I: InputPin,
    R: RngCore,
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
    pixels: [[bool; CHIP8_HEIGHT]; CHIP8_WIDTH],
    rng: R,
    debug: bool,
}

impl<D, O, I, R> Chip8<D, O, I, R>
where
    D: OriginDimensions + DrawTarget<Color = Rgb565>,
    O: OutputPin,
    I: InputPin,
    R: RngCore,
{
    pub fn new<E>(display: D, keypad: KeyPad<O, I>, rng: R, debug: bool) -> Self
    where
        D: OriginDimensions + DrawTarget<Color = Rgb565>,
        O: OutputPin<Error = E>,
        I: InputPin<Error = E>,
        R: RngCore,
    {
        let mut s = Self {
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
            pixels: [[false; CHIP8_HEIGHT]; CHIP8_WIDTH],
            rng,
            debug,
        };
        s._00e0();
        s
    }

    /// Returns the current opcode
    ///
    /// Note that the `tick` method will update
    /// the program counter so this will return something
    /// different depending on if it is called before or after `tick`
    pub fn get_current_op(&self) -> OpcodeDecoded {
        self.decode(self.fetch_opcode())
    }

    /// Returns a slice of the program memory
    pub fn get_program_memory(&self) -> &[u8] {
        &self.memory[PROGRAM_START..PROGRAM_END]
    }

    /// Returns the value of PC or program counter
    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    /// Returns the value in the I register
    pub fn get_index(&self) -> u16 {
        self.index
    }

    /// Returns the stack
    pub fn get_stack(&self) -> [u16; STACK_SIZE] {
        self.stack
    }

    /// Returns the data registers
    pub fn get_registers(&self) -> [u8; NUM_REGISTERS] {
        self.registers
    }

    /// Returns the display pixel grid
    pub fn get_pixels(&self) -> [[bool; CHIP8_HEIGHT]; CHIP8_WIDTH] {
        self.pixels
    }

    pub fn load_font<const S: usize>(&mut self, font: [u8; S]) {
        let mut current = 0x50;
        for byte in font {
            self.memory[current] = byte;
            current += 1;
        }
    }

    /// Copies a chip8 program into memory
    pub fn load_program<const S: usize>(&mut self, program: [u16; S]) {
        let mut current = PROGRAM_START;
        for op in program {
            for byte in op.to_be_bytes() {
                self.memory[current] = byte;
                current += 1;
            }
        }
    }

    /// Resets the chip8 interpreter
    /// by clearing all memory and registers
    pub fn reset(&mut self) {
        self.memory = [0; RAM_SIZE];
        self.program_counter = PROGRAM_START as u16;
        self.stack = [0; STACK_SIZE];
        self.stack_pointer = 0;
        self.registers = [0; NUM_REGISTERS];
        self.index = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.pixels = [[false; CHIP8_HEIGHT]; CHIP8_WIDTH];
    }

    /// This should be called within a loop
    /// in the main function of the hardware
    ///
    /// Note there is no time management here
    /// so this Chip8 will run very fast without a delay
    pub fn tick(&mut self) {
        let opcode = self.fetch_opcode();
        let opcode_decoded = self.decode(opcode);
        self.execute(opcode_decoded);
    }

    fn fetch_opcode(&self) -> Opcode {
        let opcode: Opcode = (
            self.memory[self.program_counter as usize],
            self.memory[(self.program_counter + 1) as usize],
        );
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
        let mut pc_increment: u16 = 2;
        let mut update_pc: bool = true;
        let mut skip_instruction: bool = false;
        match opcode.0 {
            0x0 => {
                if opcode.3 == 0x0 {
                    self._00e0();
                } else if opcode.3 == 0xe {
                    self._00ee();
                }
            }
            0x1 => {
                self._1nnn(nnn(opcode));
                update_pc = false;
            }
            0x2 => {
                self._2nnn(nnn(opcode));
                update_pc = false;
            }
            0x3 => {
                skip_instruction = self._3xnn(opcode.0, nn(opcode));
            }
            0x4 => {
                skip_instruction = self._4xnn(opcode.0, nn(opcode));
            }
            0x5 => {
                skip_instruction = self._5xy0(opcode.0, opcode.1);
            }
            0x6 => {
                self._6xnn(opcode.1, nn(opcode));
            }
            0x7 => {
                self._7xnn(opcode.1, nn(opcode));
            }
            0x8 => match opcode.3 {
                0x0 => {
                    self._8xy0(opcode.1, opcode.2);
                }
                0x1 => {
                    self._8xy1(opcode.1, opcode.2);
                }
                0x2 => {
                    self._8xy2(opcode.1, opcode.2);
                }
                0x3 => {
                    self._8xy3(opcode.1, opcode.2);
                }
                0x4 => {
                    self._8xy4(opcode.1, opcode.2);
                }
                0x5 => {
                    self._8xy5(opcode.1, opcode.2);
                }
                0x6 => {
                    self._8xy6(opcode.1, opcode.2);
                }
                0x7 => {
                    self._8xy7(opcode.1, opcode.2);
                }
                0xe => {
                    self._8xye(opcode.1, opcode.2);
                }
                _ => {}
            },
            0x9 => {
                skip_instruction = self._5xy0(opcode.0, opcode.1);
            }
            0xa => {
                self._annn(nnn(opcode));
            }
            0xb => {
                self._bnnn(nnn(opcode));
                update_pc = false;
            }
            0xc => {
                self._cxnn(opcode.1, nn(opcode));
            }
            0xd => {
                self._dxyn(opcode.1, opcode.2, opcode.3);
            }
            0xe => match opcode.2 {
                0x9 => {
                    self._ex9e(opcode.1);
                }
                0xa => {
                    self._exa1(opcode.1);
                }
                _ => {}
            },
            0xf => match opcode.2 {
                0x0 => match opcode.3 {
                    0x7 => {
                        self._fx07(opcode.1);
                    }
                    0xa => {
                        self._fx0a(opcode.1);
                    }
                    _ => {}
                },
                0x1 => match opcode.3 {
                    0x5 => {
                        self._fx15(opcode.1);
                    }
                    0x8 => {
                        self._fx18(opcode.1);
                    }
                    0xe => {
                        self._fx1e(opcode.1);
                    }
                    _ => {}
                },
                0x2 => {
                    self._fx29(opcode.1);
                }
                0x3 => {
                    self._fx33(opcode.1);
                }
                0x5 => {
                    self._fx55(opcode.1);
                }
                0x6 => {
                    self._fx65(opcode.1);
                }
                _ => {}
            },
            _ => {}
        }
        if skip_instruction {
            pc_increment += 2;
        }
        if update_pc {
            self.program_counter += pc_increment;
        }
    }

    /// 0nnn
    fn _0nnn(&self, nnn: u8) {}

    /// 00e0 Clear screen
    fn _00e0(&mut self) {
        let rect = &Rectangle::new(Point::new(0, 0), self.display.size());
        if self.display.fill_solid(rect, Rgb565::BLACK).is_err() {}
    }

    /// 00ee return
    fn _00ee(&mut self) {
        self.program_counter = self.stack[self.stack_pointer];
        self.stack_pointer -= 1;
    }

    /// 1nnn jump
    fn _1nnn(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    /// 2nnn
    fn _2nnn(&mut self, nnn: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = self.program_counter;
        self.program_counter = nnn;
    }

    /// 3xnn
    fn _3xnn(&self, x: Nibble, nn: u8) -> bool {
        self.registers[x as usize] == nn
    }

    /// 4xnn
    fn _4xnn(&self, x: Nibble, nn: u8) -> bool {
        self.registers[x as usize] != nn
    }

    /// 5xy0
    fn _5xy0(&self, x: Nibble, y: Nibble) -> bool {
        self.registers[x as usize] == self.registers[y as usize]
    }

    /// 6xnn Set register vx to nn
    fn _6xnn(&mut self, x: Nibble, nn: u8) {
        self.registers[x as usize] = nn;
    }

    /// 7xnn Add nn to register vx
    fn _7xnn(&mut self, x: Nibble, nn: u8) {
        if let Some(num) =  self.registers[x as usize].checked_add(nn) {
            self.registers[x as usize] = num;
        } else {
            self.registers[x as usize] = 255;
        }
    }

    /// 8xy0
    fn _8xy0(&self, x: Nibble, y: Nibble) {}

    /// 8xy1
    fn _8xy1(&self, x: Nibble, y: Nibble) {}

    /// 8xy2
    fn _8xy2(&self, x: Nibble, y: Nibble) {}

    /// 8xy3
    fn _8xy3(&self, x: Nibble, y: Nibble) {}

    /// 8xy4
    fn _8xy4(&self, x: Nibble, y: Nibble) {}

    /// 8xy5
    fn _8xy5(&self, x: Nibble, y: Nibble) {}

    /// 8xy6
    fn _8xy6(&self, x: Nibble, y: Nibble) {}

    /// 8xy7
    fn _8xy7(&self, x: Nibble, y: Nibble) {}

    /// 8xye
    fn _8xye(&self, x: Nibble, y: Nibble) {}

    /// 9xy0
    fn _9xy0(&self, x: Nibble, y: Nibble) -> bool {
        self.registers[x as usize] != self.registers[y as usize]
    }

    /// annn set index register i
    fn _annn(&mut self, nnn: u16) {
        self.index = nnn;
    }

    /// bnnn Jump with offset
    fn _bnnn(&mut self, nnn: u16) {
        self.program_counter = nnn + self.registers[0] as u16;
    }

    /// cxnn Random number
    fn _cxnn(&mut self, x: Nibble, nn: u8) {
        let rand_num: u8 = (self.rng.next_u32() >> 28) as u8;
        self.registers[x as usize] = rand_num & nn;
    }

    /// dxyn draw screen
    fn _dxyn(&mut self, x: Nibble, y: Nibble, n: Nibble) {
        let coords: (usize, usize) = (
            (self.registers[x as usize] % (CHIP8_WIDTH as u8)) as usize,
            (self.registers[y as usize] % (CHIP8_HEIGHT as u8)) as usize,
        );
        self.registers[0xf] = 0;
        for i in 0..n as usize {
            let sprite = self.memory[(self.index as usize + i)];
            for j in 0..u8::BITS as usize {
                if sprite & (1 << j) != 0 {
                    let point =
                        Point::new(((coords.0 + j) * 2) as i32, ((coords.1 + i) * 4) as i32);
                    let rect = &Rectangle::new(point, Size::new(2, 4));
                    if self.pixels[coords.0 + j][coords.1 + i] {
                        if self.display.fill_solid(rect, Rgb565::BLACK).is_err() {}
                        self.pixels[coords.0 + j][coords.1 + i] = false;
                        self.registers[0xf] = 1;
                    } else {
                        if self.display.fill_solid(rect, Rgb565::WHITE).is_err() {}
                        self.pixels[coords.0 + j][coords.1 + i] = true;
                    }
                }
            }
        }
    }

    /// ex9e
    fn _ex9e(&self, x: Nibble) {}

    /// exa1
    fn _exa1(&self, x: Nibble) {}

    /// fx07
    fn _fx07(&self, x: Nibble) {}

    /// fx0a
    fn _fx0a(&self, x: Nibble) {}

    /// fx15
    fn _fx15(&self, x: Nibble) {}

    /// fx18
    fn _fx18(&self, x: Nibble) {}

    /// fx1e
    fn _fx1e(&self, x: Nibble) {}

    /// fx29
    fn _fx29(&self, x: Nibble) {}

    /// fx33
    fn _fx33(&self, x: Nibble) {}

    /// fx55
    fn _fx55(&self, x: Nibble) {}

    /// fx65
    fn _fx65(&self, x: Nibble) {}
}
