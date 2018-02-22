extern crate wiringpi;

fn main() {
    
    let pi = wiringpi::setup_gpio();

    let pin_4 = pi.soft_pwm_pin(4);
    pin_4.pwm_write(50);
}
