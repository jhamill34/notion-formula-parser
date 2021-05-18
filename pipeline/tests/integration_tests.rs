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

fn step_two(input: Vec<u8>) -> HandlerResult<String> {
    match String::from_utf8(input) {
        Ok(value) => Ok(value),
        Err(e) => panic!(e),
    }
}

fn step_three(_input: Vec<u8>) -> HandlerResult<Vec<u8>> {
    Err(SimpleError::new("Something went wrong".into()))
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
        let step = FnHandler::new(step_two);
        let result = step.handle(vec![65, 66, 67]).unwrap();

        assert_eq!("ABC", result);
    }

    #[test]
    fn step_three_fails() {
        let step = FnHandler::new(step_three);
        let result = format!("{}", step.handle(vec![65, 66, 67]).unwrap_err());

        assert_eq!("Something went wrong", result)
    }

    #[test]
    fn test_simple_workflow_works() {
        let pipe: Pipeline<u8, String> = Pipeline::new().add(StepOne).add(FnHandler::new(step_two));

        let result = pipe.start(4).unwrap();

        assert_eq!("ABCD", result);
    }

    #[test]
    fn test_workflow_errors_in_the_middle() {
        let pipe: Pipeline<u8, String> = Pipeline::new()
            .add(StepOne)
            .add(FnHandler::new(step_three))
            .add(FnHandler::new(step_two));

        let result = format!("{}", pipe.start(4).unwrap_err());

        assert_eq!("Something went wrong", result);
    }

    #[test]
    fn test_empty_pipe_returns_input() {
        let pipe = Pipeline::new();
        let result = pipe.start(5).unwrap();

        assert_eq!(5, result);
    }
}
