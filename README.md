# Building

```shell
cargo buld --release
```

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
