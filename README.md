findrs 

---

todo

- [ ] Add support for non-utf8 files

---

### A simple CLI to get file information, like grep

```
Search: use
Directory: src
Total of matches: 4

──────────────────────────────────────────────

File: src/lib.rs
Matches: 1

0  | use std::{ fs, path };

──────────────────────────────────────────────

File: src/main.rs
Matches: 3

0  | use std::env;
1  | use std::process;
2  | use findrs::{ Config, run };
```

---

### Installation: 

```
git clone https://github.com/nogw/findrs
cd findrs
cargo install --locked --path .
findrs
```

---

### Usage:

```
findrs <directory> <query> <filter>

directory: file or folder to search
query: word to search, if it is a sentence use " " i.e. "a b c"
filter: file extension to filter, i.e. 
  -f rs
  -f rs,ml,txt,md
```