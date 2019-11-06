#[cfg(test)]
mod sample_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(test)]
mod vector_tests {

    use super::super::geometry::{Vector3};

    #[test]
    fn test_Vector_length() {
        // Arrange
        let v1 = Vector3::new_from_values(1.0, 0.0, 0.0);
        let v2 = Vector3::new();
        let v3 = Vector3::new_from_values(1.0, 1.0, 1.0);

        // Assert
        assert_eq!(v1.calculate_length(), 1.0);
        assert_eq!(v2.calculate_length(), 0.0);
        assert_eq!(v3.calculate_length(), 3.0_f64.sqrt())
    }
}