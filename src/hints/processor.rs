use std::rc::Rc;

use crate::{
	hints,
	hints::hint_processor::function_like_hint_processor::{FunctionLikeHintProcessor, HintFunc},
};

/// Create, setup, and return a `FunctionLikeHintProcessor` supporting our custom hints.
///
/// This function initializes a hint processor that is capable of handling custom hints
/// such as skipping instructions, mocking calls, and expecting reverts during the execution
/// of tests. The processor is designed to integrate seamlessly with the existing testing
/// framework, allowing users to specify behaviors for their contracts without modifying
/// the actual contract code.
///
/// The following custom hints are supported:
/// - **skip**: Allows skipping certain instructions in the test execution.
/// - **mock_call**: Simulates a call to another function or contract, enabling isolation
///   of the unit being tested.
/// - **expect_revert**: Asserts that a certain condition leads to a revert, allowing
///   tests to confirm that the correct error handling is in place.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// # use crate::hints::hint_processor::function_like_hint_processor::setup_hint_processor;
/// # fn test() {
///     let hint_processor = setup_hint_processor();
/// # }
/// ```
///
pub fn setup_hint_processor() -> FunctionLikeHintProcessor {
	let skip_hint = Rc::new(HintFunc(Box::new(hints::skip)));
	let mock_call_hint = Rc::new(HintFunc(Box::new(hints::mock_call)));
	let expect_revert_hint = Rc::new(HintFunc(Box::new(hints::expect_revert)));
	let mut hint_processor = FunctionLikeHintProcessor::new_empty();
	hint_processor.add_hint(String::from("skip"), skip_hint);
	hint_processor.add_hint(String::from("expect_revert"), expect_revert_hint);
	hint_processor.add_hint(String::from("mock_call"), mock_call_hint);
	hint_processor
}
