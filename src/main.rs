use std::env;
use std::io;
use image;
use std::fs::File;
use std::path::Path;
use std::time;

mod point3;
mod vec3;
mod render;

fn write_to_file(
    img_filename: &str,
    img_format: &str,
    img_pixels: &[u8],
    img_size:(usize, usize),
) -> Result<(), std::io::Error> {

    // The docs indicate that webp isn't a valid format for
    // image::save_buffer but using it produces a webp file? -vOv-
    let format_ok = match img_format {
        "webp"
        | "jpeg" 
        | "png" 
        | "ico" 
        | "pnm" 
        | "bmp" 
        | "exr" 
        | "tiff" => true,
        _ => false,

    };

    if !format_ok {
        panic!("unsupported image format")
    } 

    let filename = [img_filename, ".", img_format].concat();
    let path = Path::new(&filename);
    File::create(path)?;
    image::save_buffer(
        path,
        img_pixels,
        img_size.0 as u32,
        img_size.1 as u32,
        image::ExtendedColorType::Rgb8,
    ).unwrap();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn writes_to_png(){
        let img_filename = "static/single_pixel";
        let img_format = "png";
        let img_pixels: [u8; 3] = [100, 10, 200];
        let img_size:(usize, usize) = (1, 1);
        let result = write_to_file(
            img_filename,
            img_format,
            &img_pixels, 
            img_size,
        );

        assert!(result.is_ok());
        // single pixel as png: 72 bytes
    }

    #[test]
    fn writes_to_webp(){
        let img_filename = "static/wrong_format";
        let img_format = "webp";
        let img_pixels: [u8; 3] = [100, 10, 200];
        let img_size:(usize, usize) = (1, 1);
        let result = write_to_file(
            img_filename,
            img_format,
            &img_pixels,
            img_size,
        );

        assert!(result.is_ok());
        // single pixel as webp: 36 bytes
    }    
}

fn main() {
    println!("Let's trace some rays in Rust...\n");

    let mut env_args: Vec<String> = env::args().collect();
    if env_args.len() < 2 {
        println!("Enter a target filename: ");
        let mut target = String::new();
        io::stdin().read_line(&mut target).expect("Utf-8 encoded input");
        target.pop();
        env_args.push(target);
    }

    println!("env args: {:?}", env_args);
    let path = ["static", "/", &env_args[1]].concat();
    println!("path: {}", path);

    let format_default = "webp";
    
    // Width as 'columns', height as 'rows'.
    let common = 50;
    let display_columns: usize = 4 * common * 3;
    let display_rows: usize = 3 * display_columns / 4;
    let display = (display_columns, display_rows);
    println!("display (cols, rows): {:?}", display);

    let mut pixels = vec![0; 3 * display.0 * display.1];
    println!("Start render...");
    let now = time::SystemTime::now();    
    render::render(&mut pixels, display); 
    let elapsed = now.elapsed();
    println!("Render: {:?} milliseconds.", elapsed.unwrap().as_millis());     

    println!("Start write...");
    let now = time::SystemTime::now();
    write_to_file(
        &path, 
        format_default, 
        &pixels, 
        display,
    ).expect("image write should succeed");
    let elapsed = now.elapsed();
    println!("File write: {:?} milliseconds.", elapsed.unwrap().as_millis());

}
