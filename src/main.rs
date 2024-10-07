use image;
use std::fs::File;
use std::path::Path;

fn write_to_file(
    img_filename: &str,
    img_format: &str,
    img_pixels: &[u8],
    img_size:(usize, usize),
) -> Result<(), std::io::Error> {

    let format_ok = match img_format {
        "jpeg" 
        | "png" 
        | "ico" 
        | "pnm" 
        | "bmp" 
        | "exr" 
        | "tiff" => true,
        _ => false,

    };

    // TODO Improve error handling of unsupported formats. 
    // Although... as it happens, not introducing a panic enabled
    // serendipitous discovery of webp being supported by save_buffer!
    if !format_ok {
        println!("Unsupported image format. Expect an error for image::save_buffer.")
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

fn main() {
    println!("Hello, world! Welcome to ruray - my rust ray tracer.");

}
