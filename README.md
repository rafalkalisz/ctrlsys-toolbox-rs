# Rust Control Systems Toolbox 
A Rust-based GUI app for modeling and analyzing LTI control systems in continuous and discrete time. The project is intended to support control system and digital signal processing (DSP) application development, especially in areas like filter design and transfer function analysis.

## Features
- Continuous-time transfer functions (S-domain)
- Discrete-time transfer functions (Z-domain)
- Transfer function discretization using bilinear transform 
- GUI for interactive system tuning using egui
- Pole-zero and Bode plotting
- Low-pass filter synthesis
- In development: Impulse response simulation, PID control, root locus and advanced filter design tools

## Usage
### Linux Binary
A prebuilt release for Linux is available under the Releases section. The binary includes a statically linked `OpenBLAS` implementation of `LAPACK` required for the `ndarray-linalg` crate.
No system-wide `LAPACK` installation is required to run the binary.

### Windows
A Windows release is planned. Currently, building on Windows requires switching the linear algebra backend to `Intel MKL`.
Refer to [ndarray-linalg](https://github.com/rust-ndarray/ndarray-linalg) for backend selection details.
```
# In Cargo.toml
[dependencies]
ndarray-linalg = { version = "...", features = ["intel-mkl-static"] }
```
### License
This project is licensed under the GNU General Public License v3.0 or later.
MIT-licensed third-party crates are used in accordance with their respective licenses.
