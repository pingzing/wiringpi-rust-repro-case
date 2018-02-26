use std::thread;

extern crate wiringpi;

fn main() {
    
    let pi = wiringpi::setup_gpio();

    let pin_4 = pi.soft_pwm_pin(4);

    for i in 0..100 {
        pin_4.pwm_write(i);
        thread::sleep_ms(100);
    }
    
    thread::sleep_ms(1000);
}
