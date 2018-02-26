use std::env;

extern crate wiringpi;

fn main() {

     let args: Vec<String> = env::args().collect();

     let value = &args[1].parse::<i32>().unwrap();
    
    let pi = wiringpi::setup_gpio();

    let pin_4 = pi.soft_pwm_pin(4);
    pin_4.pwm_write(*value);
}
