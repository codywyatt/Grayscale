REM Basic command. With just saying folder
cargo run images/Rose.jpg images/NoWeightGrayRose.jpg

REM Basic Command. With ./ to show it still works with or without
REM This also shows it works with a different image
REM NOTE: Other image formats can be used for either INPUT or OUTPUT
    REM Examples: png bmp gif ico jpeg pnm webp
    REM This is done using the 'image' crate from crates.io
cargo run ./images/Rose2.jpg ./images/NoWeightGrayRose2.jpg

REM Removing REM from below command will give an error explaining how to use the -weights command with helpful documentation
REM cargo run images/Rose.jpg images/GrayRose.jpg -w

REM Showing the default values for cargo run
REM Also showing how when custom weights are used it will print to console

REM The color codes can be found by multiplying or dividing by 255 (take normal RGB color code divided by 255 will get percent) NOTE: To convert online you need to truncate the value "54.213" would just be "54"
REM Top website to convert is "https://convertingcolors.com/" (At covert line type 'RGBPercent(21%, 72%, 7%)'  )

REM Multiplying these values for the default cargo run gives us rgb(54.213, 182.376, 18.411‬‬)
REM To properly convert these values to a color you can just remove the numbers after the '.'
    REM This rgb will add up to 255 since cargo run should only give us max of 255 for all colors
cargo run images/Rose.jpg images/DefaultRose.jpg -w 0.2126 0.7152 0.0722

REM The default values favour green in the value followed by red then blue
REM The reason behind this is to make images appear brighter by increasing "green"
REM This color looks to be a generic green that someone might think of

REM Command that will "extract" color channel/data for one color
cargo run images/Rose.jpg images/RedRose.jpg -w 1 0 0
cargo run images/Rose.jpg images/GreenRose.jpg -w 0 1 0
cargo run images/Rose.jpg images/BlueRose.jpg -w 0 0 1

REM Note: I added a test to check for the values being total less than 1 to prevent over saturation but commented it out.
REM "Its a feature not a bug!" 
cargo run images/Rose.jpg images/SaturationRose.jpg -w 1 1 1

REM Under saturated is fine of course and also this shows that a zero is not needed before the .2 in the command
cargo run images/Rose.jpg images/UnderSaturatedRose.jpg -w .2 .2 .2

REM Average weights for another way to do cargo run (Adds all together and takes average for gray)
cargo run images/Rose.jpg images/AverageRose.jpg -w .3333 .3333 .3333