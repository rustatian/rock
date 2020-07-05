# Rock 

![CI](https://github.com/spiral/rock/workflows/CI/badge.svg)

Parser for the golang `pprof` profile format with mimalloc (on Linux) allocator. Data passed to the `Rock` can be in the same `zip/pb.gz` archive
as produces pprof (by default stored on Linux in `$HOME/pprof/...`)

This library can be used as intergration with http server (for example) to continuously parse profiles.

To do that, use:

```rust
Buffer::decode(&mut Vec<u8>) -> Result<Profile, RockError>
```

`Profile` will contain fully parsed pprof profile.
