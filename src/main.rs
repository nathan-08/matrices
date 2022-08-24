trait Show {
    fn show(&self) -> String;
}

trait Dim {
    fn dim(&self) -> (usize, usize);
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
}

trait Mult {
    fn mult(&self, _: &Self) -> Option<Self> where Self: Sized;
}

struct Matrix {
    data: Vec<Vec<i32>>
}

impl Dim for Matrix {
    fn dim(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }
    fn rows(&self) -> usize {
        self.dim().0
    }
    fn cols(&self) -> usize {
        self.dim().1
    }
} 

impl Mult for Matrix {
    fn mult(&self, other: &Matrix) -> Option<Self> {
        if self.cols() != other.rows() {
            return None;
        }
        let num_rows = self.rows();
        let num_cols = other.cols();
        let mut res = Matrix {data: vec![vec![0; num_cols]; num_rows]};
        for i in 0..res.data.len() {
            for j in 0..res.data[i].len() {
                for k in 0..self.cols() {
                    res.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Some(res)
    }
}

impl Show for Matrix {
    fn show(&self) -> String {
        let mut s = String::new();
        for i in 0..self.data.len() {
            s.push_str("[");
            for j in 0..self.data[i].len() {
                s.push_str(&self.data[i][j].to_string());
                if j < self.data[i].len() - 1 {
                    s.push_str(", ");
                }
            }
            s.push_str("]");
            if i < self.data.len() - 1 {
                s.push_str("\n");
            }
        }
        s
    }
}

fn main() {
    let m1 = Matrix{ data: vec![vec![0,0,1],
                                vec![0,1,0],
                                vec![1,0,0]]};

    let m2 = Matrix{ data: vec![vec![1,1,1],
                                vec![2,2,2],
                                vec![3,3,3]]};
    let m3 = m1.mult(&m2).unwrap();
    println!("{}", m1.show());
    println!("");
    println!("{}", m2.show());
    println!("");
    println!("{}", m3.show());
}

