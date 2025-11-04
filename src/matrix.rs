use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix<const ROW_COUNT: usize, const COL_COUNT: usize> {
    pub data: [[f32; COL_COUNT]; ROW_COUNT],
}

impl Eq for Matrix<4, 4> {}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> PartialEq for Matrix<ROW_COUNT, COL_COUNT> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> Mul for &Matrix<ROW_COUNT, COL_COUNT> {
    type Output = Matrix<ROW_COUNT, COL_COUNT>;

    fn mul(self, rhs: &Matrix<ROW_COUNT, COL_COUNT>) -> Self::Output {
        let mut data: [[f32; COL_COUNT]; ROW_COUNT] = self.data;
        for row_idx in 0..ROW_COUNT {
            for col_idx in 0..COL_COUNT {
                data[row_idx][col_idx] = (0..ROW_COUNT).fold(0f32, |acc, idx| {
                    acc + self.data[row_idx][idx] * rhs.data[idx][col_idx]
                });
            }
        }

        Matrix { data }
    }
}
