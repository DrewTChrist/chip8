use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{InputPin, OutputPin};

const R: usize = 4;
const C: usize = 4;

/// A KeyPad struct for representing and scanning a button matrix
///
/// Expects an array of 4 OutputPins and an array of 4 Input Pins
pub struct KeyPad<O, I>
where
    O: OutputPin,
    I: InputPin,
{
    rows: [O; R],
    cols: [I; C],
}

impl<O, I> KeyPad<O, I>
where
    O: OutputPin,
    I: InputPin,
{
    pub fn new<E>(rows: [O; R], cols: [I; C]) -> Self
    where
        O: OutputPin<Error = E>,
        I: InputPin<Error = E>,
    {
        let mut s = Self { rows, cols };
        for r in s.rows.iter_mut() {
            if r.set_high().is_err() {}
        }
        s
    }

    pub fn get<D>(&mut self, delay: &mut D) -> (bool, u8)
    where
        D: DelayMs<u32>,
    {
        let mut index: u8 = 0;
        let mut key: (bool, u8) = (false, 0);
        for row in (&mut self.rows).iter_mut() {
            if row.set_low().is_err() {}
            delay.delay_ms(10);
            for col in (&mut self.cols).iter_mut() {
                if let Ok(_) = col.is_low() {
                    delay.delay_ms(10);
                    if let Ok(b) = col.is_low() {
                            if b {
                                key = (true, index);
                            }
                    }
                }
                index += 1;
            }
            if row.set_high().is_err() {}
        }
        key
    }
}
