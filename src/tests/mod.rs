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
        // Arrange
        let v_origin = Vector3::new();
        let v_direction = Vector3::new_from_values(1.0, 0.0, 0.0);
        let ray = Ray::new_from_vectors(&v_origin, &v_direction);
        let v_test = Vector3::new_from_values(5.0, 5.0, 0.0);

        // Act
        let dist_1 = ray.distance_to(&v_test);
        let expected_1 = 5.0;
        let dist_2 = ray.distance_to(&(v_test * -1.0));
        let expected_2 = (5.0 * 5.0 + 5.0 * 5.0_f64).sqrt();

        // Assert
        assert!((dist_1 - expected_1).abs() < 0.1);
        assert_eq!(dist_2, expected_2);
    }

    #[test]
    fn test_ray_project_point() {
        // Arrange
        let v_origin = Vector3::new();
        let v_direction = Vector3::new_from_values(1.0, 0.0, 0.0);
        let ray = Ray::new_from_vectors(&v_origin, &v_direction);
        let v_test = Vector3::new_from_values(5.0, 5.0, 0.0);

        // Act
        let v_projected_1 = ray.project_vec_onto_self(&v_test);
        let expected_1 = Vector3::new_from_values(5.0, 0.0, 0.0);
        let v_projected_2 = ray.project_vec_onto_self(&-&v_test);
        let expected_2 = Vector3::new();

        // Assert
        assert_eq!(v_projected_1, expected_1);
        assert_eq!(v_projected_2, expected_2);
    }

}

#[cfg(test)]
mod sphere_tests {

    use super::super::geometry::{Vector3, Ray, Sphere};
    use super::super::traits::{RayIntersectionResult, RayCast};
    
    #[test]
    fn test_sphere_ray_intersection() {
        // Arrange
        let sphere_far = Sphere::new_from_values(&Vector3::new_from_values(10.0, 10.0, 10.0), 1.0);
        let sphere_intersecting = Sphere::new_from_values(&Vector3::new_from_values(5.0, 5.0, 1.0), 5.0);
        let ray = Ray::new_from_vectors(&Vector3::new_from_values(1.0, 1.0, 1.0), &Vector3::new_from_values(1.0, 0.0, 0.0));

        // Act
        let far_int = sphere_far.intersect_ray(&ray);
        let intersecting_int = sphere_intersecting.intersect_ray(&ray);

        // Assert
        match far_int {
            RayIntersectionResult::Some(_) => assert!(false),
            RayIntersectionResult::None => assert!(true)
        }
        match intersecting_int {
            RayIntersectionResult::None => assert!(false),
            RayIntersectionResult::Some(v) => assert_eq!(v, Vector3::new_from_values(2.0, 1.0, 1.0))
        }

    }
}

#[cfg(test)]
mod transformation_tests {
    
    use super::super::scene::{Transformation, Transformable};
    use super::super::geometry::{Vector3};

    #[test]
    fn test_identity_multiplication() {
        // Arrange
        let trans_ident = Transformation::identity();
        let trans_scale = Transformation::scale(2.0);

        // Act
        let trans_ident_squared = &trans_ident * &trans_ident;
        let trans_ident_scaled = &trans_ident * &trans_scale;

        // Assert
        assert_eq!(trans_ident_squared, trans_ident);
        assert_eq!(trans_ident_scaled, trans_scale);
    }

    #[test]
    fn test_vector_transforms() {
        // Arrange
        let v_test = Vector3::new_from_values(1.0, 5.0, 3.25);
        let trans_ident = Transformation::identity();
        let trans_scale = Transformation::scale(2.0);

        // Act
        let v_ident = v_test.transform(&trans_ident);
        let v_scale = v_test.transform(&trans_scale);

        // Assert
        assert_eq!(v_ident, v_test);
        assert_eq!(v_scale, Vector3::new_from_values(2.0, 10.0, 6.5));
    }
}

#[cfg(test)]
mod render_tests {

    use super::super::scene::{Document, Renderer};
    use image::GenericImageView;

    #[test]
    fn test_can_render_document() {
        // Arrange
        let document = Document::default_test();
        let renderer = Renderer::new_from_values(100, 75, 90.0);

        // Act
        let img = renderer.render(&document, &document.camera_table.cameras[0]);

        // Assert
        assert_eq!(renderer.width, img.width());
        assert_eq!(renderer.height, img.height());

        // save image as test
        match img.save("test.png") {
            Result::Err(_) => panic!("Failed to save image"),
            Result::Ok(_) => assert!(true)
        }
    }
}