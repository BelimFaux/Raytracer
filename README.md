# GFX - lab 3b

## Claim

- T1, T2, T3, T4, T5, T6, T7
- EF4, EF5, EF8, EF9, EF10
- B4, B6

## Tested Environments

Tested on

- EndeavourOS Linux x86_64 (Kernel: 6.14.9-arch1-1)
- Arch Linux x86_64 (Kernel: 6.14.9-arch1-1)

With

- Cargo/rustc 1.88.0 / 1.84.0

## Additional and general remarks

### Running

- The input file can be given via a commandline argument. So the program can be compiled and run with the following command:

```sh
cargo run --release -- scenes/example1.xml

# or

cargo build --release
./target/release/ray-tracer scenes/example1.xml
```

- For convenience I have included a Makefile that will compile the program (`all` will use release build and `debug` will use debug build) and run it with all the provided, as well as my custom input files (excluding `chess.xml` because it takes ages; for this run `make chess`)

- The program will save the resulting image files with the name specified in the input file in an `output` directory
  - The output directory can be changed using the `-o` (`--outdir`) flag
  - If this directory does not exist, the program will fail

- For all commandline options run the program with the `-h`/`--help` flag

### Effects

- (almost) all effects have a custom xml file to showcase the effect. They should be named pretty self-explanatory.

- Depth of Field
  - can be specified in the xml files as a subfield of the camera. It takes the focal length and the aperture size as parameters
  - `<depth_of_field focal_length=".." aperture=".." />`
  - Best if used with supersampling

- Spot light sources
  - can be specified in the xml files as in the description online

- Cook-Torrance model
  - can be specified in the xml files insted of the `phong` field in the material. It takes ambient and specular coefficients, as well as the material roughness
  - `<cook_torrance ka=".." ks=".." roughness=".." />`

- Animations
  - can be specified in the xml files by adding the `animated` field to the scene, which specifies the number of frames as well as the framerate
  - `<animated frames=".." fps=".." />`
  - for the objects to actually change between frames, you can specify endparameters for the objects. The program will linearly interpolate between start and end parameter for each frame
  - This is supported for spheres, where endposition and endradius can be specified, and julia sets where the endconstant can be specified
  - `<endposition x=".." y=".." z=".." />`

- Motion Blur
  - can not be completely specified in the xml files, but requires the `--blur` commandline flag
  - this requires the scene to be already animated (so some objects are moving)
  - this effect does not have a custom xml file. Use one for the animations and add the `--blur` flag

### Bonus

- Julia sets
  - Can be specified in the xml files similar to spheres and meshes. They take maximum iterations, epsilon, a position, and a constant (and an endconstant if animated)
  - They support only solid materials (how would you even texture map this?)

```xml
<julia_set max_iterations=".." epsilon="..">
    <position x=".." y=".." z=".." />
    <constant x=".." y=".." z=".." w=".." />
    <!-- material and transforms -->
</julia_set>
```

- Supersampling
  - Can be specified in the xml files as a field for the scene. It takes the number of samples
  - this has no dedicated custom xml file, but is instead used in some of the other effects (i.e. depth_of_field)
  - `<super_sampling samples=".." />`
