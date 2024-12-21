#[derive(Debug, PartialEq, Eq)]
pub struct Answer {
    result: Result<String, String>,
}
impl Answer {
    pub fn get_result(&self) -> Result<String, String> {
        self.result.clone()
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self { result: Ok(value) }
    }
}

impl From<&str> for Answer {
    fn from(value: &str) -> Self {
        Self {
            result: Ok(String::from(value)),
        }
    }
}

impl From<()> for Answer {
    fn from(_: ()) -> Self {
        Self {
            result: Err("No answer".to_string()),
        }
    }
}

impl<T> From<Option<T>> for Answer
where
    Answer: From<T>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => Answer::from(v),
            None => Self {
                result: Err("No answer".to_string()),
            },
        }
    }
}

impl<T, E> From<Result<T, E>> for Answer
where
    Answer: From<T>,
    E: std::fmt::Display,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => Answer::from(v),
            Err(e) => Self {
                result: Err(e.to_string()),
            },
        }
    }
}

macro_rules! from_numeric_to_answer {
    ($type:ty) => {
        impl From<$type> for Answer {
            fn from(value: $type) -> Self {
                Self {
                    result: Ok(value.to_string()),
                }
            }
        }
        impl From<&$type> for Answer {
            fn from(value: &$type) -> Self {
                Self {
                    result: Ok(value.to_string()),
                }
            }
        }
    };
}
from_numeric_to_answer!(usize);
from_numeric_to_answer!(u64);
from_numeric_to_answer!(u32);
from_numeric_to_answer!(u16);
from_numeric_to_answer!(u8);
from_numeric_to_answer!(isize);
from_numeric_to_answer!(i64);
from_numeric_to_answer!(i32);
from_numeric_to_answer!(i16);
from_numeric_to_answer!(i8);
from_numeric_to_answer!(f32);
from_numeric_to_answer!(f64);
