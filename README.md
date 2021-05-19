# Raytracer on MoonZoon 

Raytracer is implemented according to the tutorial [Ray Tracing in One Weekend](https://misterdanb.github.io/raytracinginrust/).

[MoonZoon](http://moonzoon.rs/) is a Rust Fullstack Framework.

---

## How to start it

1. Check you've installed [Rust](https://www.rust-lang.org/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/):
    ```bash
    rustc -V # rustc 1.52.1 (9bc8c42bb 2021-05-09)
    wasm-pack -V # wasm-pack 0.9.1
    ```
    - _Note:_ `wasm-pack` will be installed automatically in the future.

1. Install `mzoon` to `cargo_install_root`:
    ```bash
    cargo install mzoon --git https://github.com/MoonZoon/MoonZoon --rev a6f5070 --root cargo_install_root --locked
    ```
    - _Note:_ `cargo install mzoon` will be enough in the future. And there will be a faster way with pre-compiled binaries.

1. Build and run:
    ```bash
    cargo_install_root/bin/mzoon start
    ```
    - _Note_: The app is much faster when built in the release mode (`-r`).

---

## Problems in the tutorial

1. Missing `Vec3` impl:
    ```rust
    impl Mul<Vec3> for Vec3 {
        type Output = Vec3;

        fn mul(self, other: Vec3) -> Vec3 {
            Vec3 {
                e: [self[0] * other[0], self[1] * other[1], self[2] * other[2]]
            }
        }
    }
    ```

1. `r0 * (1.0 - r0)` corrected to `r0 + (1.0 - r0)`:
    ```rust
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
    ```

1. Change the value in `const MAX_DEPTH: u64 = 5;` to `50`. (This and the previous correction fix black _bubbles_). 

1. There are some other problems in the tutorial but all of them should be easily fixed by the Rust compiler suggestions.

---

## Notes

- The `random_scene` function has been moved to `scene.rs` and renamed to `scene`.

- The `ray_color` function has been moved to ray.rs` as `Ray::color`.

- `main.rs` rewritten to `lib.rs`.

- There is an alternative raytracer described in the [wasm-bindgen docs](https://rustwasm.github.io/wasm-bindgen/examples/raytrace.html). I assume the raytracer is based on this tutorial: [Writing a Raytracer in Rust](https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/).



