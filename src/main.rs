#![no_std]
#![no_main]

use arduino_uno::prelude::*;

use panic_halt as _;


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
        ufmt::uwriteln!(&mut serial, "Reading Temperature!\r").void_unwrap();


        arduino_uno::delay_ms(500);
        dht.set_low().void_unwrap();
        arduino_uno::delay_ms(20);
        dht.set_high().void_unwrap();
        //arduino_uno::delay_us(40);
    
        let mut lows = [0;41];
        let mut highs = [0;41];
        let mut print = 0;

        for n in 0..41 {
            let mut counter = 0;
            while dht.is_low().void_unwrap() {
                counter = counter +1;
                if counter>100 {
                    break;
                }
            }
            lows[n] = counter;
            let mut counter1 = 0;
            while dht.is_high().void_unwrap() {
                counter1 = counter1 +1;
                if counter1>100 {
                    break;
                }
            }
            highs[n] = counter1;
        }
      
        // Read a byte from the serial connection
        //let b = nb::block!(serial.read()).void_unwrap();

        // Answer
        //ufmt::uwriteln!(&mut serial, "Got !\r", ).void_unwrap();
        let mut o: u8 =0;
        ufmt::uwriteln!(&mut serial, "{} {}\r",lows[0], highs[0]).void_unwrap();
        for n in 1..41 {
            

            if highs[n] < lows[n] {
                o = o << 1;
            }
            else {
                o= (o << 1) | 1;
            }    
            if n%8 ==0 {
                ufmt::uwriteln!(&mut serial, "{}\r",o).void_unwrap();

                o=0;

            }
        }       

    }
}