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

* Struggled for an hour trying to get my rust script to communicate though serial. Yet to resolve this.
   * Throwing in the towel for the night.
     * Tired of banging my head against this, and it's late.
     * Documentation is very unhelpful.
     * Will need to clone the examples repo and see what I'm missing...

