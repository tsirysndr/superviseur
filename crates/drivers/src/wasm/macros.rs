macro_rules! check_wasm_runtime {
    ($runtimes:expr, $runtime:expr) => {
        $runtimes
            .into_iter()
            .any(|(runtime, _)| runtime == $runtime)
    };
}

pub(crate) use check_wasm_runtime;
