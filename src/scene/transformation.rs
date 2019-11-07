use std::ops;

pub trait Transformable {
    fn transform(&self, transform: &Transformation) -> Self;
}

#[derive(Debug, PartialEq)]
pub struct MatrixColumn {
    pub column: [f64; 4]
}

impl MatrixColumn {
    fn scale(scale_factor: f64) -> MatrixColumn {
        MatrixColumn {
            column: [scale_factor, scale_factor, scale_factor, 1.0]
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Transformation {
    pub matrix: [[f64; 4]; 4]
}

impl Transformation {

    pub fn new_from_rows(row0: [f64; 4], row1: [f64; 4], row2: [f64; 4], row3: [f64; 4]) -> Transformation {
        Transformation {
            matrix: [
                row0,
                row1,
                row2,
                row3
            ]
        }
    }

    pub fn get_row(&self, row_index: usize) -> [f64; 4] {
        self.matrix[row_index]
    }

    pub fn scale(scale_factor: f64) -> Transformation {
        println!("Scale: {:?}", &Transformation::identity() * scale_factor);
        &Transformation::identity() * &MatrixColumn::scale(scale_factor)
    }

    pub const fn identity() -> Transformation {
        Transformation {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]]
        }
    }
}

impl ops::Mul<&MatrixColumn> for &Transformation {
    type Output = Transformation;

    fn mul(self, rhs: &MatrixColumn) -> Transformation {
        let mut row0 = self.get_row(0);
        let mut row1 = self.get_row(1);
        let mut row2 = self.get_row(2);
        let mut row3 = self.get_row(3);

        for i in 0..4 {
            row0[i] *= rhs.column[i];
            row1[i] *= rhs.column[i];
            row2[i] *= rhs.column[i];
            row3[i] *= rhs.column[i];
        }

        Transformation::new_from_rows(row0, row1, row2, row3)
    }
}

impl ops::Mul<f64> for &Transformation {
    type Output = Transformation;

    fn mul(self, rhs: f64) -> Transformation {
        let mut row0 = self.get_row(0);
        let mut row1 = self.get_row(1);
        let mut row2 = self.get_row(2);
        let mut row3 = self.get_row(3);

        for i in 0..4 {
            row0[i] *= rhs;
            row1[i] *= rhs;
            row2[i] *= rhs;
            row3[i] *= rhs;
        }

        Transformation::new_from_rows(row0, row1, row2, row3)
    }
}

impl ops::Mul<&Transformation> for &Transformation {
    type Output = Transformation;

    fn mul(self, rhs: &Transformation) -> Transformation {
        let mut mult = Transformation::identity();
        for i in 0..4 {
            for j in 0..4 {
                mult.matrix[i][j] = self.matrix[i][0] * rhs.matrix[0][j] +
                                    self.matrix[i][1] * rhs.matrix[1][j] +
                                    self.matrix[i][2] * rhs.matrix[2][j] +
                                    self.matrix[i][3] * rhs.matrix[3][j];
            }
        }
        mult
    }
}