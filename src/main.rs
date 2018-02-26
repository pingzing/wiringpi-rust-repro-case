use std::thread;

extern crate wiringpi;

fn main() {
    
    let pi = wiringpi::setup_gpio();

    let mut pins: Vec<wiringpi::pin::SoftPwmPin<wiringpi::pin::Gpio>> = Vec::new();

    pins.push(pi.soft_pwm_pin(17));
    pins.push(pi.soft_pwm_pin(27));
    pins.push(pi.soft_pwm_pin(22));

    for i in 0..100 {
        for pin in &pins {
            pin.pwm_write(i);
        }        
        thread::sleep_ms(100);
    }
    
    thread::sleep_ms(1000);
}
