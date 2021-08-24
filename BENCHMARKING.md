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

## First naive attempt

Just getting something working:

```
$ hyperfine './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):      2.579 s ±  0.053 s    [User: 2.535 s, System: 0.017 s]
  Range (min … max):    2.537 s …  2.725 s    10 runs
```

So... much worse than my naive attempt in Go.
However, this version is also doing proper argument parsing and has a little nicer error handling.
Let's try and make it parallel!

## Rayon

Rayon is nice!
Simply substituting `sort_by_key` for `par_sort_by_key` makes this a ton faster.

```
$ hyperfine './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     609.6 ms ±   9.1 ms    [User: 3.748 s, System: 0.029 s]
  Range (min … max):   598.9 ms … 629.1 ms    10 runs
```

It's still twice as slow as the Go version, though!
Wow!

## Without error handling

There are not very many things that can go wrong in this program.
Let's just try unwrapping and panicking?

```
$ hyperfine './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     612.9 ms ±  15.0 ms    [User: 3.768 s, System: 0.030 s]
  Range (min … max):   595.7 ms … 640.2 ms    10 runs
```
