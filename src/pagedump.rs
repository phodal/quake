use quake_core::QuakeConfig;

// export data for GitHub pages as demo
pub fn page_dump(conf: QuakeConfig) {
    // 1. dump entries config;
    dump_entries_define(&conf);
    // 2. dump quake information;
    dump_transflow();
    dump_layout();
    dump_links();
    // 3. export all entry_type data to json
    dump_entries_data();
}

fn dump_transflow() {
    todo!()
}

fn dump_layout() {
    todo!()
}

fn dump_links() {
    todo!()
}

fn dump_entries_define(_conf: &QuakeConfig) {}

fn dump_entries_data() {
    todo!()
}
