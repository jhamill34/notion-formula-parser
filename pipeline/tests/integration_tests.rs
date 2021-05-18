use pipeline::*;

const ALPHA_START: u8 = 65;

struct StepOne;
impl Handler<u8, Vec<u8>> for StepOne {
    fn handle(&self, input: u8) -> HandlerResult<Vec<u8>> {
        let mut output: Vec<u8> = Vec::new();
        let mut i: u8 = 0;

        while i < input {
            output.push(i + ALPHA_START);

            i = i + 1;
        }

        Ok(output)
    }
}

struct StepTwo;
impl Handler<Vec<u8>, String> for StepTwo {
    fn handle(&self, input: Vec<u8>) -> HandlerResult<String> {
        match String::from_utf8(input) {
            Ok(value) => Ok(value),
            Err(e) => panic!(e),
        }
    }
}

enum TestErrors {
    SimpleError,
}
impl Error for TestErrors {
    fn description(&self) -> String {
        match &self {
            Self::SimpleError => String::from("I'm a simple error..."),
        }
    }
}

struct StepThree;
impl Handler<Vec<u8>, Vec<u8>> for StepThree {
    fn handle(&self, _input: Vec<u8>) -> HandlerResult<Vec<u8>> {
        Err(HandlerError::new(TestErrors::SimpleError))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn step_one_works() {
        let step = StepOne;
        let result = step.handle(3).unwrap();

        assert_eq!(vec![65, 66, 67], result)
    }

    #[test]
    fn step_two_works() {
        let step = StepTwo;
        let result = step.handle(vec![65, 66, 67]).unwrap();

        assert_eq!("ABC", result);
    }

    #[test]
    fn step_three_fails() {
        let step = StepThree;
        let result = step.handle(vec![65, 66, 67]).unwrap_err().description();

        assert_eq!("I'm a simple error...", result)
    }

    #[test]
    fn test_simple_workflow_works() {
        let pipe: Pipeline<u8, String> = Pipeline::new(StepOne).add(StepTwo);

        let result = pipe.start(4).unwrap();

        assert_eq!(String::from("ABCD"), result);
    }

    #[test]
    fn test_workflow_errors_in_the_middle() {
        let pipe: Pipeline<u8, String> = Pipeline::new(StepOne).add(StepThree).add(StepTwo);

        let result = pipe.start(4).unwrap_err().description();

        assert_eq!("I'm a simple error...", result);
    }
}
