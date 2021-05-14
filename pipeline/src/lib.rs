use std::fmt;
use std::fmt::Formatter;

pub trait Error {
    fn description(&self) -> String;
}

pub struct HandlerError<'a>(Box<dyn Error + 'a>);

impl<'a> HandlerError<'a> {
    pub fn new(err: impl Error + 'a) -> Self {
        HandlerError(Box::new(err))
    }
}
impl Error for HandlerError<'_> {
    fn description(&self) -> String {
        self.0.description()
    }
}

impl fmt::Debug for HandlerError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.description())
    }
}

pub type HandlerResult<'a, T> = Result<T, HandlerError<'a>>;

pub trait Handler {
    type Input;
    type Output;

    fn handle(&self, input: Self::Input) -> HandlerResult<Self::Output>;
}

struct Stage<'a, I, K, O> {
    current: Box<dyn Handler<Input=I, Output=K> + 'a>,
    next: Box<dyn Handler<Input=K, Output=O> + 'a>
}

impl<I, K, O> Handler for Stage<'_, I, K, O> {
    type Input=I;
    type Output=O;
    fn handle(&self, input: I) -> HandlerResult<O> {
        return match self.current.handle(input) {
            Ok(current_result) => self.next.handle(current_result),
            Err(e) => Err(e)
        };
    }
}

impl<'a, I, K, O> Stage<'a, I, K, O> {
    fn new(
        current: Box<dyn Handler<Input=I, Output=K> + 'a>,
        next: Box<dyn Handler<Input=K, Output=O> + 'a>
    ) -> Box<Stage<'a, I, K, O>> {
        Box::new(Stage { current, next })
    }
}

pub struct Pipeline<'a, I, O> {
    head: Box<dyn Handler<Input=I, Output=O> + 'a>
}
impl<'a, I: 'a, O: 'a> Pipeline<'a, I, O> {
    pub fn new(handler: impl Handler<Input=I, Output=O> + 'a) -> Pipeline<'a, I, O> {
        Pipeline { head: Box::new(handler) }
    }

    pub fn add<K: 'a>(self, handler: impl Handler<Input=O, Output=K> + 'a) -> Pipeline<'a, I, K> {
        Pipeline {
            head: Stage::new(
                self.head,
                Box::new(handler)
            )
        }
    }

    pub fn start(&self, input: I) -> HandlerResult<O> {
        self.head.handle(input)
    }
}
