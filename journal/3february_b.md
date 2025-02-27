[<= Return to index](./index.md)

# Progress Report B \- February 2025

* Attempted to change project name
  * error: app_hide.toml': No such file or directory (os error 2)
    * Resolved this error, got another about not being able to reach localhost. Decided that I wanted a refresher on the project creation process anyway.
  * New project with correct name, pushed to repo and updated its name aswell.

* Arduino setup took longer than I would've liked
  * Fedora permission issue
  * Lack of obvious documentation
    * RGB LED is pin 48...

* Created a custom [arduino script](../arduino/SerialRGB/SerialRGB.ino) that allows me to change the built-in RGB NeoPixel LED via serial command.

* Struggled for an hour trying to get my rust script to communicate though serial using '[serialport-rs](https://github.com/serialport/serialport-rs)'. Yet to resolve this.
   * Throwing in the towel for the night.
     * Tired of banging my head against this, and it's late.
     * Documentation is very unhelpful.
     * Will need to clone the examples repo and see what I'm missing...

* Came back after resting and took another look at my problem
  * Realized that another library exists that is better for my usecase
    * [tokio-serial](https://github.com/berkowski/tokio-serial) is an async serialport lib that uses '[tokio](https://docs.rs/tokio/latest/tokio/)', the rust async library that Tauri uses.
  * 'tokio-serial' has better documentation and began to work almost immediately.
    * Able to send the color codes as bytes with hard-coded port and baudrate.
* Now that I had a working rust backend, I had to figure out how to communicate with it on the frontend
  * This is the moment where my lack of React knowledge comes majorly into play.
  * Sent a hardcoded color code string as a command parameter without too much fiddling.
  * I made the interesting decision to utilize the [react-color](https://casesandberg.github.io/react-color) package to allow for myself to move around the color on a color picker for this 'Simple Test'.
    * Had to learn about [React Components](https://www.w3schools.com/react/react_components.asp) to implement it.
    * Technically it "works" but there is some kind of issue with the way it is being rendered
      * There is a lag and the cursor is the text selection cursor when it should be the pointer
      * Realized that this problem was not a priority, as it is not relevant to the end goal.

### Mission Status: Successful, but slightly jank
