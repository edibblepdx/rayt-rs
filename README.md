# rayt-rs

**A multithreaded software ray tracer written in Rust.**

## Features
- [x] Multithreaded rendering using rayon
- [x] Scene loading from toml file
- [x] Scene Deserialization
- [ ] accelerated rendering
- [ ] mesh loading
- [ ] Shadows

## Samplers
- Single
- Random
- Stratified

## Materials
- Diffuse
- Lambertian
- Metal
- Normals

## Primitives
- Sphere

## Getting Started
```sh
git clone https://github.com/edibblepdx/rayt-rs.git && cd rayt-rs/examples
```
```toml
#scene1.toml
[camera]
aspect_ratio = 1.777
image_width = 800
position = [0.0, 0.0, 0.0]
look_at = [0.0, 0.0, -1.0]
up = [0.0, 1.0, 0.0]
max_depth = 50

[camera.sampler]
type = "stratified"
nx = 20
ny = 20

[[primitive.sphere]]
center = [0.0, -100.5, -1.0]
radius = 100.0
material_id = 1

[[primitive.sphere]]
center = [0.0, 0.0, -1.2]
radius = 0.5
material_id = 2

[[primitive.sphere]]
center = [-1.0, 0.0, -1.0]
radius = 0.5
material_id = 3

[[primitive.sphere]]
center = [1.0, 0.0, -1.0]
radius = 0.5
material_id = 4

[[material.lambertian]]
id = 1
albedo = [0.8, 0.8, 0.0]

[[material.lambertian]]
id = 2
albedo = [0.1, 0.2, 0.5]

[[material.metal]]
id = 3
albedo = [0.8, 0.8, 0.8]

[[material.metal]]
id = 4
albedo = [0.8, 0.6, 0.2]
```
```sh
cargo run --release --example main > img.ppm && magick img.ppm img.jpg
```
![img](https://github.com/user-attachments/assets/8699ffd4-24de-4041-b6d6-e54f62200b10)

## License

This project is licensed under the [MIT License][License].

[License]: ./LICENSE
