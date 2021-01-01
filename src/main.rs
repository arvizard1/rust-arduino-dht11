#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use arduino_uno::hal::port::portd::PD5;
use arduino_uno::hal::port::mode::TriState;

use panic_halt as _;

const TIMEOUT: u32 = 200;
struct Dht<'a> {
    pin: &'a mut PD5<TriState>
}


impl<'a>  Dht<'a> {
    fn get_readings(&mut self) -> Result<(u8,u8), (u8)> {
        let mut lows = [0;41];
        let mut highs = [0;41];
        let mut humidity:u8 = u8::MAX;
        let mut humidity_decimal:u8 = u8::MAX;
        let mut temperature:u8 = u8::MAX;
        let mut temperature_decimal:u8 = u8::MAX;

        if !self.initialize_dht() {
            return Err(1);
        }
        
        for n in 1..41 {
            lows[n] = self.expect_pulse(false);
            highs[n] = self.expect_pulse(true);
        }
        let mut o: u8 =0;
        for n in 1..41 {
            if highs[n] < lows[n] {
                o = o << 1;
            }
            else {
                o= (o << 1) | 1;
            }    
            if n%8 ==0 {               
                if n/8 == 1 {
                     humidity = o;
                }
                if n/8 == 2 {
                    humidity_decimal= o;
               }
                if n/8 == 3 {
                     temperature = o;
                }
                if n/8 == 4 {
                    temperature_decimal = o;
               }
               if n/8 == 5 {
                if (humidity + humidity_decimal + temperature + temperature_decimal) == o  {
                    return Ok((humidity, temperature));
                }
                }
                
                o=0;
            }

    }
    return Err(1);
    }
    fn expect_pulse(&mut self, level: bool) -> u32 {
        const TIMEOUT: u32 = 200;
        let mut counter: u32 = 0;
        if level {
            while self.pin.is_high().void_unwrap() {
                counter = counter +1;
                if counter > TIMEOUT {
                    return TIMEOUT;
                }
            }
        }
        else {
            while self.pin.is_low().void_unwrap() {
                counter = counter +1;
                if counter > TIMEOUT {
                    return TIMEOUT;
                }
            }
        }
        counter
    }

    fn initialize_dht(&mut self) -> bool {
        self.pin.set_low().void_unwrap();
        arduino_uno::delay_ms(20);
        self.pin.set_high().void_unwrap();
        arduino_uno::delay_us(40);
    
        let low =self.expect_pulse(false);
        let high = self.expect_pulse(true);
        if low !=TIMEOUT && high !=TIMEOUT {

            return true;
        }
        return false;
    }
}

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );
    ufmt::uwriteln!(&mut serial, "Hello from Rusty Arduino!\r").void_unwrap();
    let mut dht = pins.d5.into_tri_state(&mut pins.ddr);


    loop {
        nb::block!(serial.read()).void_unwrap();
        let mut temp_humidity = Dht {
            pin: &mut dht
        };
        match temp_humidity.get_readings() {
            Ok((h, t)) => ufmt::uwriteln!(&mut serial, "{} {}!\r", h,t).void_unwrap(),
            _ => continue,
        }
     

    }
}