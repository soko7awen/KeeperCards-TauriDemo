[<= Return to index](./index.md)

# Progress Report A \- April 2025 (Final)

## Finishing things up...

* So! One last group showcase remains, and because of that I will lay out the steps to achieve a minimum viable demo for my concept. 
    * The most important thing to do is to physically craft the cards together to be able to showcase differentiation.
        * For now, I've decided to simply tie different gpio pins to ground via conductive foil lines on the back.
        * In the future, I would like to invest into a more appropriate conducting solution. Something that can handle being slid around on, some kind of foil tape.
        * I ended up having to just glue layers of aluminium foil on the back side of the cards, that ground the appropriate pin.
        * I used duct tape to keep the wires on the recieving pad, and soldered the wires directly to the arduino.
        * I used sharpie to mark the front of the cards for basic differentiation.
    * There was also one final software pain point, that I had yet to solve.
        * The details of this technological confusion are somewhat embarrasing.
        * I had been fiddling with the Rust -> React pipeline for hours, trying to find out why my event wasn't getting listened to succesfully on the front end.
        * On a random whim, I decided to check the console log in the webview console. Turns out, this is where all JS/TS console output is printed.
        * This confusion single-handedly set back my ability to achieve the loftier goals I had set out for myself.
        * Once I found out the main "problem" all I had to do was implement the fundamental feature I had been delaying.
    * The feature that I could now implement was the ability to reflect changes detected via rust's serialmonitor in the webview frontend.
        * I had to import the 'color' package, and reformat some of my payload code.
        * I used the 'color' package to brighten up the value to a much brighter color, as the arduino LED is very bright so that needs to be compensated for.

## Conclusive Thoughts

* While I am definitely not going to pretend I was able to achieve the lofty goals I had set out for myself, I am definitely reinvigorated having succesfully reached a ground-floor demo that I can expand upon.
* I am also really happy to have had the opportunity to explore both lower-level programming (Rust) as well as framework frontend development (React).
    * Despite my frustrations, I intend to continue to utilize this software stack. It may just be my stubborness, but I do really like the paradigm it was designed within.

### What's Next?

 * While I don't have much spare time as we close out this semester, I soon very much will. I intend to utilize my summer break to expand upon this demo and create a more appealing hardware showcase.
    * Specifically, I would love to look into my options for 3D printing, sublimation, conductivity, and embedding a storage medium into a PVC card.