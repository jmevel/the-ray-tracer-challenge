#[derive(Debug)]
pub struct Matrix<const ROW_COUNT: usize, const COL_COUNT: usize> {
    pub data: [[f32; ROW_COUNT]; COL_COUNT],
}
