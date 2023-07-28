# rgb: MB2 Rust external RGB demo
Bart Massey 2023

This MicroBit v2 program cycles an external RGB connected to
GPIO pins. It cycles through eight states in the order

    R, G, B, Y, C, M, W, -

at 0.5s per step.

The wires of the LED should be connected to the MB2
according to the following table. LED is the LED wire to
connect to. R is a current limit resistor value in ohms. MB2
is the pin on the MB2 edge connector to connect the other
end of the resistor to. The cathode of the LED should be
connect to ground. mA is the current used by that component
in milliamps.

    LED        R    MB2  mA
    
    Red        120  P08  11.0
    Green      390  P09  2.32
    Blue       120  P16  5.36

The resistor values were chosen to give roughly equal
brightness to all component colors.

The schematic looks like this.

![RGB LED hookup schematic](rgb-led-hookup.png)

The breadboard looks like this.

![RGB breadboard](rgb-breadboard.jpg)

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
