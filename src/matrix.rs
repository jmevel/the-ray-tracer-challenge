#[derive(Debug)]
pub struct Matrix<const ROW_COUNT: usize, const COL_COUNT: usize> {
    pub data: [[f32; ROW_COUNT]; COL_COUNT],
}

impl Eq for Matrix<4, 4> {}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> PartialEq for Matrix<ROW_COUNT, COL_COUNT> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
