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
        Self { rows, cols }
    }
    pub fn get(&mut self) -> (bool, u8) {
        let mut key = (false, 0);
        for (ri, row) in (&mut self.rows).iter_mut().enumerate() {
            if row.set_low().is_err() {}
            for (ci, col) in (&mut self.cols).iter_mut().enumerate() {
                match col.is_low() {
                    Ok(b) => {
                        key = (true, (((ri + 1) * (ci + 1)) - 1) as u8);
                    }
                    Err(e) => {}
                }
            }
        }
        key
    }
}
