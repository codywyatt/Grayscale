#[macro_use]
extern crate clap;
extern crate image;

use image::GenericImageView;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let img = image::open(matches.value_of("INPUT").unwrap()).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.

    //Will Replace With custom Grayscale functions with command line interface commands
        //Will totally rework grayscale to be custom options
            //Things like Custom values for each color (RGB)
            //Hopefully enough time to allow for threshold of colors to remain
                //Picking a color of Red with color 255 out of 255 with a default percent of range to remain for example, all three colors would have this percent each time of course.
                //Allow for that percent to be changed for all or each color
    let pixel = img.get_pixel(500,500);
    let data = pixel.data;
    println!("Red:{:?}  Green:{:?}  Blue:{:?}", data[0],data[1],data[2]);
    println!("Value after grayscale below");
    
    //Color space defined in terms of the CIE 1931 linear luminance Y-linear
    //(Just took the weights found online for grayscale)
    let average = (0.2126f32 * data[0] as f32 +
                  0.7152f32 * data[1] as f32 +
                  0.0722f32 * data[2] as f32) as u8
                  ;
    println!("Red:{:?}  Green:{:?}  Blue:{:?}", average, average, average);
    
    let gray = img.grayscale();
    
    let pixelg = gray.get_pixel(500,500);
    let datag = pixelg.data;
    println!("Red:{:?}  Green:{:?}  Blue:{:?}", datag[0],datag[1],datag[2]);
    println!("The Above two should be the exact same always");
    
 
    //Code To Resize if needed
        //let gray = gray.resize(gray.width()/2, gray.height()/2, image::FilterType::Nearest);
            
    gray.save("out.png").unwrap();
}