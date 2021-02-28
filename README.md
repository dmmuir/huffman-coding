# Huff

Huff is a simple cli compression utilty backed by the huffman-coding technique. This project was for learning purposes only. Use one of the many better compression tools.

## Installation

```sh
cargo install --git https://github.com/dmmuir/huffman-coding.git
```

## Usage

### Encode/Compress:

```sh
huff -f <filename>
```

Compresses the file as `<filename>.huff`.


### Decode/Decompress:

```sh
huff -df <filename>.huff
```

### Get Statistics on the file compression

#### Getting stats on an already compressed file:

```sh
huff stats <compressed-filename>
```

#### Getting stats without compressing:

```sh
huff -sf <filename>
```

### Pipes

Huff works with stdin/stdout.

```sh
echo aaaaaaaaabbbbbbbbb | huff | hexyl
```

Returns:
```sh
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 00 00 00 00 00 00 00 03 ┊ 00 00 00 00 00 00 00 1d │0000000•┊0000000•│
│00000010│ 08 0a 62 61 01 09 09 ff ┊ 01 00 10                │•_ba•__×┊•0•     │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
```
See [hexyl](https://github.com/sharkdp/hexyl/tree/master/src).


Or

```sh
echo aaaaaaaaabbbbbbbbb | huff -s 
```

Returns:
```sh
Compression ratio: 9/19; 52.63%
Dictionary stats:
Tokens:	3
Hits size:	3
Total bytes:	6
Dictionary contents:
+---+-----------+-------+------+------+
| # | Character | Count | Code | Bits |
+---+-----------+-------+------+------+
| 0 | "a"       | 9     | 0    | 1    |
+---+-----------+-------+------+------+
| 1 | "b"       | 9     | 11   | 2    |
+---+-----------+-------+------+------+
| 2 | "\n"      | 1     | 10   | 2    |
+---+-----------+-------+------+------+
```

