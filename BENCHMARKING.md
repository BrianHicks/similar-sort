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

Well, not that then.

## Without `strsim`

Maybe `strsim` is doing something inefficient?
What if we try, say, `levenshtein`, which appears to operate on strings directly instead?

```
$ hyperfine './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     715.6 ms ±   6.9 ms    [User: 4.483 s, System: 0.033 s]
  Range (min … max):   705.7 ms … 725.5 ms    10 runs
```

So, no to that too!

## Removing arg parsing overhead?

What if it's creating that big Clap struct that's causing problems?
Let's give structopt a try (and then move to deriving from Clap once 3.0.0 is finally released.)

```
$ hyperfine './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     667.5 ms ±   7.1 ms    [User: 4.158 s, System: 0.031 s]
  Range (min … max):   658.3 ms … 678.2 ms    10 runs
```

Ok, seems fine!

## Bump allocator

What if deallocation is the problem?
We don't do anything fancy in `Drop` other than flushing the final output... let's try!
(Using [bump_alloc](https://crates.io/crates/bump_alloc))

```
$ hyperfine './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):      1.351 s ±  0.030 s    [User: 5.809 s, System: 2.811 s]
  Range (min … max):    1.321 s …  1.406 s    10 runs
```

... well, no. Probably not a good idea.
