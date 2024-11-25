use navigator_rs::{Navigator, PwmChannel};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut nav = Navigator::new();
    nav.init();

    println!("Navigator initiliased");

    arm_thrusters(&mut nav);
    move_forward(&mut nav, Duration::from_secs(5));
    stop_thrusters(&mut nav);
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
