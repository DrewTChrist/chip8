#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

use embedded_graphics::draw_target::DrawTarget;

pub mod roms;

const NUM_REGISTERS: usize = 16;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;

const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0xFFF;

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

impl<D: DrawTarget> Chip8<D> {
    pub fn new(display: D) -> Self {
        Self {
            display: display,
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

    // clear memory, registers, screen
    pub fn _initialize(&self) {}

    // copy program into memory
    pub fn load_program(&mut self, program: [u8; STACK_SIZE]) {
        let mut current = PROGRAM_START;
        for i in 0..program.len() {
            self.memory[current] = program[i];
            current += i;
        }
    }

    // 0nnn
    fn _sys(&self, addr: u8) {}

    // 00e0
    fn _cls(&self) {}

    // 00ee
    fn _ret(&self) {}

    // 1nnn
    fn _jp(&self, addr: u8) {}

    // 2nnn
    fn _call(&self, addr: u8) {}

    // 3xkk
    fn _se_byte(&self, x: u8, byte: u8) {}

    // 4xkk
    fn _sne_byte(&self, x: u8, byte: u8) {}

    // 5xy0
    fn _se_register(&self, x: u8, y: u8) {}

    // 6xkk
    fn _ld_byte(&self, x: u8, byte: u8) {}

    // 7xkk
    fn _add_byte(&self, x: u8, byte: u8) {}

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

    // annn
    fn _ld_i_address(&self, addr: u8) {}

    // bnnn
    fn _jp_addr(&self, addr: u8) {}

    // cxkk
    fn _rnd(&self, x: u8, byte: u8) {}

    // dxyn
    fn _drw(&self, x: u8, y: u8, nibble: u8) {}

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
