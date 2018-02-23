# Building

```shell
cargo buld --release
```

# Running

```shell
cocotte $(pwd)
```

# Fish integration

In your `fish_prompt` configuration file

```shell
function fish_prompt

# [....]

  if test (command --search cocotte)
    cocotte (pwd)
  end
end
```
