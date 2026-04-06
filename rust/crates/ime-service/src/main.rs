use std::{thread, time::Duration};

use anyhow::Result;
use ime_core::{ImeEngine, KeyEvent, SessionState};
use ime_db::Database;
use ime_dict::MemoryDictionary;
use ime_logging::init_logging;

fn main() -> Result<()> {
    let oneshot = std::env::args().any(|arg| arg == "--oneshot");

    init_logging();

    let db = Database::new("shurufa.db");
    db.initialize()?;
    let config = db.load_config()?;
    let engine = ImeEngine::new(MemoryDictionary);
    let mut session = SessionState::default();

    let _ = engine.handle_key_event(&mut session, KeyEvent::Char('s'));
    let _ = engine.handle_key_event(&mut session, KeyEvent::Char('h'));
    let response = engine.handle_key_event(&mut session, KeyEvent::Space);

    println!(
        "Shurufa service scaffold ready. default_schema={}, commit={:?}",
        config.input.default_schema, response.commit_text
    );

    if oneshot {
        return Ok(());
    }

    println!("Shurufa dev service running. Press Ctrl+C to stop.");

    loop {
        thread::sleep(Duration::from_secs(60));
    }

    #[allow(unreachable_code)]
    Ok(())
}
