# 1BRC in Rust

For background, please check out the original project over (here)[https://github.com/gunnarmorling/1brc]

This project is to tackle the 1 billion row challenge, aiming to efficiently process and aggregate data from a large dataset consisting of one billion rows.

Although the original challenge is in Java, I wanted to have a go at completing the challenge in rust so I could get some exposure to the profiling tools available in rust. I am already familiar with the profiling tools in Java such as JFRs, heap dumps and async profiles and wanted to know how I could apply this to rust. 

The profiling tool of choice i'm using throughout this project is (flamegraph)[https://github.com/flamegraph-rs/flamegraph] as I am comfortable with flamegraphs and async profiles. The original idea was to do this project completely without external libraries but rust makes it quite difficult where most unsafe code requires you to pull in an external crate thus, ill only be pulling in external crates when I am physically unable to access a lower level API because the rust standard library is stopping me.

The profiling section has all the flamegraphs I have accumulated while testing so if you would like to see how I have made progress in my solution, please take a look at the flamegraphs.
