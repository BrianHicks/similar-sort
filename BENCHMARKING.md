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

## Unstable Sort

Looking at the Go implementation again, it looks like I used an unstable sort instead of a stable one.
OK, that's fine, we'll grab that speedup:

```
hyperfine './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     502.2 ms ±   6.6 ms    [User: 660.1 ms, System: 12.7 ms]
  Range (min … max):   491.5 ms … 513.2 ms    10 runs
```

And quite a speedup it is!
About 150ms over the previous improvement.

## Precalculate sizes

This is so much of a bigger result that I wonder if we're doing more work in parallel than we really need to?
What if we compute the distances in a `map` instead of doing it in the parallel code?

```
$ hyperfine './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     170.1 ms ±   4.5 ms    [User: 158.5 ms, System: 12.6 ms]
  Range (min … max):   163.5 ms … 181.6 ms    16 runs
```

Yay!
That finally gave us the result we wanted this whole time!
It's way faster!

A real comparison:

```
$ hyperfine './result/bin/similar-sort define < /usr/share/dict/words' './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./result/bin/similar-sort define < /usr/share/dict/words
  Time (mean ± σ):     287.8 ms ±   5.1 ms    [User: 254.8 ms, System: 75.3 ms]
  Range (min … max):   282.3 ms … 296.2 ms    10 runs

Benchmark #2: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     165.8 ms ±   7.4 ms    [User: 154.5 ms, System: 12.2 ms]
  Range (min … max):   155.8 ms … 186.2 ms    15 runs

Summary
  './target/release/similar-sort benchmark < /usr/share/dict/words' ran
    1.74 ± 0.08 times faster than './result/bin/similar-sort define < /usr/share/dict/words'
```

## Calculating sizes in parallel

What if we calculated the size in parallel?
Could we get it even faster?

```
hyperfine './result/bin/similar-sort define < /usr/share/dict/words' './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./result/bin/similar-sort define < /usr/share/dict/words
  Time (mean ± σ):     295.0 ms ±   5.6 ms    [User: 259.3 ms, System: 76.5 ms]
  Range (min … max):   287.4 ms … 305.2 ms    10 runs

Benchmark #2: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     153.5 ms ±   3.4 ms    [User: 143.0 ms, System: 11.0 ms]
  Range (min … max):   147.5 ms … 163.0 ms    19 runs

Summary
  './target/release/similar-sort benchmark < /usr/share/dict/words' ran
    1.92 ± 0.06 times faster than './result/bin/similar-sort define < /usr/share/dict/words'
```

Yep!

## Bump Allocation (again)

Let's try the bump allocator again... it seemed like a fluke that it produced such a severe performance degradation.

```
$ hyperfine './result/bin/similar-sort define < /usr/share/dict/words' './target/release/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./result/bin/similar-sort define < /usr/share/dict/words
  Time (mean ± σ):     286.2 ms ±   5.0 ms    [User: 251.4 ms, System: 72.5 ms]
  Range (min … max):   280.8 ms … 298.4 ms    10 runs

Benchmark #2: ./target/release/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     111.3 ms ±   3.4 ms    [User: 94.7 ms, System: 16.0 ms]
  Range (min … max):   104.1 ms … 118.5 ms    24 runs

Summary
  './target/release/similar-sort benchmark < /usr/share/dict/words' ran
    2.57 ± 0.09 times faster than './result/bin/similar-sort define < /usr/share/dict/words'
```

That seems more like what I'd expect!

## Coda

2.57 times faster for an hour and a half of work is pretty good!
Next time I'm running a Linux machine, maybe I'll try some of the fancier Linux-only Rust performance tools; maybe there's more to be gained here!

Once I get a `naersk` build with all the optimizations enabled, here's the final result:

```
$ hyperfine './result/bin/similar-sort define < /usr/share/dict/words' './go-result/bin/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./result/bin/similar-sort define < /usr/share/dict/words
  Time (mean ± σ):      99.1 ms ±   2.3 ms    [User: 83.7 ms, System: 16.0 ms]
  Range (min … max):    95.9 ms … 104.6 ms    28 runs

Benchmark #2: ./go-result/bin/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     304.4 ms ±   4.4 ms    [User: 269.9 ms, System: 77.1 ms]
  Range (min … max):   299.9 ms … 313.8 ms    10 runs

Summary
  './result/bin/similar-sort define < /usr/share/dict/words' ran
    3.07 ± 0.08 times faster than './go-result/bin/similar-sort benchmark < /usr/share/dict/words'
```

## Coda II

Well, turns out I forgot to have the program calculate the edit distance in parallel.
It's even faster now!

```
$ hyperfine -L impl iter,par_iter './{impl}/bin/similar-sort benchmark < /usr/share/dict/words'
Benchmark #1: ./iter/bin/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):     118.6 ms ±   2.2 ms    [User: 102.4 ms, System: 15.7 ms]
  Range (min … max):   113.9 ms … 123.3 ms    24 runs

Benchmark #2: ./par_iter/bin/similar-sort benchmark < /usr/share/dict/words
  Time (mean ± σ):      91.7 ms ±   2.1 ms    [User: 247.9 ms, System: 113.5 ms]
  Range (min … max):    88.3 ms …  95.6 ms    31 runs

Summary
  './par_iter/bin/similar-sort benchmark < /usr/share/dict/words' ran
    1.29 ± 0.04 times faster than './iter/bin/similar-sort benchmark < /usr/share/dict/words'
```
