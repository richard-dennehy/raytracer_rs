# Performance tuning

Performance tweaks/optimisations are listed below, in order of application, with a brief description of the change, and the performance impact relative to the previous performance change.

### Replace nested for loops in renderer with iterators
In preparation for parallelising hot loop, split canvas mutation from individual ray casting.

- Effort: moderate - strange borrow checker issues
- Performance impact: minimal

### Parallelise ray tracing using Rayon
Replace first renderer iterator with rayon `par_iter`

- Effort: minimal - add import, change two `into_iter`s to `into_par_iter`, add `Sync` bound to `Shape` trait
- Performance impact: reduced rendering time by 67 - 75%

### Configure build flags
Tweak build flags to potentially gain extra performance for free

- Enabled LTO; Negligible performance impact
- Abort on panics; Negligible performance impact
- Use native target CPU (in `~/.cargo/config`); Slight performance improvement (2 - 7%)