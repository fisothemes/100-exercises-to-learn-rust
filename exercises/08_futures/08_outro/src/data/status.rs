use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    #[default]
    ToDo,
    InProgress,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Self::ToDo => "To-do",
            Self::InProgress => "In progress",
            Self::Done => "Done"
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_status_string_representation(){
        assert_eq!(Status::ToDo.to_string(), "To-do".to_string());
        assert_eq!(Status::InProgress.to_string(), "In progress".to_string());
        assert_eq!(Status::Done.to_string(), "Done".to_string());
    }
}