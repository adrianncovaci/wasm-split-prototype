//! A caller that has no `tracing` subscriber relies on `tracing` turning its events into `log`
//! records. Running a split must leave that fallback in place. This file is its own test
//! binary, so no other test can install a subscriber in the same process.

use std::sync::Mutex;

use wasm_encoder::{LinkingSection, Module, RefType, SymbolTable, TableSection, TableType};
use wasm_split_cli_support::{transform, Options};

struct RecordingLogger(Mutex<Vec<String>>);

impl log::Log for RecordingLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        self.0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(record.args().to_string());
    }

    fn flush(&self) {}
}

static LOGGER: RecordingLogger = RecordingLogger(Mutex::new(Vec::new()));

#[test]
fn caller_events_still_reach_log_after_a_split() {
    log::set_logger(&LOGGER).expect("no logger was installed before");
    log::set_max_level(log::LevelFilter::Info);

    let mut module = Module::new();
    let mut tables = TableSection::new();
    tables.table(TableType {
        element_type: RefType::FUNCREF,
        minimum: 0,
        maximum: None,
        table64: false,
        shared: false,
    });
    module.section(&tables);
    let mut symbols = SymbolTable::new();
    symbols.table(
        SymbolTable::WASM_SYM_BINDING_LOCAL,
        0,
        Some("__indirect_function_table"),
    );
    let mut linking = LinkingSection::new();
    linking.symbol_table(&symbols);
    module.section(&linking);
    let input = module.finish();
    let tmp = tempfile::tempdir().expect("create tmpdir");
    let main_out = tmp.path().join("main.wasm");
    let mut opts = Options::new(&input);
    opts.output_dir = tmp.path();
    opts.main_out_path = &main_out;
    transform(opts).expect("transform succeeds");

    tracing::info!("after the split");

    let lines = LOGGER
        .0
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    assert!(
        lines.iter().any(|line| line == "after the split"),
        "the caller's event did not reach the logger: {lines:?}"
    );
}
