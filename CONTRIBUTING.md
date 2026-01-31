# Contributing to rustfetch

Thanks for your interest in contributing! Everyone is welcome and encouraged to contribute. Don't be scared to open a PR just to fix a typo or improve documentation!

## Documentation
- [How to add a feature](/docs/add_a_feature.md)

## Getting Started

1. Fork the repository
2. Create a branch for your changes (`git checkout -b my-feature`)
3. Make your changes
4. Test your changes (`cargo test` and run the binary)
5. Submit a pull request

**Planning a big change?** Open a discussion first so the community and the creators can chat about it.

## Code Style & Principles

### Style
- **Follow the existing code style!** Consistency makes a codebase easy on the eyes and on the mind.
- Run `cargo +nightly fmt` before committing

### Performance Matters
This is a system info tool that should be fast and lightweight. Keep these in mind:

- **Avoid allocations when possible** - reuse buffers, use references where you can
- **Skip unnecessary abstractions** - if simple code is clearer and faster, prefer that
- **Minimize system calls** - batch operations when possible, avoid `sys_refresh_all()` if you only need specific data
- **Keep dependencies minimal** - avoid pulling in large crates for small tasks

## Pull Requests

### Size
- **Keep PRs small and focused** - easier to review, faster to merge
- **Split PRs if they implement different things** - separate bug fixes from features, refactors from new functionality

### Description
**Explain WHY you are doing something, not what you did** - we can read the code to see what changed, but your PR description must explain:
- Why is this change needed?
- What problem does it solve?

### Example
**A good PR description explains everything thoroughly**: "Switched to `/proc/meminfo` instead of relying on Sysinfo to reduce memory allocations (saves around 2ms)"

## Questions?

Open an issue or discussion, we'll be happy to help!

Thanks for contributing!
