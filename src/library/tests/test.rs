#[cfg(test)]
mod tests {
    use coil_calculator::coil::Coil;
    use coil_calculator::utils::*;
    use std::f64::EPSILON;
    
    #[test]
    fn test_cross_product() {
        let mut result = [0.0; 3];

        // Test case 1: Standard vectors
        cross_product(&[1.0, 2.0, 3.0], &[2.0, 3.0, 4.0], &mut result);
        assert_eq!(result, [-1.0, 2.0, -1.0]);

        // Test case 2: Unit vectors i × j = k
        cross_product(&[1.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &mut result);
        assert_eq!(result, [0.0, 0.0, 1.0]);

        // Test case 3: Zero vector result
        cross_product(&[1.0, 2.0, 3.0], &[2.0, 4.0, 6.0], &mut result);
        assert_eq!(result, [0.0, 0.0, 0.0]);
    }

    // #[test]
    // fn test_vec_add() {
    //     let mut result = [0.0; 3];
    // 
    //     // Test case 1: Positive numbers
    //     vec_add(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], &mut result);
    //     assert_eq!(result, [5.0, 7.0, 9.0]);
    // 
    //     // Test case 2: Mixed positive and negative
    //     vec_add(&[1.0, -2.0, 3.0], &[-4.0, 5.0, -6.0], &mut result);
    //     assert_eq!(result, [-3.0, 3.0, -3.0]);
    // 
    //     // Test case 3: Zero vector
    //     vec_add(&[0.0, 0.0, 0.0], &[1.0, 2.0, 3.0], &mut result);
    //     assert_eq!(result, [1.0, 2.0, 3.0]);
    // }

    #[test]
    fn test_vec_diff() {
        let mut result = [0.0; 3];

        // Test case 1: Positive numbers
        vec_diff(&[4.0, 5.0, 6.0], &[1.0, 2.0, 3.0], &mut result);
        assert_eq!(result, [3.0, 3.0, 3.0]);

        // Test case 2: Mixed positive and negative
        vec_diff(&[1.0, -2.0, 3.0], &[-4.0, 5.0, -6.0], &mut result);
        assert_eq!(result, [5.0, -7.0, 9.0]);

        // Test case 3: Zero result
        vec_diff(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0], &mut result);
        assert_eq!(result, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_vec_div() {
        let mut result = [0.0; 3];

        // Test case 1: Division by positive number
        vec_div(&[2.0, 4.0, 6.0], 2.0, &mut result);
        assert_eq!(result, [1.0, 2.0, 3.0]);

        // Test case 2: Division by negative number
        vec_div(&[2.0, 4.0, 6.0], -2.0, &mut result);
        assert_eq!(result, [-1.0, -2.0, -3.0]);

        // Test case 3: Division of zero vector
        vec_div(&[0.0, 0.0, 0.0], 5.0, &mut result);
        assert_eq!(result, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_abs3() {
        let mut result = 0.0;

        // Test case 1: Unit vector
        abs3(&[1.0, 0.0, 0.0], &mut result);
        assert!((result - 1.0).abs() < EPSILON);

        // Test case 2: General vector
        abs3(&[1.0, 2.0, 2.0], &mut result);
        assert!((result - 27.0).abs() < EPSILON); // √(1² + 2² + 2²)³ = 3³ = 27

        // Test case 3: Zero vector
        abs3(&[0.0, 0.0, 0.0], &mut result);
        assert!((result - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_change_pos() {
        // Test case 1: Positive z-coordinate
        let mut pos = [1.0, 1.0, 1.0];
        change_pos(&mut pos, 0.1, 1.0, 2.0, 10, 5);
        // We don't test exact values here since the function is complex,
        // but we can verify the general behavior
        assert!(pos[1] > 1.0); // y should increase

        // Test case 2: Negative z-coordinate
        let mut pos = [1.0, 1.0, -1.0];
        let original_pos = pos;
        change_pos(&mut pos, 0.1, 1.0, 2.0, 10, 5);
        assert!(pos != original_pos); // Position should change
        assert!(pos[1] > original_pos[1]); // y should increase
    }
    
    #[test]
    fn test_infinite() {
        let coil: Coil = Coil::Infinite((0.02, 3.5, 2000).into());
        assert_eq!(coil.calculate(200), 11.697892841760549);
    }
}