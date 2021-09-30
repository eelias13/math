use crate::linear_algebra::Vector;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Debug)]
pub struct Matrix {
    cols: usize,
    rows: usize,
    matrix_flatt: Vector,
    is_transpose: bool,
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.cols() == other.cols()
            && self.rows() == other.rows()
            && self.matrix_flatt() == other.matrix_flatt()
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.cols() {
            writeln!(f, "{}", self.col(i))?;
        }
        Ok(())
    }
}

impl Add for Matrix {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = self.clone();
        result.add_mat(&other);
        result
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, other: Self) {
        self.add_mat(&other);
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self.clone();
        result.sub_mat(&other);
        result
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, other: Self) {
        self.sub_mat(&other);
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = self.clone();
        result.mul_mat(&other);
        result
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, other: Self) {
        self.mul_mat(&other);
    }
}

impl Div for Matrix {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut result = self.clone();
        result.div_mat(&other);
        result
    }
}

impl DivAssign for Matrix {
    fn div_assign(&mut self, other: Self) {
        self.div_mat(&other);
    }
}

impl Matrix {
    /// converts 2d vec in to matrix
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
    /// ```
    /// crates matrix that looks like this:
    ///
    /// [3.0, 2.0, 4.0]
    /// [4.0, 5.0, 6.0]
    ///
    pub fn new(vec: Vec<Vec<f32>>) -> Self {
        let cols = vec.len();
        let rows = vec[0].len();

        let mut flatt: Vec<f32> = Vec::with_capacity(cols * rows);

        vec.iter().for_each(|col| {
            if col.len() != rows {
                panic!("wrong row shape expected {}, got {}", rows, col.len())
            }
            col.iter().for_each(|&x| flatt.push(x))
        });

        Self {
            cols: cols,
            rows: rows,
            matrix_flatt: Vector::new(flatt),
            is_transpose: false,
        }
    }

    /// returns the Matrix of the [outer product] with the vectors
    ///
    /// [outer product]:https://en.wikipedia.org/wiki/Outer_product
    ///
    ///  ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let vector1 = Vector::new(vec![2., 4., 3.]);
    /// let vector2 = Vector::new(vec![2., 7., 9.]);
    /// let matrix = Matrix::new_outer(&vector1,&vector2);
    /// assert_eq!(matrix, Matrix::new_flatt(vec![4.0, 14.0, 18.0, 8.0, 28.0, 36.0, 6.0, 21.0, 27.0], 3, 3));
    /// ```
    pub fn new_outer(vector1: &Vector, vector2: &Vector) -> Self {
        let mut vec = Vec::new();
        for i in 0..vector1.len() {
            let mut temp = Vec::new();
            for j in 0..vector2.len() {
                temp.push(vector1.index(i) * vector2.index(j));
            }
            vec.push(temp);
        }

        Self::new(vec)
    }

    /// generats a matrix from a 1D Vector
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let matrix = Matrix::new_flatt(vec![3., 2., 4., 4., 5., 6.], 2, 3);
    /// assert_eq!(matrix.matrix_flatt(), Vector::new(vec![3., 2., 4., 4., 5., 6.]));
    /// ```
    pub fn new_flatt(matrix_flatt: Vec<f32>, cols: usize, rows: usize) -> Self {
        if cols * rows != matrix_flatt.len() {
            panic!(
                "cols * rows = {} has to be the same len as the matrix_flatt = {}",
                cols * rows,
                matrix_flatt.len()
            );
        }

        Self {
            cols,
            rows,
            matrix_flatt: Vector::new(matrix_flatt),
            is_transpose: false,
        }
    }

    /// generates a matrix of size `cols` and `rows` with random values between 0 and 1
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let matrix = Matrix::new_rand(2, 3);
    /// assert_eq!(
    ///     matrix.matrix_flatt(),
    ///     Vector::new(vec![
    ///        0.69186187,
    ///        0.3494884,
    ///        0.23957491,
    ///        0.06540034,
    ///        0.5443042,
    ///        0.013656098,
    ///    ])
    /// );
    /// ```
    pub fn new_rand(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            matrix_flatt: Vector::new_rand(cols * rows),
            is_transpose: false,
        }
    }

    /// generates a matrix of size `cols` and `rows` with all values being 0.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let matrix = Matrix::new_zero(2, 3);
    /// assert_eq!(matrix.matrix_flatt(), Vector::new(vec![0., 0., 0., 0., 0., 0.]));
    /// ```
    pub fn new_zero(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            matrix_flatt: Vector::new_zero(cols * rows),
            is_transpose: false,
        }
    }

    /// getter for the internal matrix_flatt representation
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
    /// assert_eq!(matrix.matrix_flatt(), Vector::new(vec![2., 3., 5., 7., 1., 4.]));
    /// ```
    pub fn matrix_flatt(&self) -> Vector {
        if self.is_transpose {
            let mut matrix_flatt = Vec::with_capacity(self.cols * self.rows);
            for i in 0..self.rows {
                for val in self.col(i).vec() {
                    matrix_flatt.push(val);
                }
            }
            Vector::new(matrix_flatt)
        } else {
            self.matrix_flatt.clone()
        }
    }

    /// return index(row, col) from matrix
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
    /// assert_eq!(matrix.index(0, 1), 2.);
    /// ```
    pub fn index(&self, mut row: usize, mut col: usize) -> f32 {
        if self.is_transpose {
            let temp = row;
            row = col;
            col = temp;
        }

        if self.rows < row {
            panic!("index out of bounds max row {}", self.rows - 1)
        }
        if self.cols < col {
            panic!("index out of bounds max col {}", self.cols - 1)
        }

        let index = row * self.rows + col;
        self.matrix_flatt.index(index)
    }

    /// sets the value of the matrix at the specifide index row col
    ///
    /// ## Example
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let mut matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
    /// matrix.set_index(0, 1, 10.);
    /// assert_eq!(matrix.matrix_flatt(), Vector::new(vec![2.0, 10.0, 5.0, 7.0, 1.0, 4.0]));
    /// ```
    pub fn set_index(&mut self, mut row: usize, mut col: usize, val: f32) {
        if self.is_transpose {
            let temp = row;
            row = col;
            col = temp;
        }

        if self.rows < row + 1 {
            panic!("index out of bounds max row {}", self.rows - 1)
        }
        if self.cols < col + 1 {
            panic!("index out of bounds max col {}", self.cols - 1)
        }

        let index = row * self.rows + col;
        self.matrix_flatt.set_index(index, val);
    }

    /// return the length of the columns
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
    /// assert_eq!(matrix.cols(), 2);
    /// ```
    pub fn cols(&self) -> usize {
        if self.is_transpose {
            self.rows
        } else {
            self.cols
        }
    }

    /// return the length of the rows
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
    /// assert_eq!(matrix.rows(), 3);
    /// ```
    pub fn rows(&self) -> usize {
        if self.is_transpose {
            self.cols
        } else {
            self.rows
        }
    }

    /// return column from matrix
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
    /// assert_eq!(matrix.col(0), Vector::new(vec![3., 2., 4.]));
    /// ```
    pub fn col(&self, col: usize) -> Vector {
        if self.is_transpose {
            self.get_row(col)
        } else {
            self.get_col(col)
        }
    }

    /// return row from matrix
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
    /// assert_eq!(matrix.row(0), Vector::new(vec![3., 4.]));
    /// ```
    pub fn row(&self, row: usize) -> Vector {
        if self.is_transpose {
            self.get_col(row)
        } else {
            self.get_row(row)
        }
    }

    /// returns true if the matrix is a [square matrix]  
    ///
    /// that means if it has as much rows as cols
    ///
    /// [square matrix]:https://en.wikipedia.org/wiki/Square_matrix
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let matrix = Matrix::new(vec![vec![3., 2.], vec![4., 5.]]);
    /// assert_eq!(matrix.is_square(), true);
    /// ```
    pub fn is_square(&self) -> bool {
        self.cols() == self.rows()
    }

    /// getter for the transpose
    pub fn is_transpose(&self) -> bool {
        self.is_transpose
    }

    /// [transposes] matrix flips rows and cols
    ///
    /// [transposes]: https://en.wikipedia.org/wiki/Transpose
    pub fn transpose(&mut self) {
        self.is_transpose = !self.is_transpose;
    }

    /// multiplies each component from the matrix with a scalar value and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let mut matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
    /// matrix.mul_scalar(&2.);
    /// assert_eq!(
    ///     matrix,
    ///     Matrix::new(vec![
    ///         vec![2. * 2., 3. * 2., 5. * 2.],
    ///         vec![7. * 2., 1. * 2., 4. * 2.]
    ///     ])
    /// );
    /// ```
    pub fn mul_scalar(&mut self, scalar: &f32) {
        self.matrix_flatt.mul_scalar(scalar);
    }

    /// multiplies each component from the matrix with a scalar value and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let mut matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
    /// matrix.add_scalar(&2.);
    /// assert_eq!(
    ///     matrix,
    ///     Matrix::new(vec![
    ///         vec![2. + 2., 3. + 2., 5. + 2.],
    ///         vec![7. + 2., 1. + 2., 4. + 2.]
    ///     ])
    /// );
    /// ```
    pub fn add_scalar(&mut self, scalar: &f32) {
        self.matrix_flatt.add_scalar(scalar);
    }

    /// multiplies each component from the matrix with a scalar value and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let mut matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
    /// matrix.div_scalar(&2.);
    /// assert_eq!(
    ///     matrix,
    ///     Matrix::new(vec![
    ///         vec![2. / 2., 3. / 2., 5. / 2.],
    ///         vec![7. / 2., 1. / 2., 4. / 2.]
    ///     ])
    /// );
    /// ```
    pub fn div_scalar(&mut self, scalar: &f32) {
        self.matrix_flatt.div_scalar(scalar);
    }

    /// multiplies each component from the matrix with a scalar value and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let mut matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
    /// matrix.sub_scalar(&2.);
    /// assert_eq!(
    ///     matrix,
    ///     Matrix::new(vec![
    ///         vec![2. - 2., 3. - 2., 5. - 2.],
    ///         vec![7. - 2., 1. - 2., 4. - 2.]
    ///     ])
    /// );
    /// ```
    pub fn sub_scalar(&mut self, scalar: &f32) {
        self.matrix_flatt.sub_scalar(scalar);
    }

    /// computes the dot product between the vector and this matrix
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let matrix = Matrix::new(vec![vec![1., -1., 2.], vec![0., -3., 1.]]);
    /// assert_eq!(
    ///     matrix.dot_vec(&Vector::new(vec![2., 1., 0.])),
    ///     Vector::new(vec![1., -3.])
    /// );
    /// ```
    pub fn dot_vec(&self, vector: &Vector) -> Vector {
        let vec = vector.vec();
        check_vector(self, vector);

        let mut result: Vec<f32> = Vec::with_capacity(self.cols());
        for i in 0..self.cols() {
            result.push(
                self.col(i)
                    .vec()
                    .iter()
                    .enumerate()
                    .map(|(j, x)| vec[j] * x)
                    .sum(),
            );
        }
        Vector::new(result)
    }

    /// adds each component from the vector with the component of the other matrix and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let mut matrix = Matrix::new(vec![vec![2., -3., 1.], vec![2., 0., -1.]]);
    /// let vector = Vector::new(vec![2., 4., 6.]);
    /// matrix.add_vec(&vector);
    /// assert_eq!(
    ///     matrix,
    ///     Matrix::new(vec![vec![4.0, -3.0, 1.0], vec![6.0, 0.0, -1.0]])
    /// );
    /// ```
    /// note it panics if the matrices have not the same rows and cols
    pub fn add_vec(&mut self, vector: &Vector) {
        check_vector(self, vector);
        for row in 0..self.rows() - 1 {
            for col in 0..self.cols() - 1 {
                let val = self.index(row, col) + vector.index(row);
                self.set_index(row, col, val);
            }
        }
    }

    /// subtracts each component from the vector with the component of the other matrix and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let mut matrix = Matrix::new(vec![vec![2., -3., 1.], vec![2., 0., -1.]]);
    /// let vector = Vector::new(vec![2., 4., 6.]);
    /// matrix.sub_vec(&vector);
    /// assert_eq!(
    ///     matrix,
    ///     Matrix::new(vec![vec![0.0, -3.0, 1.0], vec![-2.0, 0.0, -1.0]])
    /// );
    /// ```
    /// note it panics if the matrices have not the same rows and cols
    pub fn sub_vec(&mut self, vector: &Vector) {
        check_vector(self, vector);
        for row in 0..self.rows() - 1 {
            for col in 0..self.cols() - 1 {
                let val = self.index(row, col) - vector.index(row);
                self.set_index(row, col, val);
            }
        }
    }

    /// multiplys each component from the vector with the component of the other matrix and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let mut matrix = Matrix::new(vec![vec![2., -3., 1.], vec![2., 0., -1.]]);
    /// let vector = Vector::new(vec![2., 4., 6.]);
    /// matrix.mul_vec(&vector);
    /// assert_eq!(
    ///     matrix,
    ///     Matrix::new(vec![vec![4.0, -3.0, 1.0], vec![8.0, 0.0, -1.0]])
    /// );
    /// ```
    /// note it panics if the matrices have not the same rows and cols
    pub fn mul_vec(&mut self, vector: &Vector) {
        check_vector(self, vector);
        for row in 0..self.rows() - 1 {
            for col in 0..self.cols() - 1 {
                let val = self.index(row, col) * vector.index(row);
                self.set_index(row, col, val);
            }
        }
    }

    /// divides each component from the vector with the component of the other matrix and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let mut matrix = Matrix::new(vec![vec![2., -3., 1.], vec![2., 0., -1.]]);
    /// let vector = Vector::new(vec![2., 4., 6.]);
    /// matrix.div_vec(&vector);
    /// assert_eq!(
    ///     matrix,
    ///     Matrix::new(vec![vec![1.0, -3.0, 1.0], vec![0.5, 0.0, -1.0]])
    /// );
    /// ```
    /// note it panics if the matrices have not the same rows and cols
    pub fn div_vec(&mut self, vector: &Vector) {
        check_vector(self, vector);
        for row in 0..self.rows() - 1 {
            for col in 0..self.cols() - 1 {
                let val = self.index(row, col) / vector.index(row);
                self.set_index(row, col, val);
            }
        }
    }

    /// adds each component from the matrix with the component of the other matrix and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let mut matrix1 = Matrix::new(vec![vec![2., -3., 1.], vec![2., 0., -1.]]);
    /// let matrix2 = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
    ///
    /// matrix1.add_mat(&matrix2);
    /// assert_eq!(
    ///     matrix1,
    ///     Matrix::new(vec![vec![4.0, 0.0, 6.0], vec![9.0, 1.0, 3.0]])
    /// );
    /// ```
    /// note it panics if the matrices have not the same rows and cols
    pub fn add_mat(&mut self, other: &Matrix) {
        check_matrix(self, other);
        self.matrix_flatt = self.matrix_flatt() + other.matrix_flatt();
        self.is_transpose = false;
        self.cols = other.cols();
        self.rows = other.rows();
    }

    /// subtracts each component from the matrix with the component of the other matrix and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let mut matrix1 = Matrix::new(vec![vec![2., -3., 1.], vec![2., 0., -1.]]);
    /// let matrix2 = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
    ///
    /// matrix1.sub_mat(&matrix2);
    /// assert_eq!(
    ///   matrix1,
    ///   Matrix::new(vec![vec![0.0, -6.0, -4.0], vec![-5.0, -1.0, -5.0]])
    /// );
    /// ```
    /// note it panics if the matrices have not the same rows and cols
    pub fn sub_mat(&mut self, other: &Matrix) {
        check_matrix(self, other);
        self.matrix_flatt = self.matrix_flatt() - other.matrix_flatt();
        self.is_transpose = false;
        self.cols = other.cols();
        self.rows = other.rows();
    }

    /// divides each component from the matrix with the component of the other matrix and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let mut matrix1 = Matrix::new(vec![vec![2., -3., 1.], vec![2., 0., -1.]]);
    /// let matrix2 = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
    ///
    /// matrix1.div_mat(&matrix2);
    /// assert_eq!(
    ///     matrix1,
    ///     Matrix::new(vec![vec![1.0, -1.0, 0.2], vec![0.2857143, 0.0, -0.25]])
    /// );
    /// ```
    /// note it panics if the matrices have not the same rows and cols
    pub fn div_mat(&mut self, other: &Matrix) {
        check_matrix(self, other);
        self.matrix_flatt = self.matrix_flatt() / other.matrix_flatt();
        self.is_transpose = false;
        self.cols = other.cols();
        self.rows = other.rows();
    }

    /// multiples each component from the matrix with the component of the other matrix and stors the result in this matrix   
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let mut matrix1 = Matrix::new(vec![vec![2., -3., 1.], vec![2., 0., -1.]]);
    /// let matrix2 = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
    ///
    /// matrix1.mul_mat(&matrix2);
    /// assert_eq!(
    ///   matrix1,
    ///   Matrix::new(vec![vec![4.0, -9.0, 5.0], vec![14.0, 0.0, -4.0]])
    /// );
    /// ```
    /// note it panics if the matrices have not the same rows and cols
    pub fn mul_mat(&mut self, other: &Matrix) {
        check_matrix(self, other);
        self.matrix_flatt = self.matrix_flatt() * other.matrix_flatt();
        self.is_transpose = false;
        self.cols = other.cols();
        self.rows = other.rows();
    }

    /// returns the [determinant] of this matrix
    ///
    /// [determinant]: https://en.wikipedia.org/wiki/Determinant
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let matrix = Matrix::new(vec![vec![1., 2.], vec![3., 4.]]);
    /// assert_eq!(matrix.det(), -2.);
    /// ```
    ///  note the matrix has to be a [square matrix]
    ///
    /// [square matrix]: https://en.wikipedia.org/wiki/Square_matrix
    pub fn det(&self) -> f32 {
        check_square(self);
        if self.rows() == 2 {
            self.index(0, 0) * self.index(1, 1) - self.index(1, 0) * self.index(0, 1)
        } else {
            let mut sign = 1.;
            let mut sum = 0.;

            for col in 0..self.cols() {
                let sub = self.finde_sub(0, col);
                sum += sub.det() * sign * self.index(0, col);
                sign *= -1.;
            }

            sum
        }
    }

    /// this returns the [eigenvalues] of this matrix
    ///
    /// [eigenvalues]: https://en.wikipedia.org/wiki/Eigenvalues_and_eigenvectors
    ///
    /// ## Example
    ///
    /// ```rust
    ///
    /// ```
    /// note the matrix has to be a [square matrix]
    ///
    /// [square matrix]: https://en.wikipedia.org/wiki/Square_matrix
    pub fn eigen_val(&self) -> f32 {
        check_square(self);
        todo!();
    }

    pub fn eigen_vec(&self) -> Vector {
        check_square(self);
        todo!();
    }

    pub fn dot_mat(&self, other: &Matrix) {
        check_matrix(self, other);
        todo!();
    }

    pub fn inv(&mut self) {
        check_square(self);
        let det = self.det();
        if det == 0. {
            panic!("the determinant of the matrix can't be 0")
        }
        todo!();
    }

    /// applyes the lamda function to each value in the matrix
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let mut matrix = Matrix::new(vec![vec![0.7, 0.2, 0.3], vec![0.5, 0.6, 0.1]]);
    /// let step: Box<(dyn Fn(f32) -> f32 + 'static)> = Box::new(|x: f32| -> f32 {
    ///     if x > 0.5 {
    ///         1.
    ///     } else {
    ///         0.
    ///     }
    /// });
    /// matrix.apply_func_val(&step);
    /// assert_eq!(matrix.matrix_flatt().vec(), vec![1., 0., 0., 0., 1., 0.]);
    /// ```
    pub fn apply_func_val(&mut self, lamda: &Box<(dyn Fn(f32) -> f32 + 'static)>) {
        self.matrix_flatt.apply_func(lamda);
    }

    /// returns a vector of the sumed rows
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// use math::linear_algebra::Vector;
    /// let matrix = Matrix::new(vec![vec![3., 1.], vec![5., 3.]]);
    /// assert_eq!(matrix.sum_vec(), Vector::new(vec![8., 4.]));
    /// ```
    pub fn sum_vec(&self) -> Vector {
        let mut vec = Vec::new();
        for i in 0..self.rows() {
            vec.push(self.row(i).sum());
        }
        Vector::new(vec)
    }

    /// returns the sum of the elements
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let matrix = Matrix::new(vec![vec![3., 1.], vec![5., 3.]]);
    /// assert_eq!(matrix.sum(), 12.);
    /// ```
    pub fn sum(&self) -> f32 {
        self.matrix_flatt.sum()
    }

    // finds the sub matrix is user for the determinant
    fn finde_sub(&self, row: usize, col: usize) -> Self {
        let mut flatt = Vec::with_capacity((self.cols() - 1) * (self.rows() - 1));

        for i in 0..self.cols() {
            for j in 0..self.rows() {
                if !(i == col || j == row) {
                    flatt.push(self.index(i, j));
                }
            }
        }
        Self::new_flatt(flatt, self.cols() - 1, self.rows() - 1)
    }

    fn get_row(&self, row: usize) -> Vector {
        if self.rows < row + 1 {
            panic!("index out of bounds max row {}", self.rows - 1)
        }

        let mut result: Vec<f32> = Vec::with_capacity(self.cols);
        for i in 0..self.cols {
            result.push(self.matrix_flatt.index(i * self.rows + row));
        }

        Vector::new(result)
    }

    fn get_col(&self, col: usize) -> Vector {
        if self.cols < col + 1 {
            panic!("index out of bounds max col {}", self.cols - 1)
        }

        let mut result: Vec<f32> = Vec::with_capacity(self.rows);
        for i in (col * self.rows)..((1 + col) * self.rows) {
            result.push(self.matrix_flatt.index(i));
        }

        Vector::new(result)
    }
}

fn check_square(mat: &Matrix) {
    if !mat.is_square() {
        panic!("the matrix has to be a square matrix");
    }

    if mat.rows() == 1 {
        panic!("the matrix has to have more then one row");
    }
}

fn check_vector(mat: &Matrix, vec: &Vector) {
    if vec.len() != mat.rows() {
        panic!(
            "wrong vector shape expected {}, got {}",
            mat.rows,
            vec.len()
        )
    }
}

fn check_matrix(mat1: &Matrix, mat2: &Matrix) {
    if mat1.rows() != mat2.rows() {
        panic!("wrong row shape expected {}, got {}", mat1.rows, mat2.rows)
    }

    if mat1.cols() != mat2.cols() {
        panic!("wrong col shape expected {}, got {}", mat1.cols, mat2.cols)
    }
}

#[cfg(feature = "gpu")]
use crate::random;
#[cfg(feature = "gpu")]
use std::mem;

#[cfg(feature = "gpu")]
impl Matrix {
    /// this return a vector of bytes representing the matrix
    ///
    /// this is useful for the *GPU* because the interface only uses bytes
    ///
    /// ## Example
    ///
    /// ```rust
    /// use math::linear_algebra::Matrix;
    /// let matrix = Matrix::new(vec![vec![2., 3.], vec![7., 4.]]);
    /// assert_eq!(
    ///     matrix.bytes(),
    ///     vec![0, 0, 0, 64, 0, 0, 0, 64, 0, 0, 0, 64, 0, 0, 64, 64, 0, 0, 224, 64, 0, 0, 128, 64]
    /// );
    /// ```
    /// note the fist and seconde `f32` is the rows and cols of the matrix
    pub fn bytes(&self) -> Vec<u8> {
        let size = (2 + self.rows() * self.cols()) * mem::size_of::<f32>();
        let mut bytes = Vec::<u8>::with_capacity(size);

        for b in (self.rows() as f32).to_ne_bytes().to_vec() {
            bytes.push(b);
        }
        for b in (self.cols() as f32).to_ne_bytes().to_vec() {
            bytes.push(b);
        }

        // `skip(4)` because the first 4 bytes is the len of the vector (f32 = 4bytes)
        for &b in self.matrix_flatt().bytes().iter().skip(4) {
            bytes.push(b);
        }
        bytes
    }
}
