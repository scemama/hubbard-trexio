# hubbard-trexio

A command-line tool written in Rust that generates [TREXIO](https://trex-coe.github.io/trexio/) files
containing the one- and two-electron integrals for the **Hubbard model** on a rectangular lattice.

The Hubbard model Hamiltonian is:

$$H = -t \sum_{\langle i,j \rangle,\sigma} c_{i\sigma}^\dagger c_{j\sigma} + U \sum_i n_{i\uparrow} n_{i\downarrow}$$

where *t* is the hopping parameter (kinetic energy between neighboring sites), *U* is the
on-site Coulomb interaction, and the lattice uses periodic boundary conditions.

The output TREXIO file stores the integrals in a format that can be read by any quantum chemistry
code that supports the TREXIO library.

---

## Prerequisites

### 1. Rust toolchain

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Rust **1.85 or later** is required (this project uses `edition = "2024"`, stabilised in Rust 1.85.0, released February 2025).

### 2. TREXIO library

The tool tries to write an HDF5-backed TREXIO file first, and falls back to a plain-text backend
if HDF5 is unavailable.  For HDF5 support, install the TREXIO C library **and** the HDF5
development headers before building:

* **Ubuntu / Debian**

  ```bash
  sudo apt install libhdf5-dev
  ```

* **Fedora / RHEL**

  ```bash
  sudo dnf install hdf5-devel
  ```

* **macOS (Homebrew)**

  ```bash
  brew install hdf5
  ```

Then install the TREXIO library from source or your distribution's package manager:
  <https://github.com/TREX-CoE/trexio/releases>
  
If you skip the HDF5 library the tool will still work — it will write a plain-text TREXIO
directory instead of a single `.h5` file.

---

## Building

Clone the repository and compile with Cargo:

```bash
git clone https://github.com/scemama/hubbard-trexio.git
cd hubbard-trexio
cargo build --release
```

The compiled binary is placed at `target/release/hubbard-trexio`.

You can also install it into `~/.cargo/bin/` so it is available on your `PATH`:

```bash
cargo install --path .
```

---

## Usage

```
hubbard-trexio -n <n_sites_x> [-m <m_sites_y>] [-t <hopping>] [-u <interaction>] TREXIO_FILE
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `-n <n_sites_x>` | Number of sites along the x-axis (**required**) | — |
| `-m <m_sites_y>` | Number of sites along the y-axis | `1` |
| `-p <periodic>` | Apply perdiodic boundary conditions | `true` |
| `-t <hopping>` | Hopping parameter *t* | `1.0` |
| `-u <interaction>` | On-site interaction parameter *U* | `1.0` |
| `-u <interaction>` | On-site interaction parameter *U* | `1.0` |
| `TREXIO_FILE` | Output file path (**required**) | — |

The total number of sites (atomic orbitals) is `n × m`.  The tool writes:

* **Overlap matrix** – identity matrix (*S = I* for a site basis)
* **Core Hamiltonian** – the one-electron hopping integrals
* **Two-electron integrals** – on-site Coulomb repulsion (*U* terms only)
* **Cholesky decomposition** of the two-electron integrals
* **Electron numbers** – `n_up = n_down = n_sites / 2`

---

## Examples

### 1-D chain of 4 sites with default *t* and *U* with no periodic boundary conditions

```bash
hubbard-trexio -n 4 -p false chain_4.h5
```

### 2-D 2×3 lattice with custom hopping and interaction

```bash
hubbard-trexio -n 2 -m 3 -t 1.0 -u 2.0 2x3_t1_u2.h5
```

### 1-D chain of 6 sites with *U* = 4

```bash
hubbard-trexio -n 6 -u 4.0 chain_6_u4.h5
```

### Single site with *t* = 0.5 (uses default *U* = 1.0)

```bash
hubbard-trexio -n 1 -m 1 -t 0.5 single_site.h5
```

---

## Output

When run successfully the program prints a summary of the model parameters and the output
file path:

```
Hubbard Model Integrals

  Sites : 2 x 3
  Periodic : true
  t = 1
  U = 2

  File: 2x3_t1_u2.h5
```

The resulting TREXIO file can be read by any program that links against the TREXIO library
(e.g. [Quantum Package](https://github.com/QuantumPackage/qp2)).

---

## License

This project is licensed under the **GNU General Public License v3.0 or later** — see the
[LICENSE](LICENSE) file for the full text.
