//Macro_use only used for the matches from yaml(App::from_yaml) Recommend not to use other macros
#[macro_use]
extern crate clap;
extern crate image;
extern crate rand;

use image::{GenericImage,GenericImageView};
use clap::App;

//Rand Used for Testing only

//  use rand::Rng;


fn main() {
    //Load the Clap yml file for commands
    let yaml = load_yaml!("cli.yml");
    //Get all command line entered commands matching cli.yml
    let matches = App::from_yaml(yaml).get_matches();
    
    //Set weights to user specified [0 to 1](Can be over)
    // Starts by making a Vec for weights
    // If user specified use their values
    let weights: Vec<_> = if matches.is_present("weights")
        {matches.values_of("weights").unwrap().collect()} 
        //Or Default for regular grayscale
        else{vec!["0.2126", "0.7152", "0.0722"]};
     
    //Make a Vec to hold the weights 
    //Converts the 'string's from command line into float 32 by parsing each element
    let f32weights: Result<Vec<f32>, _> =
    weights.iter().map(|x| x.parse()).collect();
    //ADD Check here to make sure an error didn't occur.
    //Or just leave and unwrap the OK(result)
    let f32weights = f32weights.unwrap();  
    
    //Print the weights being used for the grayscale (IF SPECIFIED)
    if matches.is_present("weights"){
        println!("Custom Weights R:{:?}, G:{:?}, B:{:?}",
            f32weights[0], 
            f32weights[1], 
            f32weights[2]);
        }
        
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    // mutable image to be modified in grayscale instead of creating a new image buffer
    //  opens value of INPUT specified at command line
    //      Unwrap of value of INPUT (Should add check here to make sure value is valid)
    //      Unwraps value of the image (Removes Ok(file) to allow use of file, should add check here)
    let mut img = image::open(matches.value_of("INPUT").unwrap()).unwrap();

    // The dimensions method returns the images width and height.
    let (width, height) = img.dimensions();
    
    
    //Tests below
/*
    //Prints the dimensions of the Image for checking for size error
    println!("dimensions({:?}, {:?})", width, height);
    
    //Random Number Generator for random pixel picking
    let mut rng = rand::thread_rng();
    
    //Pick a Random pixel in the image (testx, testy) (x,y)
    let testx = rng.gen_range(0, img.width());
    let testy = rng.gen_range(0, img.height());
    //Prints the coordinates to see which is being tested
    println!("Random Pixel ({:?}, {:?})", testx, testy);
    
    //Get the pixel at location x, y to get data to check grayscale function
    let pixel = img.get_pixel(testx,testy);
    
    
    //Get the data in the pixel, which will be the color for each channel
    //  Only need this if you dont want to write pixel.data[0] and instead write data[0]
    //let data = pixel.data;
    
    //Print out the color codes for each
    //  Can Use https://convertingcolors.com
    //      Or
    //      https://www.w3schools.com/colors/colors_picker.asp
    // Use link if you want to see the colors without trying to use other code

    println!("Value before grayscale");
    println!("rgb({:?},{:?},{:?})", pixel.data[0],pixel.data[1],pixel.data[2]);
    
    //Color space defined in terms of the CIE 1931 linear luminance Y-linear
    //      (Just took the weights found online for grayscale)
    //Average in this case is actually using the default values unless weights are specified
    //  average is just the weights for each color times by the same color all added together
    //      The use case is using percentages for weights.
    //      Can add check in f32weights conversion if instead of weights a value from 0 to 255 is passed and divid the passed value by 255 for a percent

    let average = (f32weights[0] * pixel.data[0] as f32 +
                  f32weights[1] * pixel.data[1] as f32 +
                  f32weights[2] * pixel.data[2] as f32) as u8
                  ;

    //Grayscale with image values using provided weights
    println!("Value after grayscale");
    println!("rgb({:?},{:?},{:?})", average, average, average);
*/
    
        
        //Start of code that will take in a colorsplash and use the values to check the pixels
        //  Allows for multiple times used and converts each value to a float to be used
        //  
        //  Need to put in the grayscale function starting at "let splashlen"
        let colorsplash: Vec<_> = matches.values_of("colorsplash").unwrap().collect();
        let splash: Result<Vec<f32>, _> =
        colorsplash.iter().map(|x| x.parse()).collect();
        let splash = splash.unwrap();
        let splashlen = splash.len()/4;
        
        /* For Testing Splash
        for s in 0..splashlen {
            println!("Red Splash: {:?}-{:?}",(splash[s*4] - splash[s*4+3]*255.0), (splash[s*4] + splash[s*4+3]*255.0));
            println!("Green Splash: {:?}-{:?}",(splash[s*4+1] - splash[s*4+3]*255.0), (splash[s*4+1] + splash[s*4+3]*255.0));
            println!("Blue Splash: {:?}-{:?}",(splash[s*4+2] - splash[s*4+3]*255.0), (splash[s*4+2] + splash[s*4+3]*255.0));
            //println!("Splash List {}: {:?} {:?} {:?} {:?}", s , splash[s*4], splash[s*4+1], splash[s*4+2], splash[s*4+3]));
            
                let pixel = img.get_pixel(590,370);
                
                if pixel[0] > (splash[s*4] - splash[s*4+3]*255.0) as u8 && pixel[0] < (splash[s*4] + splash[s*4+3]*255.0) as u8 &&
                pixel[1] > (splash[s*4+1] - splash[s*4+3]*255.0) as u8 && pixel[1] < (splash[s*4+1] + splash[s*4+3]*255.0) as u8 &&
                pixel[2] > (splash[s*4+2] - splash[s*4+3]*255.0) as u8 && pixel[2] < (splash[s*4+2] + splash[s*4+3]*255.0) as u8
                {
                    println!("Passed Test")
                }
                else {
                    println!("Red Test: {:?} in range of {:?} to {:?}",pixel[0],(splash[s*4] - splash[s*4+3]*255.0) as u8, (splash[s*4] + splash[s*4+3]*255.0) as u8);
                    println!(":{:?}", pixel[0]as u16 > (splash[s*4] - splash[s*4+3]*255.0));
                    println!("red({:?})", pixel[0]);
                    //println!(":{:?}", pixel[0] > (splash[s*4] + splash[s*4+3]*255.0) as u8);
                    println!(":{:?} {:?}", pixel[0], (splash[s*4] + splash[s*4+3]*255.0) as u8);
                    println!("Splash Red: {:?} Using % {:?} With multiplying by 255 gets {:?}", splash[s*4],splash[s*4+3], (splash[s*4+3]*255.0) as u8);
                    //println!("Green Test: {:?} in range of {:?}to{:?}",pixel[1],(splash[s*4+1] - splash[s*4+3]*255.0), (splash[s*4+1] + splash[s*4+3]*255.0));
                    //println!("Blue Test: {:?} in range of {:?}to{:?}",pixel[2],(splash[s*4+2] - splash[s*4+3]*255.0), (splash[s*4+2] + splash[s*4+3]*255.0));
                    
                }
        }
    */

    
    //Custom Grayscale code
    // Go through every pixel and do function
    if matches.is_present("colorsplash"){
        //println!("ColorSplash Enabled");
        for x in 0..width {
            'pixel: for y in 0..height {
                //First Get the pixel at x,y which will start from 0 and go to the end of the image
                let pixel = img.get_pixel(x, y);
                //Find the value we want to set each pixel to that will make the pixel grayscale
                //graypixel in this case is actually using the default values unless weights are specified. Default values are done in conversion
                //  graypixel is just the weights for each color times by the same color all added together
                //      graypixel = Red Weight  *  Pixel's Red +
                //                  Green Weight  *  Pixel's Green +
                //                  Blue Weight  *  Pixel's Blue
                //          This can give strange results since you could specify weights that sum to more than 1 which would oversaturate the image
                //      The use case is using percentages for weights.
                //
                
                
                //Need an If else statement to see if a colorsplash was given. If so leave that color and continue
                /*
                  if let Some(in_v) = matches.values_of("colorsplash") {
                    for in_splash in in_v {
                        println!("Color Splash: {}", in_splash);
                    }
                  }
                */
                
                for s in 0..splashlen {
                    //Check each value
                    if (pixel[0] as f32) > (splash[s*4] - splash[s*4+3]*255.0) && (pixel[0] as f32) < (splash[s*4] + splash[s*4+3]*255.0) &&
                        (pixel[1] as f32) > (splash[s*4+1] - splash[s*4+3]*255.0) && (pixel[1] as f32) < (splash[s*4+1] + splash[s*4+3]*255.0) &&
                        (pixel[2] as f32) > (splash[s*4+2] - splash[s*4+3]*255.0) && (pixel[2] as f32) < (splash[s*4+2] + splash[s*4+3]*255.0)
                        {
                            img.put_pixel(x,y, image::Rgba([pixel[0], pixel[1], pixel[2], 255]));
                        }
                    //println!("Splash List {}: {:?} {:?} {:?} {:?}", s , splash[s*4], splash[s*4+1], splash[s*4+2], splash[s*4+3] );

                    else{
                        let graypixel = (f32weights[0] * pixel[0] as f32 +
                                      f32weights[1] * pixel[1] as f32 +
                                      f32weights[2] * pixel[2] as f32) as u8;    
                        img.put_pixel(x,y, image::Rgba([graypixel, graypixel, graypixel, 255]));
                    }
                }                      


            }
        }
    }
    
    else{
        for x in 0..width {
            for y in 0..height {
                let pixel = img.get_pixel(x, y);      
                let graypixel = (f32weights[0] * pixel[0] as f32 +
                              f32weights[1] * pixel[1] as f32 +
                              f32weights[2] * pixel[2] as f32) as u8;           
                img.put_pixel(x,y, image::Rgba([graypixel, graypixel, graypixel, 255]));
            }
        }
    }
    
    
    /*
    let pixelg = img.get_pixel(testx,testy);
    println!("rgb({:?},{:?},{:?})", pixelg.data[0],pixelg.data[1],pixelg.data[2]);
    println!("The Above two will be the same if using default weights");
    */
    
   
    img.save(matches.value_of("OUTPUT").unwrap()).unwrap();
}
