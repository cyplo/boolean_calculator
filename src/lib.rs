mod boolean_calculator {
    #[derive(Debug)]
    pub struct Calculator {
    }

    impl Calculator {
        pub fn new(text: &str) -> Calculator {
            Calculator{}
        }

        pub fn get_result(&self) -> bool {
            true 
        }
    }

    #[cfg(test)]
    mod calculator_should {
        use super::*;
        #[test]
        fn return_true_for_t() {
            let text = "T";
            let calculator = Calculator::new(text);
            let result = calculator.get_result();
            assert_eq!(result, true);
        }
    }

}