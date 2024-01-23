# SIMD benchmarks
Attempted to use SIMD instructions for the Vec3 struct.

Note that the data type used is now 3 f32:s (4 are used, but one is ignored)
instead of 3 f64:s. This could have led to differences in performance that
affect the results.

All measurements show the amount of time it takes to render a 500x281 image with
1000 samples. The image was rendered three times and the average of the times is
presented here.

## Before changes (12f8c61)
Before any SIMD instructions were implemented.

Average time: 32.93s

## Partial implementation (d9f7c0f)
The Vec3 struct was changed from holding three `f64`:s to instead holding a `__m128`.
Note that this means it holds 4 `f32`:s instead of `f64`:s. This change of data type
might have impacted the measurements. Although all methods still take and return `f64`:s
as input/output, so all data types are casted before being returned. So the remainder of
the raytracer code still uses `f64`:s, including method implementations for `Vec3`.

To be clear, this commit only replaced the storage of the `Vec3` struct with an `__m128`.
All operations, like vector addition, multiplication, dot product, etc. were still calculated
"komponentvis" by extracting the x, y, z components from the `__m128`, operating on them, and
creating a new `Vec3` instance. The hypothesis for this commit was therefore that it would
yield slower results as time has to be spent converting between `__m128` and `f64` components
and back.

To my surprise, this commit resulted in a dramatic decrease in render time. I am not fully
sure why since it does things basically the same as before, but with extra conversion steps.
Either way, I was interested to see what the performance would look like when I fully
implement SIMD for all operations.

Average time: 18.73s

## Full implementation (8de7e57)
The sad part was that the more I implemented using SIMD instructions, the worse performance
got. I would have guessed that implementing vector operations would be when SIMD really
could shine, where vector addition, multiplication and dot products all are one instruction
as supposed to many. Unfortunately, the more operations I converted to SIMD the slower build
times got.

I still don't quite understand why this is. It makes little sense to me that unpacking
`__m128`:s into 4 `f32` each, taking the correct ones, adding them, and creating a new
`__m128` out of the result could be faster than using one `_mm_add_ps` instruction. I
will have to take a look at the generated machine code to really see what's happening.
Maybe the rust compiler is able to optimize the previous code in some way, and needs to
add other code when using the second approach.

The resulting time is very similar to the original time, before SIMD was introduced, which
is a disappointing and confusing result. Since the partial commit did result in a performance
improvement, reverting back to that commit to benefit from the performance is still possible,
even if I can't explain why it's faster.

Average time: 32.04s

# alvinw-raytracer
A ray tracer implemented in Rust with help from [Ray Tracing in One Weekend](https://raytracing.github.io/).

Also has triangles and some simple lighting. Shadows are only really visible when everything is dark though.

Runs on the CPU and renders PNG images.

![Example image](./img/example1.png)

<small>A 500x281 (16:9) render with 100 samples per pixel. Took just over 11 minutes to render.</small>

![Example image 2](./img/example2.png)

<small>A 500x281 (16:9) render with 1000 samples per pixel. Took 32 seconds to render. When it's dark shadows are easier to see.</small>