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

If you don't have `nix`, you'll need to install a Rust compiler toolchain yourself and run `cargo build`.

[Isaec](https://github.com/isaec) has also put together [an AUR package](https://aur.archlinux.org/packages/similar-sort-git) if you're on Arch.

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

Assuming you're using tmux as your window manager, integration looks something like this:

```
define-command -docstring 'open files named similarly to the current buffer' open-similar %{
    tmux-terminal-horizontal sh -c %{
        set -euo pipefail
        FILE="$(git ls-files --others --cached --exclude-standard | similar-sort $1 | grep -v $1 | fzf --tiebreak index)"
        printf "evaluate-commands -client %s edit %s\n" "$2" "$FILE" | kak -p $3
    } -- %val{bufname} %val{client} %val{session}
}
```

I have this bound to `-` with `map global normal <minus> ': open-similar<ret>'`.

### Using as a Spellchecker

If you use [fish](https://fishshell.com/), [Isaec](https://github.com) has written [a spell-checker using similar-sort](https://gist.github.com/isaec/ebed80db75d826a77fd528e955db8670). Source reproduced below for convenience:

```fish
function near
    set selection (
        cat /usr/share/dict/american-english \
            | similar-sort -- (xclip -o) \
            # tiebreak=index to prefer LD sort order
            | fzf -i +m --tiebreak=index --preview "dict {}" --preview-window="right,80%,wrap,<60(wrap,up,45%)"
    )

    # only run xclip if something was selected (i.e. not if fzf was exited)
    if test "$status" -eq 0
        # using -loops 2 to wait until clipboard manager has grabbed
        # using -quiet puts xclip in foreground so we wait to exit
        echo "$selection" | xclip -r -sel clip -loops 2 -quiet
    end

    exit
end
```

## License

CC BY-SA 4.0.
See LICENSE in the root of the project for details.
