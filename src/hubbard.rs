use trexio;
use ndarray::*;

pub struct HubbardModel {
    ao_num: usize,
    ao_1e: Array2<f64>,
    ao_overlap: Array2<f64>,
    ao_2e: Vec<(usize,usize,usize,usize,f64)>,
    ao_2e_cholesky: Vec<(usize,usize,usize,f64)>,
}

impl HubbardModel {
    pub fn new(n: usize, m: usize, t: f64, u: f64) -> Self {


        // Build lattice
        let ao_num = n*m;

        let mut ao_1e = Array2::zeros( (ao_num,ao_num) );
        let mut ao_2e = vec![];
        let mut ao_2e_cholesky = vec![];

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
                ao_1e[[ij,kl]] -= t;
            }
            ao_1e[[ij,ij]] = 0.0;
            ao_2e.push( (ij,ij,ij,ij,u) );
            ao_2e_cholesky.push( (ij,ij,ij,u.sqrt()) );
          }
        }

        let ao_overlap = Array2::eye( ao_num );

        Self { ao_num, ao_1e, ao_2e, ao_overlap, ao_2e_cholesky }
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

        file.write_electron_up_num((self.ao_num/2).try_into().unwrap()).unwrap();
        file.write_electron_dn_num((self.ao_num/2).try_into().unwrap()).unwrap();
        file.write_nucleus_repulsion(0.0).unwrap();
        file.write_ao_num(self.ao_num).unwrap();
        file.write_ao_1e_int_overlap(&self.ao_overlap.flatten().to_vec()).unwrap();
        file.write_ao_1e_int_core_hamiltonian(&self.ao_1e.flatten().to_vec()).unwrap();
        file.write_ao_2e_int_eri(0, &self.ao_2e).unwrap();
        file.write_ao_2e_int_eri_cholesky_num(self.ao_2e_cholesky.len()).unwrap();
        file.write_ao_2e_int_eri_cholesky(0, &self.ao_2e_cholesky).unwrap();
    }
}


