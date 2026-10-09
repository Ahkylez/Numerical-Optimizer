
// We could start with a quadradic minimization problem 
// (1/2)x^TPx+q^Tx + r

//use std::vec;


// Unconstrained minimization problems

//S = {x ∈ dom f | f (x) ≤ f (x(0))} 



struct Quadratic {
    p: [[f64; 2]; 2], // P ∈ S^n_+, Hard coded 2x2 for now since idk. Make non negative float type later. 
    q: [f64; 2], // q ∈ R^n, hard coded two for now. 
    r: f64, // r ∈ R 
}


fn main() {
    let f: Quadratic = Quadratic{
        p: [[1.0, 2.0], [1.1, 2.2]], // P ∈ S^n_+
        q: [3.0, -1.0],
        r: 10.0,
    };
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
