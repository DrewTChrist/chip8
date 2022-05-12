use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::blocking::delay::DelayMs;

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
            if r.set_high().is_err() {
            }
        }
        s
    }

    pub fn get<D>(&mut self, delay: &mut D) -> (bool, u8) 
    where
        D: DelayMs<u32>
    {
        let mut index: u8 = 0;
        for row in (&mut self.rows).iter_mut() {
            if row.set_low().is_err() {}
            for col in (&mut self.cols).iter_mut() {
                match col.is_low() {
                    Ok(_) => {
                        delay.delay_ms(10);
                        match col.is_low() {
                            Ok(b) => {
                                if b {
                                    return (b, index)
                                }
                            },
                            Err(e) => {}
                        }
                    }
                    Err(e) => {}
                }
                index += 1;
            }
            if row.set_high().is_err() {}
        }
        return (false, 0);
    }
}
