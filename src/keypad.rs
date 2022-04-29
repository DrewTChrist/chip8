use embedded_hal::digital::v2::{InputPin, OutputPin};

const R: usize = 4;
const C: usize = 4;

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
    //pub fn new<E>(rows: [O; R], cols: [I; C]) -> Result<Self, E>
    pub fn new<E>(rows: [O; R], cols: [I; C]) -> Self
    where
        O: OutputPin<Error = E>,
        I: InputPin<Error = E>,
    {
        //Ok(Self { rows, cols })
        Self { rows, cols }
    }
}
