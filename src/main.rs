//Macro_use only used for the matches from yaml(App::from_yaml) Recommend not to use other macros
#[macro_use]
extern crate clap;
extern crate image;

use image::GenericImageView;
use clap::App;

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

    //Print the weights being used for the grayscale
    println!("Weights:: R:{},G:{},B:{}", weights[0], weights[1], weights[2]);
        
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let img = image::open(matches.value_of("INPUT").unwrap()).unwrap();

    // The dimensions method returns the images width and height.
    // Could use later for other image uses.
    println!("dimensions {:?}", img.dimensions());

    // Write the contents of this image to the Writer in selected format.

    //Will Replace With custom Grayscale functions with command line interface commands
        //Will totally rework grayscale to be custom options
            //Things like Custom values for each color ***Implimented***
            //Hopefully enough time to allow for threshold of colors to remain
                //Picking a color of Red with color 255 out of 255 with a default percent of range to remain for example, all three colors would have this percent each time of course.
                //Allow for that percent to be changed for all or each color
    //Start of going through and checking pixels
    //Tests below
    
    //Get the pixel at location x, y
    let pixel = img.get_pixel(0,0);
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
    let average = (0.2126f32 * data[0] as f32 +
                  0.7152f32 * data[1] as f32 +
                  0.0722f32 * data[2] as f32) as u8
                  ;
                  
    //Grayscale with image values using provided weights
    println!("rgb({:?},{:?},{:?})", average, average, average);
    //
    let gray = img.grayscale();
    
    let pixelg = gray.get_pixel(0,0);
    let datag = pixelg.data;
    println!("rgb({:?},{:?},{:?})", datag[0],datag[1],datag[2]);
    //
    println!("The Above two should be the exact same always");
    
   
    gray.save("out.png").unwrap();
}
