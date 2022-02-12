# C2Rust conversion

```bash
c2rust transpile compile_commands.json --emit-no-std --output-dir c2rust-gen --emit-modules --emit-build-files
```

```bash
c2rust refactor --cargo --rewrite-mode inplace convert_printfs convert_format_args remove_redundant_casts remove_redundant_let_types remove_null_terminator link_funcs sink_lets fix_unused_unsafe
```
