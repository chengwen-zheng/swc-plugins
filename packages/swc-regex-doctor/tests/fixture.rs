use std::path::PathBuf;
use swc_core::ecma::visit::as_folder;
use swc_ecma_transforms_testing::test_fixture;

use swc_regex_doctor::TransformVisitor;
use testing::fixture;

#[fixture("tests/fixture/simple/**/input.js")]
fn regex_test_and_match(input: PathBuf) {
    let output: PathBuf = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|_t| as_folder(TransformVisitor),
        &input,
        &output,
        Default::default(),
    );
}
