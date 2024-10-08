use std::env;
use std::io;
use image;
use std::fs::File;
use std::path::Path;

fn write_to_file(
    img_filename: &str,
    img_format: &str,
    img_pixels: &[u8],
    img_size:(usize, usize),
) -> Result<(), std::io::Error> {

    // The docs indicate that webp isn't a valid format for
    // image::save_buffer but using it produces a webp file
    // so the docs seem to be out of date.
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
    fn writes_single_pixel_to_new_image_file(){
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
    }

    #[test]
    fn saves_as_webp(){
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
        // Ummm - this seems to work, contrary to the docs?
        // Oh, and check it out! png 72 bytes, webp 36 bytes. Whoa!
    }    
    
}

fn render(pixels: &mut [u8], display: (usize, usize)) {
    for row in 0..display.1 {
        for col in 0..display.0 {
            let r = (0.1 + ((col as f32) * 0.9 / (display.0 as f32 - 1.0))) as f32;
            let g = 0.5;
            let b = (0.1 + ((row as f32) * 0.9 / (display.1 as f32 - 1.0))) as f32;

            // current pixel index: advance to row, advance to col (nb 3 vals per pixel)
            let index_red = (( row * display.0 ) + col) * 3;

            // TODO use palette crate or similar to avoid fudging the values with ~256 as f32
            pixels[index_red] = (r * 255.9999) as u8;
            pixels[index_red + 1] = (g * 255.9999) as u8;
            pixels[index_red + 2] = (b * 255.9999) as u8;
        }
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

    // Usually, 'width' and 'height' are used, but they don't convey _units_.
    // Will use 'columns' and 'rows' to avoid ambiguity.
    let display_columns: usize = 4 * 50 * 3;
    let display_rows: usize = 3 * display_columns / 4;
    println!("display: {:?} cols, {:?} rows", display_columns, display_rows);

    let display = (display_columns, display_rows);
    println!("display dim tuple: {:?}", display);
    
    let mut pixels = vec![0; 3 * display_columns * display_rows];
    render(&mut pixels, display);

    write_to_file(&path, format_default, &pixels, display).expect("image write should succeed");

}
