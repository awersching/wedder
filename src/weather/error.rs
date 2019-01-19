use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UndefinedCondition;

const ERROR_MSG: &str = "Undefined weather condition";

impl fmt::Display for UndefinedCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ERROR_MSG)
    }
}

impl error::Error for UndefinedCondition {
    fn description(&self) -> &str {
        ERROR_MSG
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
