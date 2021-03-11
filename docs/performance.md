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

### Sequential generation of hot loop index pairs
Generate (x, y) indexes of hot loop in sequence, then cast rays in parallel, as the overhead of forking and joining threads makes parallel iteration much slower.

- Effort: slight - needed to collect into a `Vec` rather than parallelising sequential iterator
- Performance impact: reduced loop iteration time by ~25%

Note: ideally would avoid allocating 2 `Vec`s and looping 3 times, but switching to parallel iterator and back again makes this difficult to avoid

### Precalculating transformation matrix inverse 
Calculate inverse eagerly and store it along with matrix data, to avoid having to recalculate it constantly

- Effort: moderate - required refactoring of `matrix` module to split Transform data structure and constructors, and underlying 4D Matrix and maths
- Performance impact: 5-10% reduction on trivial scenes, 15-20% on complex scenes

Note: storing normal matrix and inverse increases size of struct significantly, which may negatively impact performance

### Only store transformation matrix inverse
Only store inverted matrix in `Transform` data structure, and invert it back for operations that require the non-inverted matrix
e.g. multiplying Vectors/Points

This reduces the size of the type, which may improve cache utilisation and function calls

- Effort: moderate - required substantial effort to introduce strong property tests, then minor effort to change implementation details
- Performance impact: 4-8% reduction in execution time

Note: this change allows the Transform inverse function to no longer return an option, removing some branching. The performance impact of this additional change is negligible.