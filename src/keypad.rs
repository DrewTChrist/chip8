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
}
