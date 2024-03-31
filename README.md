# 1BRC in Rust

For background, please check out the original project over [here](https://github.com/gunnarmorling/1brc)

This project is to tackle the 1 billion row challenge, aiming to efficiently process and aggregate data from a large dataset consisting of one billion rows.

Although the original challenge is in Java, I wanted to have a go at completing the challenge in rust so I could get some exposure to the profiling tools available in rust. I am already familiar with the profiling tools in Java such as JFRs, heap dumps and async profiles and wanted to know how I could apply this to rust. 

## Profiling
The profiling tool of choice i'm using throughout this project is [flamegraph](https://github.com/flamegraph-rs/flamegraph) as I am comfortable with flamegraphs and async profiles. The original idea was to do this project completely without external libraries but rust makes it quite difficult where most unsafe code requires you to pull in an external crate thus, ill only be pulling in external crates when I am physically unable to access a lower level API because the rust standard library is stopping me.

The profiling directory has all the flamegraphs I have accumulated while testing so if you would like to see how I have made progress in my solution, please take a look at the flamegraphs.

## Config
Place a config.toml in the root of this directory and use the following config params:
```toml
[myapp]
mode = "single_thread/multi_thread/rayon"
```
To switch between modes, keep the mode you want to run in config.toml

## Conclusions
I'm going to stop here for now. The benchmarks taken dont include loading config params as thats something I added to make switching between solutions easier. The overhead they add to the app I havent included in the benchmarks and only start the timer before any of the processing code beings. To make this more fair, I open the file and build the hashmap after starting the timer so the only variable not accounted for in my benchmarks is the time taken to load config.

For now, this is my "completed" solution until I figure out a way to make it run faster (maybe spawning the threads asynchronously and reimplementing the key hashing and float parsing. Could then make the local_maps vector a channel/queue that I know has been populated once the threads have finished executing). I wasnt able to create the whole measurements.txt file as the JVM OOM when trying to create the file so on my machine for a 10000000 file these are my results:
- Single threaded: 6600 ms (6 seconds)
- Multithreaded: 1400 ms (1.4 seconds)
- Multithreaded (using rayon): 1400 ms (1.4 seconds)

Results when profiling:
- Single threaded: 810 ms (0.81 seconds)
- Multithreaded: 180 ms (0.18 seconds)
- Multithreaded (using rayon): 180 ms (0.18 seconds)

Not entirely sure why profiling with flamegraph saved a considerable amount of seconds on execution time but think it might be performing some magic behind the scenes to get the application to run faster.
