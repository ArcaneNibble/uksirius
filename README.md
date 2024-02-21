# modem experiment thing

> \[U.S. Robotics Corporation\] is a reference to the fictional company U.S. Robots and Mechanical Men which featured prominently in the works of Isaac Asimov. [ref](https://en.wikipedia.org/wiki/USRobotics)

> \[Sirius Cybernetics Corporation\] is not known for the quality of their products, and almost all of their known inventions are faulty. [ref](https://hitchhikers.fandom.com/wiki/Sirius_Cybernetics_Corporation)

This project is an experiment to teach myself practical DSP algorithm implementation. The eventual goal is to implement a pure-software V.everything ISP modem (including V.90) that speaks SIP/RTP directly at a software PBX and ATA (no ISDN or legacy telco tech required).

Currently only V.21 and V.23 (AFSK) work. Automode and call supervision all need a complete rewrite.

V.8 works, but the modem being tested with doesn't negotiate V.21/V.23 via V.8.
