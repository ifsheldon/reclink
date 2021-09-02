# Reclink
A tool for recursively hard-linking files.

## Usage
Suppose we have a directory of the below structure:
```
- a
  - b
    - c.txt
    - d.md
    - e
      f.md
      g.txt
```
Then if we want to recursively hard-link files in `a/b` in `a/bbb`, we can use `reclink a/b a/bbb`, then `a/bbb` will be created automatically, containing hardlinks to all files in `a/b`.

If we don't want to hardlink txt files, use `reclink a/b a/bbb --ignore_patterns="**/*.txt"`, in which `**` stands for any subdirectories and `*` for any strings. For details of patterns, please see [glob pattern](https://docs.rs/glob/0.3.0/glob/struct.Pattern.html).

## Note
Directories are not hard-linked but replicated with the same name since hardlinking directories are forbidden.
