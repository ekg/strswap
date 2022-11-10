# strswap

Swap string patterns listed one per line in an input file

## building

Use `cargo` to build:

```bash
cargo install --force --path .
```

## usage

Given a set of patterns:

```txt
aa bb
ce do
xo ee
```

We can rewrite a file:

```txt
aaaace ao do cece xo xxxo
```

... like this:

```bash
strswap -s patterns.txt -i text.txt >rewritten.txt
# bbbbdo ao do dodo ee xxee
```

The patterns are applied in order given in the file.
