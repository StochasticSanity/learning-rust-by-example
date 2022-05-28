# Diff output
```rust
5,8c5,12
< trait Contains<A, B> {
<     fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
<     fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
<     fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
---
> trait Contains {
>     // Define generic types here which methods will be able to utilize.
>     type A;
>     type B;
> 
>     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
>     fn first(&self) -> i32;
>     fn last(&self) -> i32;
11,12c15,22
< impl Contains<i32, i32> for Container {
<     // True if the numbers stored are equal.
---
> impl Contains for Container {
>     // Specify what types `A` and `B` are. If the `input` type
>     // is `Container(i32, i32)`, the `output` types are determined
>     // as `i32` and `i32`.
>     type A = i32;
>     type B = i32;
> 
>     // `&Self::A` and `&Self::B` are also valid here.
16d25
< 
24,27c33
< // `C` contains `A` and `B`. In light of that, having to express `A` and
< // `B` again is a nuisance.
< fn difference<A, B, C>(container: &C) -> i32 where
<     C: Contains<A, B> {
---
> fn difference<C: Contains>(container: &C) -> i32 {
42c48
< 
---
>     
```