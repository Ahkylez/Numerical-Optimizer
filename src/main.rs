
// We could start with a quadradic minimization problem 
// (1/2)x^TPx+q^Tx + r

//use std::vec;


// Unconstrained minimization problems


//S = {x ∈ dom f | f (x) ≤ f (x(0))} 

// Apparently there is a one liner for doing this. will figure that out later. 

// the dot product is just multiplying the first term's of 
// each array together and summing. Has to be built in functions.
fn dot<const N: usize>(a: &[f64; N], b: &[f64; N]) -> f64 {
    let mut sum: f64 = 0.0; 
    for i in 0..N {
        sum += a[i] * b[i];  
    }
    sum
}

fn matrix_product<const N: usize>(matrix: &[[f64; N]; N], vector: &[f64; N]) -> [f64; N]{
    let mut result:[f64; N] = [0.0; N];
    for i in 0..N {
        result[i] = dot(&matrix[i], vector);
    }
    result
}

// (1/2)x^TPx+q^Tx + r
struct Quadratic<const N: usize> {
    p: [[f64; N]; N], // P ∈ S^n_+, Hard coded 2x2 for now since idk. Make non negative float type later. 
    q: [f64; N], // q ∈ R^ n, hard coded two for now. 
    r: f64, // r ∈ R 
}

impl<const N: usize> Quadratic<N> {
    fn eval(&self, x: &[f64; N]) -> f64 {
        0.5 * dot(x, &matrix_product(&self.p, x)) + dot(&self.q, x) + self.r
    }
}


fn main() {
    let f: Quadratic::<2> = Quadratic {
        p: [[2.0, 0.5], [0.5, 1.0]], // P ∈ S^n_+
        q: [3.0, -1.0],
        r: 10.0,
    };

    let x0: [f64; 2] = [1.0, 0.0];

    println!("{}",f.eval(&x0));



    //print_quadradic(f);

    // Starting point x_0, must lie in the domain of f. 
    //let x0: f64 = 1.0;

}

// fn print_matrix(a: [[f64; 2];2]){
//     for element in a {
//         println!("{a[element]}");
//     }
// }

// fn print_quadradic(f: Quadratic) {
//     // Quadradic should completely on stack since there are no vectors in it yet or ever. 
//     println!("(1/2) * ")

// }



// fn minimizing_sequence() {

// }

// fn gradient_descent() {
//     // Given a starting point x in domain of f
// }
