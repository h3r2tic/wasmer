// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build (build/wasitests.rs).

#[test]
fn test_quine() {
    assert_wasi_output!(
        "../../wasitests/quine.wasm",
        "quine",
        vec![".".to_string(),],
        vec![],
        vec![],
        "../../wasitests/quine.out"
    );
}
