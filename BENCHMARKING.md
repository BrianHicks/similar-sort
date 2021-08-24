# Benchmarking

This program started out life as a fairly simple Go program that just runs everything in a single thread.
I'm rewriting it in Rust for learning, but also because I think Rayon might let me make this problem parallelizable!

Here's an initial benchmark on a `/usr/share/dict/words` of 235,886 lines:

```
$ hyperfine './result/bin/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./result/bin/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     307.4 ms ±   6.0 ms    [User: 273.5 ms, System: 75.5 ms]
  Range (min … max):   298.3 ms … 317.2 ms    10 runs
```

Let's see if we can get any faster than that!
