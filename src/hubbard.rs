use trexio;
use ndarray::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HubbardModel {
    n: usize,
    m: usize,
    t: f64,
    u: f64,

    mo_num: usize,
    mo_1e: Array2<f64>,
    mo_2e: Vec<(usize,usize,usize,usize,f64)>,
}

impl HubbardModel {
    pub fn new(n: usize, m: usize, t: f64, u: f64) -> Self {


        // Build lattice
        let mo_num = n*m;

        let mut mo_1e = Array2::zeros( (mo_num,mo_num) );
        let mut mo_2e = vec![];

        for i in 0..n {
          let up   = if i>0 {i-1} else {n-1};
          let down = if i<n-1 {i+1} else {0};
          for j in 0..m {
            let left  = if j>0 {j-1} else {m-1};
            let right = if j<m-1 {j+1} else {0};
            let v = [ (up,j), (down,j), (i, left), (i, right) ];
            let ij = i*m + j;
            for (k,l) in v {
                let kl = k*m + l;
                mo_1e[[ij,kl]] = -t;
            }
            mo_1e[[ij,ij]] = 0.0;
            mo_2e.push( (ij,ij,ij,ij,u) );
          }
        }


        Self { n, m, t, u,
        mo_num, mo_1e, mo_2e }
    }

    pub fn write(&self, trexio_filename: &str) {
        let file =
           match trexio::File::open(trexio_filename, 'w', trexio::BackEnd::Hdf5) {
               Ok(f) => f,
               _     => {
                   trexio::File::open(trexio_filename, 'w', trexio::BackEnd::Text)
                       .expect("Failed to open TREXIO file")
                   }
           };

        file.write_electron_up_num((self.mo_num/2).try_into().unwrap());
        file.write_electron_dn_num((self.mo_num/2).try_into().unwrap());
        file.write_mo_num(self.mo_num);
        file.write_mo_1e_int_core_hamiltonian(&self.mo_1e.flatten().to_vec());
        file.write_mo_2e_int_eri(0, &self.mo_2e);
    }
}


