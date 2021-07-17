# watch
### A watch command actually integrating with your usr/bin/env
#### Background (dramatical presentation)
Once upon a time, a developer wanted to watch his thermal statistics using a shell command:
Frustratingly, he was soon to find that the orignal [psprocs-ng](https://gitlab.com/procps-ng/procps) actually uses the glib 'system(const char *)' call.
Annoyed by the limit of the '-x' option he set out to write his own watch implementation, which determines the
current shell dynamically, in the fancy trendy language of __Rust__.
#### TL;DR
__Watch implementation determining shell dynamically from enviroment__
### Building:
Run 
```
cargo build
```

or 

```
cargo run
```
