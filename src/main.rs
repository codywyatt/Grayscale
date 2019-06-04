//Macro_use only used for the matches from yaml(App::from_yaml) Recommend not to use other macros
#[macro_use]
extern crate clap;
extern crate image;
extern crate rand;

use image::{GenericImage,GenericImageView};
use clap::App;
use rand::Rng;





fn main() {
    //Load the Clap yml file for commands
    let yaml = load_yaml!("cli.yml");
    //Get all command line entered commands matching cli.yml
    let matches = App::from_yaml(yaml).get_matches();
    
    //Set weights to user specified [0 to 1] ***Still Needs to be tested for***
    // Starts by making a Vec for weights
    // If user specified use their values
    let weights: Vec<_> = if matches.is_present("weights")
        {matches.values_of("weights").unwrap().collect()} 
        //Or Default for grayscale
        else{vec!["0.2126", "0.7152", "0.0722"]};
     
     
    let f32weights: Result<Vec<f32>, _> =
    weights.iter().map(|x| x.parse()).collect();
    let f32weights = f32weights.unwrap();  
    
    //Print the weights being used for the grayscale
    println!("Weights R:{:?}, G:{:?}, B:{:?}",
        f32weights[0], 
        f32weights[1], 
        f32weights[2]);
        
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let mut img = image::open(matches.value_of("INPUT").unwrap()).unwrap();

    // The dimensions method returns the images width and height.
    // Could use later for other image uses.
    let (width, height) = img.dimensions();
    println!("dimensions({:?}, {:?})", width, height);

    //Will Replace With custom Grayscale functions with command line interface commands
        //Will totally rework grayscale to be custom options
            //Things like Custom values for each color ***Implimented***
            //Hopefully enough time to allow for threshold of colors to remain
                //Picking a color of Red with color 255 out of 255 with a default percent of range to remain for example, all three colors would have this percent each time of course.
                //Allow for that percent to be changed for all or each color
    //Start of going through and checking pixels
    //Tests below
    
    let mut rng = rand::thread_rng();
    let testx = rng.gen_range(0, img.width());
    let testy = rng.gen_range(0, img.height());
    println!("Random Pixel ({:?}, {:?})", testx, testy);
    
    //Get the pixel at location x, y
    let pixel = img.get_pixel(testx,testy);
    
    //Get the data in the pixel, which will be the color for each channel
    let data = pixel.data;
    //Print out the color codes for each
    
    //      https://www.w3schools.com/colors/colors_picker.asp
    // Use link if you want to see the colors without trying to use other code
    println!("rgb({:?},{:?},{:?})", data[0],data[1],data[2]);
    println!("Value after grayscale below");
    
    //Color space defined in terms of the CIE 1931 linear luminance Y-linear
    //      (Just took the weights found online for grayscale)
    //Average in this case is actually using the default values unless weights are specified
    
    let average = (f32weights[0] * data[0] as f32 +
                  f32weights[1] * data[1] as f32 +
                  f32weights[2] * data[2] as f32) as u8
                  ;
                  
    //Grayscale with image values using provided weights
    println!("rgb({:?},{:?},{:?})", average, average, average);
    //
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let graypixel = (f32weights[0] * pixel[0] as f32 +
                          f32weights[1] * pixel[1] as f32 +
                          f32weights[2] * pixel[2] as f32) as u8
                          ;
            img.put_pixel(x,y, image::Rgba([graypixel, graypixel, graypixel, 255]));
        }
    }

    let testdata = img.get_pixel(0,0).data;
    println!("Test Data{:?})", testdata);

    
    //graypixels(&img, f32weights[0],f32weights[1],f32weights[2]);
    let gray = img;
    
    let pixelg = gray.get_pixel(testx,testy);
    let datag = pixelg.data;
    println!("rgb({:?},{:?},{:?})", datag[0],datag[1],datag[2]);
    //
    println!("The Above two will be the same if using default weights");
    
   
    gray.save(matches.value_of("OUTPUT").unwrap()).unwrap();
}
