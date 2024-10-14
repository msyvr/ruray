use std::{alloc::System, thread::sleep};
use std::time::{Duration, SystemTime};

use indicatif::{ProgressBar, ProgressStyle};

pub fn render(pixels: &mut [u8], display: (usize, usize)) {

    let pb = ProgressBar::new((display.1).try_into().unwrap());    
    pb.set_style(
        ProgressStyle::default_bar()
        .template("[{bar:40.cyan/red}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("~>/")
    );

    // Compute display pixel values based on scene 
    // object interactions and world lighting.  

    for row in 0..display.1 {
        let now = SystemTime::now();
        for col in 0..display.0 {
            let r = (0.1 + ((col as f32) * 0.9 / (display.0 as f32 - 1.0))) as f32;
            let g = 0.5;
            let b = (0.1 + ((row as f32) * 0.9 / (display.1 as f32 - 1.0))) as f32;

            // Color representation requires 3 values per pixel: 
            // red-green-blue || cyan-magenta-yellow.
            let index_red = (( row * display.0 ) + col) * 3;

            pixels[index_red] = (r * 255.9999) as u8;
            pixels[index_red + 1] = (g * 255.9999) as u8;
            pixels[index_red + 2] = (b * 255.9999) as u8;
        }
        if now.elapsed().unwrap() < Duration::from_millis(10) {
            sleep(Duration::from_millis(10));
        }
        pb.set_position(row.try_into().unwrap());
    }
}