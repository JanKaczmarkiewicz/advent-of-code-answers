Part 2

Here I have to find at which button press the `rx` module will get low signal.

`rx` module single source of signal: `xn` which is _Conjunction_ module. I checked manually that has four independent processes providing input to it. That means when this four processes output high signal at the same time `rx` will recive low lignal (what I am looking for). Every input module to `xn` will do it once per button press - So I can treat it as a single event for button click.

I can analyze each process separately since they are not dependent on each other. I can look for some periodicity to see when produces high signal eg (5, 7, 3, 2). Then the possible conjunction will be at 5 * 7 * 3 * 2. 

How to detect the cycle?
For each output:
Detect when output produced high signal.

If the process is in already recorded state then their interation distance is the periodicity + nr of clicks to get to the first accurence.