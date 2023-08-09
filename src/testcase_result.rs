use colored::*;

/// `TestCaseResult` is an enumeration that contains \
/// all possible supported status of test cases.\
/// Note that `WrongAnswer` and `RuntimeError` contains\
/// simple **error hints**.
pub enum TestCaseResult {
    Accepted,
    WrongAnswer (String),
    TimeLimitError,
    RuntimeError (String),
}

impl TestCaseResult {
    /// Function `result_convert` accepts a reference to `TestCaseResult` enum \
    /// as input, and converts it to corresponding output of `ColoredString` format \
    /// for further operation.
    /// 
    /// The format is: *bold* for all status by default, \
    /// where `AC` is *green*, `WA` *red*, `TLE` *blue*, `RE` *purple*
    pub fn result_convert(tcr: &TestCaseResult) -> ColoredString {
        (match tcr {
            Self::Accepted => "AC".green(),
            Self::WrongAnswer (_) => "WA".red(),
            Self::TimeLimitError => "TLE".blue(),
            Self::RuntimeError (_) => "RE".purple(),
        })
        .bold()
    }
}




#[cfg(test)]
mod tests {
    use crate::testcase_result;

    #[test]
    fn test1() {
        println!(
            "{}",
            testcase_result::TestCaseResult::result_convert(
                &testcase_result::TestCaseResult::Accepted
            )
        );
    }
    #[test]
    fn test2() {
        println!(
            "{}",
            testcase_result::TestCaseResult::result_convert(
                &testcase_result::TestCaseResult::WrongAnswer ("".to_string())
            )
        );
    }

    #[test]
    fn test3() {
        println!(
            "{}",
            testcase_result::TestCaseResult::result_convert(
                &testcase_result::TestCaseResult::RuntimeError ("".to_string())
            )
        );
    }

    #[test]
    fn test4() {
        println!(
            "{}",
            testcase_result::TestCaseResult::result_convert(
                &testcase_result::TestCaseResult::TimeLimitError
            )
        );
    }
}
