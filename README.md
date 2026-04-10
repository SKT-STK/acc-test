# ACC-TEST #

## What is this? ##

acc-test (im too lazy to change the name) is a simple command line program written in Rust which allows you to automate changing TC and BB settings throughout the lap.

## How does it work? ##

The program utilizes ACC's API (shared memory region) to get the current track position as well as current brake-balance and uses a virtual keyboard that comes with Logitech G-Hub to automate keybind presses for specific TC levels and BB adjustments.

## How to use? ##

Start by either downloading the built acc-test.exe alongside InputSimulator.dll from the releases page, or build from source using `cargo build --release`.  
To use the tool, execute the .exe from the terminal with the following options:  

- TC adjustments as pairs of track percentage and keybind hex-code within parentheses (optionally followed by minimum speed for activation, great for SPA race starts),  
- BB adjustments as pairs of track percentage and requested BB within brackets,  

additionally, if you wish to use the BB functionality you must include keybind hex-codes for increasing and decreasing BB as well as a car-specific offset in brackets before any actual BB points.  
If thats too confusing, see examples below.

### Usage Examples ###

Lets say you wanna change the TC level to 1 before eau rouge at SPA, well use 8% of that track in this example. First you must bind TC level 1 to any key in ACC settings, well use Numpad0 for this example. The hex-code for Numpad0 is 0x60 so the command line would look like this:  
`acc-test.exe (8 0x60)`  
then, after the climb lets say you want it back at TC 5, so you have to bind TC level 5 to another key, lets say Numpad1:  
`acc-test.exe (8 0x60) (25 0x61)`  
now lets say you dont feel comfortable with TC1 at race start, we can utilize the minimum speed functionality to prevent that:  
`acc-test.exe (8 0x60 80) (25 0x61)`.  
The hex codes for each and every key can be found here: <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes>.  


Now, lets explore the BB functionality. Well again use SPA and the busstop chicane with the Ferrari 296 GT3 as the example. As stated previously the BB part of the command line must start with keybind hex-codes for increasing and decreasing BB in brackets, for this example, bind them to Numpad3 and Numpad5:  
`acc-test.exe [0x63 0x65 -5] [92 50.0] [99 54.6]`.  
BB offsets for every car can be found in this PDF in the appendix 4: <https://assettocorsamods.net/attachments/accsharedmemorydocumentationv1-8-12-pdf.7992/>.  


It is of course possible to use both functionalities at the same time, for example, take a look at my Ferrari 296 GT3 at SPA setup:  
`acc-test.exe (8 0x61 80) (25 0x62) (76 0x60) (92 0x62) [0x63 0x65 -5] [92 54.0] [0 50.0] [8 56.4]`
