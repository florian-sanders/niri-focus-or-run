# niri-focus-or-run

Make [niri](https://github.com/niri-wm/niri) focus the next window matching the given program name.
If no active windows exists, runs the program instead.

My first `rust` program, mostly a pretext to learn the language!

## How to use

```kdl
// config.kdl - your niri config file
bind {
  Mod+ampersand { spawn "niri-focus-or-run" "kitty"; }
  Mod+eacute { spawn "niri-focus-or-run" "firefox"; }
}
```
