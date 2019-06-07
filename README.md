# Grayscale
Binary to take in a file with (Optional) commands and flags and return a gray scale image

Copyright (c)  2019 Cody Wyatt

## Commands

```
Required:
	<Input File> <Output File>
Optional:
	-w(eights) <Red> <Green> <Blue>
		weights must be a float between 0 and 1 in total
		Defaults are set to be traditional weights
Info/Help:
	-h help enabled by Clap
```

# Authors

* **Cody Wyatt** - *Initial work* - <Cwyatt@pdx.edu>

# License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details


# Project Writeup

## What was built

The project that was built was a command line program that would take in an image and output another image with the desired grayscale values. The intent was to have a program in rust that would have custom grayscale values instead of the 'image' crates default grayscale which is also used as default if the user just wanted to use the image library for grayscale without having to program anything themselves. The project uses the 'clap' crate so that commands could be passed in through the command line without having to try and remake all the functionality that it provides. The code for this is in a yaml file instead of the in-line code that examples use to reduce that ammount of clutter that is in the main file.

## How it worked

This project works by taking the 'image' library crate and the 'clap' library crate to allow for an image to be selected through the command line with custom parameters with the image decoding/incoding modules from the 'image' crate. Once the image has been read in my implimentation takes the image and goes through every pixel and adds the values for each color after applying the custom weights then outputs one value to be used for all three color values. Doing this assures that the color can only be a 'gray' color, or a color between black and white.

## What doesn't work

Currently in the state the project is in everything works as intended and custom grayscale images can be created very quick and the use of a 'bat' file allows for many commands on this program to be ran one after the other. One could take a bat file and have it run through a whole folder and take the name of the file and add 'Gray' or some other indicator to prevent overriding or even just output to another file. I prefer this method since the commands could vary from picture to picture and if you wanted to say change the image output type it would be fairly simple with the 'bat' file.
As far as what doesn't work though I wanted to have a 'color splash' or 'selective coloriation' sort of function that would take another value of color and keep that color in the image. Also I wanted to be able to pass in weights that were from 0 to 255 instead of just percent and we could just divide by 255 to get the percent and still use the same functionality from everything else.
Also another planned idea was to instead of having a 'bat' file to have a way to just pass a folder in and with either another folder to output to with the same filenames or to the same file with an added 'flag' in the image name to prevent overwriting of the images.
Also I wanted to try and allow for images with removed backgrounds to still have the removed backgrounds after grayscaling but the project will just output it to white.

## Lessons Learned

Learning Rust was great and I found it to be useful and fairly easy to understand after writing the code. One thing I learned is that Rust is still fairly new so finding examples of special cases of what I wanted to achieve were somewhat difficult to come by.

For Example I was unsure how to cast input from the command line which was a string into a float 32 I needed for creating my custom weights. Also in doing this I had to learn how to use the 'clap' crate to determine when a value was passed in or not (is_present) and if not use a default value for the weights. Also in testing I was testing a random pixel and forgot to check the bounds on the image so I had to figure out how to find the image size so I could check any pixel in the bound along with running a loop to do calculations on every pixel. (let (width, height) = img.dimensions();) and (for x in 0..width {for y in 0..height {}})
Another leason learned was in the image crate that I needed to pass an extra value to allow me to 'put_pixel' in the image (img.put_pixel(x,y, image::Rgba([graypixel, graypixel, graypixel, 255]));) since the put_pixel function only worked on Rgba.
