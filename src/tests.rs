pub struct Test {
    pub func: fn(),
    pub path: &'static str,
}

pub(crate) fn test_runner(tests: &[&Test]) {
    log::info!("running {} tests", tests.len());

    let mut passed = 0usize;

    for test in tests {
        (test.func)();
        log::info!("test {} ... ok", test.path);

        passed += 1;
    }

    log::info!("");
    log::info!(
        "test result: ok. {} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out",
        passed
    );
}
