#[cfg(test)]
mod chip8 {
    use chip8::fonts::DEFAULT;
    use chip8::keypad::KeyPad;
    use chip8::Chip8;
    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::pixelcolor::Rgb565;
    use embedded_hal::digital::v2::{InputPin, OutputPin};
    use embedded_hal_mock::{
        common::Generic,
        delay::MockNoop as MockDelay,
        pin::{Mock as MockPin, State as PinState, Transaction as PinTransaction},
    };
    use rand::RngCore;

    struct MockRng;

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, _: &mut [u8]) {}
        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), rand::Error> {
            Ok(())
        }
    }

    type MockChip8 = Chip8<
        MockDisplay<Rgb565>,
        Generic<PinTransaction>,
        Generic<PinTransaction>,
        MockRng,
        MockDelay,
    >;

    fn get_fixture_no_keypad() -> MockChip8 {
        let expect = [PinTransaction::set(PinState::High)];

        let keypad = KeyPad::new(
            [
                MockPin::new(&expect),
                MockPin::new(&expect),
                MockPin::new(&expect),
                MockPin::new(&expect),
            ],
            [
                MockPin::new(&[]),
                MockPin::new(&[]),
                MockPin::new(&[]),
                MockPin::new(&[]),
            ],
        );

        let mut chip8 = Chip8::new(
            MockDisplay::<Rgb565>::new(),
            keypad,
            MockRng {},
            MockDelay::default(),
        );

        chip8.load_font(DEFAULT);

        chip8
    }

    #[test]
    fn _1nnn() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0x12, 0x02, 0x1f, 0xff]);
        chip8.tick();
        assert_eq!(chip8.get_program_counter(), 0x202);
        chip8.tick();
        assert_eq!(chip8.get_program_counter(), 0xfff);
    }

    #[test]
    fn _2nnn() {}

    #[test]
    fn _3xnn_true() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0x30, 0xff]);
        chip8.write_register(0x0, 0xff);
        chip8.tick();
        assert_eq!(chip8.get_program_counter(), 0x204);
    }

    #[test]
    fn _3xnn_false() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0x30, 0x00]);
        chip8.write_register(0x0, 0xff);
        chip8.tick();
        assert_eq!(chip8.get_program_counter(), 0x202);
    }

    #[test]
    fn _4xnn_true() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0x40, 0x00]);
        chip8.write_register(0x0, 0xff);
        chip8.tick();
        assert_eq!(chip8.get_program_counter(), 0x204);
    }

    #[test]
    fn _4xnn_false() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0x40, 0xff]);
        chip8.write_register(0x0, 0xff);
        chip8.tick();
        assert_eq!(chip8.get_program_counter(), 0x202);
    }

    #[test]
    fn _5xy0_true() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0x50, 0x10]);
        chip8.write_register(0x0, 0xff);
        chip8.write_register(0x1, 0xff);
        chip8.tick();
        assert_eq!(chip8.get_program_counter(), 0x204);
    }

    #[test]
    fn _5xy0_false() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0x50, 0x10]);
        chip8.write_register(0x0, 0x00);
        chip8.write_register(0x1, 0xff);
        chip8.tick();
        assert_eq!(chip8.get_program_counter(), 0x202);
    }

    #[test]
    fn _6xnn() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0x60, 0xa]);
        chip8.tick();
        assert_eq!(chip8.get_registers()[0x0], 0xa);
        assert_eq!(chip8.get_program_counter(), 0x202);
    }

    #[test]
    fn _7xnn() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0x70, 0x09]);
        chip8.write_register(0x0, 0x01);
        chip8.tick();
        assert_eq!(chip8.get_registers()[0x0], 0xa);
        assert_eq!(chip8.get_program_counter(), 0x202);
    }

    #[test]
    fn _7xnn_overflow() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0x70, 0x01]);
        chip8.write_register(0x0, 0xff);
        chip8.tick();
        assert_eq!(chip8.get_registers()[0x0], 0x00);
        assert_eq!(chip8.get_program_counter(), 0x202);
    }

    #[test]
    fn _fx55() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[0xff, 0x55]);
        chip8.set_index(0x203);
        chip8.write_register(0x00, 0xff);
        chip8.write_register(0x01, 0xff);
        chip8.write_register(0x02, 0xff);
        chip8.write_register(0x03, 0xff);
        chip8.write_register(0x04, 0xff);
        chip8.write_register(0x05, 0xff);
        chip8.write_register(0x06, 0xff);
        chip8.write_register(0x07, 0xff);
        chip8.write_register(0x08, 0xff);
        chip8.write_register(0x09, 0xff);
        chip8.write_register(0x0a, 0xff);
        chip8.write_register(0x0b, 0xff);
        chip8.write_register(0x0c, 0xff);
        chip8.write_register(0x0d, 0xff);
        chip8.write_register(0x0e, 0xff);
        chip8.write_register(0x0f, 0xff);
        chip8.tick();
        assert_eq!(chip8.get_program_counter(), 0x202);
        assert_eq!(chip8.get_program_memory()[0x03], 0xff);
        assert_eq!(chip8.get_program_memory()[0x12], 0xff);
    }

    #[test]
    fn _fx65() {
        let mut chip8 = get_fixture_no_keypad();
        chip8.load_program(&[
            0xff, 0x65, 0x00, 0xff, 0xff, 
            0xff, 0xff, 0xff, 0xff, 0xff, 
            0xff, 0xff, 0xff, 0xff, 0xff, 
            0xff, 0xff, 0xff, 0xff,
        ]);
        chip8.set_index(0x203);
        chip8.tick();
        assert_eq!(chip8.get_program_counter(), 0x202);
        assert_eq!(chip8.get_registers()[0x0], 0xff);
        assert_eq!(chip8.get_registers()[0xf], 0xff);
    }
}
