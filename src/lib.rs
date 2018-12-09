
#![no_std]

use embedded_hal::blocking::i2c::WriteRead;

const MAX17048_ADDR: u8 = 0x6C;


pub struct Max17048<I> {
    i2c: I,
    recv_buffer: [u8; 2]
}


impl<I, E> Max17048<I> 
where I: WriteRead<Error = E>,
      E: core::fmt::Debug
{

    pub fn new(i2c: I) -> Self {
        Max17048 {
            i2c: i2c,
            recv_buffer: [0u8; 2]
        }
    }

    pub fn version(&mut self) -> Result<u16, E> {
        self.read(0x08)
    }

    pub fn soc(&mut self) -> Result<u16, E> {
        match self.read(0x04) {
            Ok(val) => Ok(val / 256),
            Err(e) => Err(e)
        }
    }

    /// Return C/Rate in %/hr
    pub fn charge_rate(&mut self) -> Result<f32, E> {
        match self.read(0x16) {
            Ok(val) => Ok(val as f32 * 0.208),
            Err(e) => Err(e)
        }
    }

    pub fn vcell(&mut self) -> Result<f32, E> {
        match self.read(0x02) {
            Ok(val) => Ok(val as f32 * 0.000078125),
            Err(e) => Err(e)
        }
    }

    fn read(&mut self, reg: u8) -> Result<u16, E> {
        match self.i2c.write_read(MAX17048_ADDR, &[reg], &mut self.recv_buffer) {
            Ok(_) => Ok((self.recv_buffer[0] as u16) << 8 | self.recv_buffer[1] as u16),
            Err(e) => Err(e)
        }
    }

    // fn read(&mut self, reg: u8) -> u16 {
    //     self.i2c.write_read(MAX17048_ADDR, &[reg], &mut self.recv_buffer).unwrap();
    //     (self.recv_buffer[0] as u16) << 8 | self.recv_buffer[1] as u16
    // }
}
