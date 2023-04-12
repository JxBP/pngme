# PNGme

A CLI tool to hide secret messages in PNG file.

## Description

This is my implementation of the PNGme project as described [here](https://picklenerd.github.io/pngme_book/)

## Usage

A chunk type is a 4 byte string containing only alphabetic ASCII characters.
The chunk type should be unique and not conflict with standard chunks (see the attached RFC under section 4.3.).
If there are multiple chunks matching the specified type, this program will always operate on the first one.

### Encode a secret into a PNG

Note that the program currently does not override any existing chunks when using this, but rather append a new one.
If OUT_PATH is not specified, then the input file will be overwritten.

```
pngme <PATH> encode <CHUNK_TYPE> <MESSAGE> [OUT_PATH]
```

### Decode a secret from a PNG

```
pngme <PATH> decode <CHUNK_TYPE>
```

### Remove a secret from a PNG

```
pngme <PATH> remove <CHUNK_TYPE>
```

### Print all chunks in a PNG

```
pngme <PATH> print
```

# WARNING!

This tool does **NOT** safely hide your secrets!
Your hidden message will **NOT** be encrypted and can be easily viewed using tools like `strings` or `hexdump`.
For the unsuspecting eye, the PNG file will look like any other though.

If you are looking to secure your files you probably want to encrypt them instead.
In that case, you should have a look at the [OpenPGP](https://www.openpgp.org/) standard and use a software like the [GNU Privacy Guard](https://gnupg.org/).
