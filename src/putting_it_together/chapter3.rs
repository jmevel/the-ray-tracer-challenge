use the_ray_tracer_challenge::{Matrix, Tuple, Vector};

#[allow(dead_code)]
pub fn putting_it_together() {
    let identity_matrix: Matrix<4, 4> = Matrix::identity_matrix();
    let identity_matrix_inverted = identity_matrix.invert().unwrap();
    assert_eq!(identity_matrix, identity_matrix_inverted);
    println!("The inverse of the identity matrix is equal to the identity matrix");

    let matrix = Matrix {
        data: [
            [0f32, 9f32, 3f32, 0f32],
            [9f32, 8f32, 0f32, 8f32],
            [1f32, 8f32, 5f32, 3f32],
            [0f32, 0f32, 3f32, 8f32],
        ],
    };

    let inverse = matrix.invert().unwrap();
    let matrix_multiplied_by_its_inverse = &matrix * &inverse;
    assert_eq!(matrix_multiplied_by_its_inverse, identity_matrix);
    println!("A matrix multiplied by its inverse is equal to the identity matrix");

    let transpose_of_inverse = inverse.transpose();
    let inverse_of_transpose = matrix.transpose().invert().unwrap();
    assert_eq!(transpose_of_inverse, inverse_of_transpose);
    println!("The transpose of the inverse of a matrix is equal to its inverse of its transpose");

    let tuple = Vector::new(1f32, 2f32, 3f32, 4f32);
    let tuple_multiplied_by_identity_matrix = &tuple * &identity_matrix;
    assert_eq!(tuple_multiplied_by_identity_matrix, tuple);
    println!("A tuple multiplied by the identity matrix is equal to the initial tuple");
    let mut data = identity_matrix.data;
    data[2][3] = 2f32;
    let identity_matrix_modified = Matrix { data };
    let _tuple_multiplied_by_modified_identity_matrix = &tuple * &identity_matrix_modified;
    println!(
        "A tuple multiplied by an identity matrix that had a single element modified gets only one of its element modified as well"
    );
}
