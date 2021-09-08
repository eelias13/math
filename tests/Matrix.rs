#[cfg(test)]
mod tests {
    use math::linear_algebra::Matrix;
    use math::linear_algebra::Vector;

    #[test]
    fn new_rand() {
        let matrix = Matrix::new_rand(3, 4);
        assert_eq!(
            matrix.matrix_flatt(),
            vec![
                0.69186187,
                0.3494884,
                0.23957491,
                0.06540034,
                0.5443042,
                0.013656098,
                0.4336478,
                0.8349666,
                0.10932327,
                0.52898574,
                0.4612443,
                0.3579495,
            ]
        );

        let matrix = Matrix::new_rand(2, 3);
        assert_eq!(
            matrix.matrix_flatt(),
            vec![
                0.69186187,
                0.3494884,
                0.23957491,
                0.06540034,
                0.5443042,
                0.013656098,
            ]
        );
    }
    #[test]
    #[ignore]
    fn det() {
        let matrix = Matrix::new(vec![vec![2., -3., 1.], vec![2., 0., -1.], vec![1., 4., 5.]]);
        assert_eq!(matrix.det(), 49.);
    }

    #[test]
    #[should_panic(expected = "the matrix has to be a square matrix")]
    fn det_panic() {
        let matrix = Matrix::new(vec![
            vec![2., -3., 1.],
            vec![2., 0., -1.],
            vec![1., 4., 5.],
            vec![2., 0., -1.],
        ]);
        assert_eq!(matrix.det(), 49.);
    }

    #[test]
    fn bytes() {
        let matrix = Matrix::new(vec![vec![2., 3.], vec![7., 4.]]);
        assert_eq!(
            matrix.bytes(),
            vec![0, 0, 0, 64, 0, 0, 0, 64, 0, 0, 0, 64, 0, 0, 64, 64, 0, 0, 224, 64, 0, 0, 128, 64]
        );
    }

    #[test]
    #[ignore]
    fn matrix_flatt() {
        let mut matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
        assert_eq!(matrix.matrix_flatt(), vec![2., 3., 5., 7., 1., 4.]);
        matrix.transpose();
        assert_eq!(matrix.matrix_flatt(), vec![2., 7., 3., 1., 5., 4.]);
    }

    #[test]
    #[should_panic(expected = "wrong row shape expected 3, got 4")]
    fn new() {
        let _ = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4., 1.]]);
    }

    #[test]
    fn mul_scalar() {
        let mut matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
        matrix.mul_scalar(&2.);
        assert_eq!(
            matrix,
            Matrix::new(vec![
                vec![2. * 2., 3. * 2., 5. * 2.],
                vec![7. * 2., 1. * 2., 4. * 2.]
            ])
        );
    }

    #[test]
    fn div_scalar() {
        let mut matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
        matrix.div_scalar(&2.);
        assert_eq!(
            matrix,
            Matrix::new(vec![
                vec![2. / 2., 3. / 2., 5. / 2.],
                vec![7. / 2., 1. / 2., 4. / 2.]
            ])
        );
    }

    #[test]
    fn add_scalar() {
        let mut matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
        matrix.add_scalar(&2.);
        assert_eq!(
            matrix,
            Matrix::new(vec![
                vec![2. + 2., 3. + 2., 5. + 2.],
                vec![7. + 2., 1. + 2., 4. + 2.]
            ])
        );
    }

    #[test]
    fn sub_scalar() {
        let mut matrix = Matrix::new(vec![vec![2., 3., 5.], vec![7., 1., 4.]]);
        matrix.sub_scalar(&2.);
        assert_eq!(
            matrix,
            Matrix::new(vec![
                vec![2. - 2., 3. - 2., 5. - 2.],
                vec![7. - 2., 1. - 2., 4. - 2.]
            ])
        );
    }

    #[test]
    fn transpose() {
        let mut matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
        assert_eq!(matrix.is_transpose(), false);
        matrix.transpose();
        assert_eq!(matrix.is_transpose(), true);
        matrix.transpose();
        assert_eq!(matrix.is_transpose(), false);
    }

    #[test]
    fn col() {
        let mut matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
        assert_eq!(matrix.cols(), 2);
        assert_eq!(matrix.col(0), Vector::new(vec![3., 2., 4.]));
        assert_eq!(matrix.col(1), Vector::new(vec![4., 5., 6.]));

        matrix.transpose();

        assert_eq!(matrix.cols(), 3);
        assert_eq!(matrix.col(0), Vector::new(vec![3., 4.]));
        assert_eq!(matrix.col(1), Vector::new(vec![2., 5.]));
        assert_eq!(matrix.col(2), Vector::new(vec![4., 6.]));
    }

    #[test]
    fn row() {
        let mut matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
        assert_eq!(matrix.row(0), Vector::new(vec![3., 4.]));
        assert_eq!(matrix.row(1), Vector::new(vec![2., 5.]));
        assert_eq!(matrix.row(2), Vector::new(vec![4., 6.]));

        matrix.transpose();

        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.row(0), Vector::new(vec![3., 2., 4.]));
        assert_eq!(matrix.row(1), Vector::new(vec![4., 5., 6.]));
    }

    #[test]
    #[should_panic(expected = "index out of bounds max row 2")]
    fn row_panic() {
        let matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
        let _ = matrix.row(3);
    }

    #[test]
    #[should_panic(expected = "index out of bounds max col 1")]
    fn col_panic() {
        let matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
        let _ = matrix.col(2);
    }

    #[test]
    #[ignore]
    fn index_mat() {
        let matrix = Matrix::new(vec![vec![3., 2., 4.], vec![4., 5., 6.]]);
        assert_eq!(matrix.index(0, 0), 3.);
        assert_eq!(matrix.index(0, 1), 2.);
        assert_eq!(matrix.index(0, 2), 4.);
        assert_eq!(matrix.index(1, 0), 4.);
        assert_eq!(matrix.index(1, 1), 5.);
        assert_eq!(matrix.index(1, 2), 6.);
    }

    #[test]
    fn dot_mat() {
        let matrix = Matrix::new(vec![vec![1., -1., 2.], vec![0., -3., 1.]]);
        assert_eq!(
            matrix.dot_vec(&Vector::new(vec![2., 1., 0.])),
            Vector::new(vec![1., -3.])
        )
    }

    #[test]
    #[should_panic(expected = "wrong vector shape expected 3, got 2")]
    fn dot_vec_mat_panic() {
        let matrix = Matrix::new(vec![vec![1., -1., 2.], vec![0., -3., 1.]]);
        assert_eq!(
            matrix.dot_vec(&Vector::new(vec![2., 1.])),
            Vector::new(vec![1., -3.])
        )
    }
}