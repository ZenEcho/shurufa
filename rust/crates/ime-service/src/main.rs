use std::{thread, time::Duration};

use anyhow::Result;
use ime_config::AppConfig;
use ime_core::{ImeEngine, KeyEvent, SessionState};
use ime_db::{HotkeyEntry, NewErrorLogEntry, NewUserDictionaryEntry};
use ime_dict::MemoryDictionary;
use ime_logging::init_logging;
use ime_service::ServiceRuntime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
struct DeleteDictionaryEntryPayload {
    id: i64,
}

#[derive(Debug, Deserialize)]
struct DeleteHotkeyPayload {
    id: String,
}

#[derive(Debug, Deserialize)]
struct ProcessTypingKeyPayload {
    session: SessionState,
    event: KeyEvent,
}

#[derive(Debug, Serialize)]
struct TypingDemoResult {
    initial_mode: String,
    first_preedit: String,
    first_candidate: Option<String>,
    selected_commit: Option<String>,
    toggled_mode: String,
    english_commit: Option<String>,
}

fn print_json<T>(value: &T) -> Result<()>
where
    T: serde::Serialize,
{
    println!("{}", serde_json::to_string(value)?);
    Ok(())
}

fn handle_json_command(service: &ServiceRuntime, args: &[String]) -> Result<bool> {
    let Some(command) = args.first().map(String::as_str) else {
        return Ok(false);
    };

    match command {
        "get-config" => {
            let config = service.get_config()?;
            print_json(&config)?;
            Ok(true)
        }
        "get-runtime-status" => {
            let status = service.get_runtime_status()?;
            print_json(&status)?;
            Ok(true)
        }
        "reset-config" => {
            let config = service.reset_config()?;
            print_json(&config)?;
            Ok(true)
        }
        "set-config" => {
            let payload = args.get(1).cloned().unwrap_or_default();
            let config: AppConfig = serde_json::from_str(&payload)?;
            let status = service.update_config(config)?;
            print_json(&status)?;
            Ok(true)
        }
        "list-user-dictionary" => {
            let entries = service.list_user_dictionary_entries()?;
            print_json(&entries)?;
            Ok(true)
        }
        "create-user-dictionary-entry" => {
            let payload = args.get(1).cloned().unwrap_or_default();
            let entry: NewUserDictionaryEntry = serde_json::from_str(&payload)?;
            let created = service.create_user_dictionary_entry(entry)?;
            service.append_error_log(NewErrorLogEntry {
                level: "info".to_string(),
                module: "desktop-settings".to_string(),
                message: format!("Created dictionary entry {}", created.id),
                context_json: None,
            })?;
            print_json(&service.list_user_dictionary_entries()?)?;
            Ok(true)
        }
        "delete-user-dictionary-entry" => {
            let payload = args.get(1).cloned().unwrap_or_default();
            let payload: DeleteDictionaryEntryPayload = serde_json::from_str(&payload)?;
            let entries = service.delete_user_dictionary_entry(payload.id)?;
            print_json(&entries)?;
            Ok(true)
        }
        "list-history" => {
            let entries = service.list_input_history_entries()?;
            print_json(&entries)?;
            Ok(true)
        }
        "list-hotkeys" => {
            let entries = service.list_hotkeys()?;
            print_json(&entries)?;
            Ok(true)
        }
        "save-hotkey" => {
            let payload = args.get(1).cloned().unwrap_or_default();
            let entry: HotkeyEntry = serde_json::from_str(&payload)?;
            let entries = service.save_hotkey(entry)?;
            print_json(&entries)?;
            Ok(true)
        }
        "delete-hotkey" => {
            let payload = args.get(1).cloned().unwrap_or_default();
            let payload: DeleteHotkeyPayload = serde_json::from_str(&payload)?;
            let entries = service.delete_hotkey(&payload.id)?;
            print_json(&entries)?;
            Ok(true)
        }
        "list-error-logs" => {
            let entries = service.list_error_logs()?;
            print_json(&entries)?;
            Ok(true)
        }
        "create-typing-session" => {
            let session = service.create_typing_session()?;
            print_json(&session)?;
            Ok(true)
        }
        "process-typing-key" => {
            let payload = args.get(1).cloned().unwrap_or_default();
            let payload: ProcessTypingKeyPayload = serde_json::from_str(&payload)?;
            let snapshot = service.process_typing_key(payload.session, payload.event)?;
            print_json(&snapshot)?;
            Ok(true)
        }
        "run-typing-demo" => {
            let session = service.create_typing_session()?;
            let initial_mode = format!("{:?}", session.input_mode.clone());
            let first = service.process_typing_key(session, KeyEvent::Char('s'))?;
            let selected =
                service.process_typing_key(first.session.clone(), KeyEvent::Number(1))?;
            let toggled =
                service.process_typing_key(selected.session.clone(), KeyEvent::ToggleInputMode)?;
            let english =
                service.process_typing_key(toggled.session.clone(), KeyEvent::Char('x'))?;

            print_json(&TypingDemoResult {
                initial_mode,
                first_preedit: first.response.preedit.composition_text,
                first_candidate: first
                    .response
                    .candidates
                    .items
                    .first()
                    .map(|item| item.text.clone()),
                selected_commit: selected.response.commit_text,
                toggled_mode: format!("{:?}", toggled.response.input_mode),
                english_commit: english.response.commit_text,
            })?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let oneshot = args.iter().any(|arg| arg == "--oneshot");
    let json_mode = args.iter().position(|arg| arg == "--json");

    init_logging();

    let service = ServiceRuntime::new("shurufa.db")?;

    if let Some(json_index) = json_mode {
        let command_args = args
            .iter()
            .skip(json_index + 1)
            .cloned()
            .collect::<Vec<_>>();
        if handle_json_command(&service, &command_args)? {
            return Ok(());
        }
    }

    let config = service.get_config()?;
    let engine = ImeEngine::new(MemoryDictionary);
    let mut session = SessionState::default();

    let _ = engine.handle_key_event(&mut session, KeyEvent::Char('s'));
    let _ = engine.handle_key_event(&mut session, KeyEvent::Char('h'));
    let response = engine.handle_key_event(&mut session, KeyEvent::Space);

    println!(
        "书入法服务脚手架已启动。default_schema={}, commit={:?}",
        config.input.default_schema, response.commit_text
    );

    if oneshot {
        return Ok(());
    }

    println!("书入法开发服务运行中。按 Ctrl+C 停止。");

    loop {
        thread::sleep(Duration::from_secs(60));
    }

    #[allow(unreachable_code)]
    Ok(())
}
