use mouse_keyboard_input::{Coord, VirtualDevice};
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pudding", about = "Jiggles the mouse cursor.")]
struct Opt {
    #[structopt(
        short = "p",
        long = "period",
        help = "Jiggling period in seconds",
        default_value = "1"
    )]
    jiggling_period_secs: u64,

    #[structopt(
        short = "d",
        long = "duration",
        help = "Program running duration in minutes (if not specified, runs indefinitely)"
    )]
    running_duration_minutes: Option<u64>,
}

fn main() {
    let opt = Opt::from_args();

    let mut device = VirtualDevice::default().unwrap();

    let jiggling_period_secs = opt.jiggling_period_secs;

    if let Some(duration_minutes) = opt.running_duration_minutes {
        let running_duration_secs = duration_minutes
            .checked_mul(60)
            .expect("Duration too large");
        let mut current_duration = 0;

        while current_duration < running_duration_secs {
            jiggle_mouse(&mut device, jiggling_period_secs, None);

            current_duration += 2 * jiggling_period_secs;
        }
    } else {
        loop {
            jiggle_mouse(&mut device, jiggling_period_secs, None);
        }
    }
}

fn jiggle_mouse(device: &mut VirtualDevice, jiggling_period_secs: u64, pixels: Option<Coord>) {
    let pixels = pixels.unwrap_or(5);

    device.move_mouse(pixels, 0).unwrap();
    sleep(Duration::from_secs(jiggling_period_secs));
    device.move_mouse(-pixels, 0).unwrap();
    sleep(Duration::from_secs(jiggling_period_secs));
}
