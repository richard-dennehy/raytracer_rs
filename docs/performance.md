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

### Parallelise rows of pixels rather than individual pixels
Create one task for each row of pixels, rather than one task per pixel, 
increasing execution time of individual tasks, but potentially reducing overall overhead of synchronising and coordinating parallel tasks

- Effort: small - requires deeper knowledge of Rayon API, but actual code changes are trivial
- Performance impact: significant regression - up to 23% increase in execution time

### Distinct type for normalised Vectors
Implemented for correctness reasons - appeared to improve performance, but within likely noise threshold 
(running benchmarks multiple times without any changes can change performance by up to 8% in extreme cases)

- Effort: moderate - a large number of Vectors across the codebase needed to be updated to Normals where appropriate
- Performance impact: improvement within noise threshold

### Avoid calculating reflection data multiple times
Calculate reflection data at most once per ray, instead of potentially once for reflection, and once again for reflectance.
Primarily implemented to avoid duplicated logic, and to simplify sections of the code (e.g. ray colour calculations), but also potentially avoids duplicating work.

- Effort: small - changes were localised to a small area of the codebase
- Performance impact: within noise threshold - only a small number of calculations have been removed, and it's possible the compiler was optimising the duplicated calculations out

### Rust 1.51
No noticeable change

### Use 1D/flat Vector for Canvas
Use flat Vector for Canvas as opposed to 2D nested Vectors, to reduce pointer indirection

- Effort: minimal - very localised change
- Performance impact: mild regression (~4% generally; 12% slowdown in empty scene) - possibly causing more cache misses, but this kind of thing can be very difficult to diagnose

### Draw directly to canvas in parallel
Note: more or less taken entirely from https://blog.adamchalmers.com/grids-1/

By providing a callback function to the canvas, the canvas can be iterated over in parallel directly and safely,
and therefore the overhead of the various intermediate data structures is removed

- Effort: minimal - implementation mostly taken from blog post
- Performance impact: ~67% reduced hot loop execution time, noting that the absolute time reduction is fairly low (~50ms),
 and the impact on more complicated scenes is less noticeable