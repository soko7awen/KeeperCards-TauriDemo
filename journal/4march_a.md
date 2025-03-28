[<= Return to index](./index.md)

# Progress Report A \- March 2025

* Finally! After banging my head against this problem for hours, and a little bit of advice from both two AI's I was able to achieve read **and** write simultaneously and it's awesome.
  * Had to set the serialport not be exclusive
  * Had to properly utilize the async library
    * Created a 'state' variable that is checked every serial_poll
  * The serial_setup() function runs at startup
    * Checks for an argument to set as the tty for serial, otherwise defaults to OS-specifc default tty
    * Sets non-exclusivity
    * Initializes 'state' and 'guard'
      * State is for storing the data that is received
      * Guard holds off the others until done writing

* I also decided it would be worthwhile to add an LED fade-out
  * This took an embarrassing percentage of my work time today
  * It looks good though!

* Next, I want to show the current color on the black div on my page.

