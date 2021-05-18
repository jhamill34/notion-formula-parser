use std::error::Error;
use std::fmt;

pub type HandlerResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct SimpleError {
    details: String
}
impl SimpleError {
    pub fn new(details: String) -> Box<Self> {
        Box::new(SimpleError { details })
    }
}
impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl Error for SimpleError {}

pub trait Handler<I, O> {
    fn handle(&self, input: I) -> HandlerResult<O>;
}

pub struct FnHandler<I, O> {
    func: fn (I) -> HandlerResult<O>
}
impl<I, O> FnHandler<I, O> {
    pub fn new(func: fn(I) -> HandlerResult<O>) -> Self {
        FnHandler { func }
    }
}
impl<I, O> Handler<I, O> for FnHandler<I, O> {
    fn handle(&self, input: I) -> HandlerResult<O> {
        (self.func)(input)
    }
}

struct Stage<'a, I, K, O> {
    current: Box<dyn Handler<I, K> + 'a>,
    next: Box<dyn Handler<K, O> + 'a>,
}
impl<I, K, O> Handler<I, O> for Stage<'_, I, K, O> {
    fn handle(&self, input: I) -> HandlerResult<O> {
        return match self.current.handle(input) {
            Ok(current_result) => self.next.handle(current_result),
            Err(e) => Err(e),
        };
    }
}
impl<'a, I, K, O> Stage<'a, I, K, O> {
    fn new(
        current: Box<dyn Handler<I, K> + 'a>,
        next: Box<dyn Handler<K, O> + 'a>,
    ) -> Box<Stage<'a, I, K, O>> {
        Box::new(Stage { current, next })
    }
}

pub struct Pipeline<'a, I, O> {
    head: Box<dyn Handler<I, O> + 'a>,
}
impl<'a, I: 'a, O: 'a> Pipeline<'a, I, O> {
    pub fn new(handler: impl Handler<I, O> + 'a) -> Pipeline<'a, I, O> {
        Pipeline {
            head: Box::new(handler),
        }
    }

    pub fn add<K: 'a>(self, handler: impl Handler<O, K> + 'a) -> Pipeline<'a, I, K> {
        Pipeline {
            head: Stage::new(self.head, Box::new(handler)),
        }
    }

    pub fn start(&self, input: I) -> HandlerResult<O> {
        self.head.handle(input)
    }
}
