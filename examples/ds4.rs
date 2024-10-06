use std::{thread, time};

use vigem_client::{BatteryStatus, DS4Buttons, DS4ReportExBuilder, DS4SpecialButtons, DS4Status, DS4TouchReport, DS4TouchPoint};

fn main() {
    // Connect to the ViGEmBus driver
    let client = vigem_client::Client::connect().unwrap();

    // Create the virtual controller target
    let id = vigem_client::TargetId::DUALSHOCK4_WIRED;
    let mut target = vigem_client::DualShock4Wired::new(client, id);

    // Plugin the virtual controller
    target.plugin().unwrap();

    // Wait for the virtual controller to be ready to accept updates
    target.wait_ready().unwrap();

    let cycle_duration = 10.0; // 10 seconds for a full cycle
    let half_cycle = cycle_duration / 2.0; // Half cycle for moving down or up
    let start = time::Instant::now();

    loop {
        let elapsed = start.elapsed().as_secs_f64();

        // Play for 1000 seconds
        if elapsed >= 1000.0 {
            break;
        }

        // Calculate the position of the touch point
        let touch_y = if elapsed % cycle_duration < half_cycle {
            // Moving down
            ((elapsed % half_cycle) / half_cycle * 942.0) as u16 // 942 is the max Y value for the touchpad
        } else {
            // Moving up
            (942.0 - ((elapsed % half_cycle) / half_cycle * 942.0)) as u16
        };

        let report = DS4ReportExBuilder::new()
            // Spin the right thumb stick in circles
            .thumb_lx(((elapsed.cos() + 1.) * 127.) as u8)
            .thumb_ly(((elapsed.sin() + 1.) * 127.) as u8)
            // Spin the right thumb stick in circles
            .thumb_rx(255 - ((elapsed.cos() + 1.) * 127.) as u8)
            .thumb_ry(255 - ((elapsed.sin() + 1.) * 127.) as u8)
            // Twiddle the triggers
            .trigger_l(((((elapsed * 1.5).sin() * 127.0) as i32) + 127) as u8)
            .trigger_r(((((elapsed * 1.5).cos() * 127.0) as i32) + 127) as u8)
            .buttons(DS4Buttons::new().cross(true).circle(true))
            .special(DS4SpecialButtons::new().ps_home(true))
            .status(DS4Status::with_battery_status(BatteryStatus::Charging(8)))
            // Set the touch report with the calculated Y position
            .touch_reports(Some(DS4TouchReport::new(0, Some(DS4TouchPoint::new(1920, touch_y)), None)), None, None)
            .build();

        let _ = target.update_ex(&report);

        thread::sleep(time::Duration::from_millis(10));
    }
}
