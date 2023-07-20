use ndarray::Array1;
use ndarray::Array;


fn main() {
    let g = 1.0;
    let w = 0.1;
    let m = 1;
    let d = 0.0 * g;
    let mut c_op_list: Vec<f64> = Vec::new();
    let mut sm_list: Vec<f64> = Vec::new();
    let l = 2.0 * std::f64::consts::PI;
    let k = 1;
    let r = l;
    let rp = vec![0.0, 8.5, 17.3, 29.4, 36.1, 47.2, 54.0, 90.2, 56.9, 70.2];
    let I = vec![0.249, 0.25, 0.01, 0.01, 0.01, 0.01];
    let N = vec![1, 2, 3, 4, 5, 6, 7];
    let xobs = 100.2;

    let tlist: Array1<f64> = Array::linspace(0.0, 10.0, 500);
    println!("{:?}", tlist);

    let x1: f64 = 0.0;
    let x2: f64 = 0.468;
    let x3: f64 = 0.796;
    let x4: f64 = 1.03;
    let a1: f64 = 0.0;
    let a2: f64 = 0.43;
    let a3: f64 = 0.750;

    let wlist: Array1<f64> = Array::linspace(-3.0, 3.0, 1000);
    println!("{:?}", wlist);

    let H: f64 = 0.0;
    let E: f64 = w; // Assuming `w` is a previously defined variable
    let mut op_list: Vec<f64> = Vec::new();
}
