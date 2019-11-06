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

    #[test]
    fn test_project_onto() {
        // Arrange
        let v_to_be_projected = Vector3::new_from_values(5.0, 5.0, 0.0);
        let v_to_project_onto = Vector3::new_from_values(1.0, 0.0, 0.0);

        // Act
        let projected = v_to_be_projected.project_onto(&v_to_project_onto);

        // Assert
        assert_eq!(projected.calculate_length(), 5.0);
    }
}

#[cfg(test)]
mod ray_tests {

    use super::super::geometry::{Vector3, Ray};

    #[test]
    fn test_ray_distance_to() {
        //Arrange
        let v_origin = Vector3::new();
        let v_direction = Vector3::new_from_values(1.0, 0.0, 0.0);
        let ray = Ray::new_from_vectors(&v_origin, &v_direction);
        let v_test = Vector3::new_from_values(5.0, 5.0, 0.0);

        // Act
        let dist = ray.distance_to(&v_test);

        // Assert
        assert!((dist - 5.0).abs() < 0.1);
    }
}