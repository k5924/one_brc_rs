# 1BRC in Rust

For background, please check out the original project over [here](https://github.com/gunnarmorling/1brc)

This project is to tackle the 1 billion row challenge, aiming to efficiently process and aggregate data from a large dataset consisting of one billion rows.

Although the original challenge is in Java, I wanted to have a go at completing the challenge in rust so I could get some exposure to the profiling tools available in rust. I am already familiar with the profiling tools in Java such as JFRs, heap dumps and async profiles and wanted to know how I could apply this to rust. 

The profiling tool of choice i'm using throughout this project is [flamegraph](https://github.com/flamegraph-rs/flamegraph) as I am comfortable with flamegraphs and async profiles. The original idea was to do this project completely without external libraries but rust makes it quite difficult where most unsafe code requires you to pull in an external crate thus, ill only be pulling in external crates when I am physically unable to access a lower level API because the rust standard library is stopping me.

The profiling section has all the flamegraphs I have accumulated while testing so if you would like to see how I have made progress in my solution, please take a look at the flamegraphs.

## Conclusions
I'm going to stop here for now. I tried to make a multithreaded solution but the compiler has been fighting me about lifetimes for the file which I couldnt find a way around (would appreciate if anyone comes up with a solution, please feel free to open a PR). 

When using rayon for a quick "win" to see how fast my current solution would be running multithreaded, instead of running faster, in some scenarios it actually ran slower. Using flamegraph, lots of syscalls are being made which explains why CPU utilization jumps from 11% to 60% on my machine. 

For now, this is my "completed" single threaded solution until I figure out a way to make it run faster. I wasnt able to create the whole measurements.txt file as the JVM OOM when trying to create the file so on my machine for a 10000000 file these are my results:
- Single threaded: 7000 ms (7 seconds) with my 
- Multithreaded (using rayon): 14000 ms (14s)

Interestingly, I was able to hit 840ms (0.84 seconds) using flamegraph to profile my application so the results above are without accounting for any magic flamegraph might be doing behind the scenes to get my application to run faster. 
