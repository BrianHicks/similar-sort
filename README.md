# Similar Sort

This is a small Go program that will:

1. take a reference string as the first argument
2. and a list of candidate strings in stdin
3. and output the candidates sorted according to their edit distance from the reference, lowest first.

"What use is this?" you may ask!
Well!
It turns out to be really useful to do fuzzy file finding a large project.

When I am in some filesystem hierarchy and I trigger my fuzzy-finder, I want to see sibling files before I see similarly-named files further away.
I also want to match on test files pretty easily.
Say I have this project structure:

```
example
└── src
    ├── Main.elm
    └── Page
        └── Learn
            └── Home
                ├── Main.elm
                └── View.elm
```

If I am in `src/Page/Learn/Home/View.elm` and I want to get to the sibling file `Main.elm`, the default `fzf` config shows me `src/Main.elm` first.
That's not what I wanted!

But if I sort the files instead by piping them through `similar-sort src/Page/Learn/Home/View.elm`, the sibling file will show up first.
This works surprisingly well; I really like it!

It could probably perform a *little* better by doing some heuristic based on equivalent file structure except for the addition/removal of "tests", "specs", etc, but I haven't bothered yet.

## Installing

If you have [`nix`](https://nixos.org) installed, clone this and type:

```sh
nix-env -if .
```

After this, the `similar-sort` binary should be available on your `PATH`.

If you don't have `nix`, you'll need to install the `go` compiler yourself and run `go build similar-sort.go`.
The project has no external dependencies and should result in a static binary you can put wherever.

### Adding to Vim

Add this to your vim config:

```vim
nnoremap <silent> <C-t> :call fzf#run(fzf#wrap({
  \ "source": "git ls-files --others --cached --exclude-standard \| similar-sort " . @% . " \| grep -v " . @%,
  \ "sink": "edit",
  \ "options": "--tiebreak index"
  \ }))<CR>
```

(You'll need `fzf` and `fzf.vim` installed.)
This will bind ctrl-t to the fuzzy finder.
When you select a match, it will open in the current pane.

If you want to split or vsplit, change `"sink": "edit"` to `"sink": "split"` or `"sink": "vsplit"`.
See the docs for `fzf#run` for more customization options.

### Adding to Kakoune

You can look in `dotfiles/kakoune.nix` in <https://git.bytes.zone/brian/dotfiles.nix> to see how to use in [kakoune](https://kakoune.org).
I use [connect.kak](https://github.com/alexherbo2/connect.kak) to spawn a terminal window with about the same command line as in the vim config above.
