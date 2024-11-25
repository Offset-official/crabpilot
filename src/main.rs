use navigator_rs::{Navigator, PwmChannel, UserLed};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut nav = Navigator::new();
    nav.init();

    println!("Navigator initiliased");
    nav.set_led(UserLed::Led1, true);

    // startup delay
    sleep(Duration::from_secs(5));

    arm_thrusters(&mut nav);
    nav.set_led(UserLed::Led2, true);
    nav.set_led(UserLed::Led3, true);
    move_forward(&mut nav, Duration::from_secs(5));
    stop_thrusters(&mut nav);
    nav.set_led(UserLed::Led2, false);
    nav.set_led(UserLed::Led3, false);
}

fn arm_thrusters(nav: &mut Navigator) {
    let channels = [PwmChannel::Ch1, PwmChannel::Ch2];
    // send esc init signal
    nav.set_pwm_channels_value(&channels, 1500);
    sleep(Duration::from_secs(1));
}

fn move_forward(nav: &mut Navigator, duration: Duration) {
    let channels = [PwmChannel::Ch1, PwmChannel::Ch2];
    let values = [1450, 1550];
    nav.set_pwm_channels_values(&channels, &values);
    sleep(duration);
}

fn stop_thrusters(nav: &mut Navigator) {
    let channels = [PwmChannel::Ch1, PwmChannel::Ch2];

    nav.set_pwm_channels_value(&channels, 1500);
    nav.set_pwm_enable(false);
}
