use super::{Modules, Reg};
use crate::{devices::SeesawDevice, Driver, DriverExt, SeesawError};

const TOUCH_CHANNEL: &Reg = &[Modules::Touch.into_u8(), 0x10];

pub trait TouchModule<D: Driver>: SeesawDevice<Driver = D> {
    fn touch_read(&mut self, _pin: u8) -> Result<u16, SeesawError<D::Error>> {
        let addr = self.addr();
        let mut retry_counter = 0;

        loop {
            self.driver().delay_us(1000);
            match self.driver().read_u16_with_delay(addr, TOUCH_CHANNEL, 5000) {
                Ok(cap) => {
                    if cap < u16::max_value() {
                        return Ok(cap);
                    }
                }
                Err(e) => {
                    retry_counter += 1;
                    if retry_counter > 2 {
                        return Err(SeesawError::I2c(e));
                    }
                    continue;
                }
            }
        }
    }
}
