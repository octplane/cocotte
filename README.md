Cocotte will colorize your iTerm tabs according to the parameter you pass. It is meant to work with your folder, giving a large hue range while being stable and more or less following the alphabetical order.

This is a working POC for now. So code is ugly and features suck.

# Building

```shell
cargo buld --release
```

# Configuration

- create a `.cocotterc.toml` in your home

Example:
```toml
blacklist = [
          "users",
          "pierrebaillet",
          ".",
          "Documents",
          "src",
          "datadog",
          "mine"
]
```

## Configuration keys

### blacklist

- String list of path segment that will be ignored by the coloring algorithm

# Running

```shell
cocotte $(pwd)
```

# Fish integration

## ssh tab color

```shell
function ssh
    cocotte echo $argv
    /usr/bin/ssh $argv
end
```

## cwd tab color

In your `fish_prompt` configuration file

```shell
function fish_prompt

# [....]

  if test (command --search cocotte)
    cocotte (pwd)
  end
end
```
