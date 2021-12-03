findrs 

---

### a simple CLI to get file information, like grep

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

```
usage: findrs <directory> <query> <filter>

directory: file or folder to search
query: word to search, if it is a sentence use " " e.g "a b c"
filter: file extension to filter
```