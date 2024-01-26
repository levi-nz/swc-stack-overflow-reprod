use std::path::PathBuf;
use std::sync::Arc;
use swc::config::Options;
use swc::Compiler;
use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::errors::{ColorConfig, Handler};
use swc_core::common::{FileName, Globals, SourceMap, GLOBALS};
use swc_core::ecma::transforms::base::pass::noop;

fn main() {
    // Input file
    let src = PathBuf::from("input.js");
    // Output file
    let dst = PathBuf::from("output.js");

    // Plaintext code
    let code = std::fs::read_to_string(&src).expect("failed to read src");

    // Setup SWC
    let cm = Arc::<SourceMap>::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let compiler = Compiler::new(cm.clone());
    let fm = cm.new_source_file(FileName::Real(src), code.clone());

    let globals = Globals::new();
    GLOBALS.set(&globals, || {
        // Process JS
        let output = compiler
            .process_js_with_custom_pass(
                fm,
                None,
                &handler,
                &Options::default(),
                SingleThreadedComments::default(),
                |_| noop(),
                |_| noop(),
            )
            .expect("process_js_with_custom_pass failed");

        // Write output
        std::fs::write(&dst, output.code).expect("std::fs::write failed");
    });
}