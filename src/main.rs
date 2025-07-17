mod cli;
mod cli_commands;
mod cli_handlers;
#[cfg(feature = "gui")]
mod components;
mod config;
mod database;
mod git;
mod models;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    // Check if running in CLI mode
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        // CLI mode
        env_logger::init();

        let cli_args = cli::Cli::parse();

        // Create a new runtime for CLI mode
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(async {
            // Load configuration
            let config = config::Config::load().await?;

            // Initialize database
            let db = database::Database::new(&config.database_path()).await?;

            match cli_args.command {
                cli::Commands::Exec {
                    command,
                    mode,
                    branch,
                    worktree,
                    skip_permissions,
                    continue_from_last,
                } => {
                    cli_commands::execute_command_immediate(
                        &command,
                        &mode,
                        branch.as_deref(),
                        worktree,
                        cli_args.verbose,
                        skip_permissions,
                        continue_from_last,
                    )
                    .await?;
                }
                cli::Commands::Schedule {
                    command,
                    time,
                    date,
                    mode,
                    branch,
                    worktree,
                    memo,
                    skip_permissions,
                    continue_from_last,
                } => {
                    cli_commands::schedule_command(
                        &db,
                        &command,
                        &time,
                        &date,
                        &mode,
                        branch.as_deref(),
                        worktree,
                        memo.as_deref(),
                        skip_permissions,
                        continue_from_last,
                    )
                    .await?;
                }
                cli::Commands::List {
                    status,
                    format,
                    limit,
                } => {
                    cli_handlers::list_schedules(&db, status.as_deref(), &format, limit).await?;
                }
                cli::Commands::History {
                    status,
                    exec_type,
                    branch,
                    format,
                    limit,
                    from,
                    to,
                } => {
                    cli_handlers::show_history(
                        &db,
                        status.as_deref(),
                        exec_type.as_deref(),
                        branch.as_deref(),
                        &format,
                        limit,
                        from,
                        to,
                    )
                    .await?;
                }
                cli::Commands::Daemon {
                    port,
                    interval,
                    pid_file,
                    log_file,
                    detach,
                } => {
                    if detach {
                        // TODO: Implement proper daemonization
                        eprintln!("Detach mode not yet implemented. Running in foreground.");
                    }

                    cli_handlers::run_daemon(
                        &db,
                        port,
                        interval,
                        pid_file.as_deref(),
                        log_file.as_deref(),
                        detach,
                    )
                    .await?;
                }
                cli::Commands::Config { action } => match action {
                    cli::ConfigAction::Show => {
                        let all_config = db.get_all_config().await?;
                        for (key, value) in all_config {
                            println!("{key} = {value}");
                        }
                    }
                    cli::ConfigAction::Get { key } => {
                        if let Some(value) = db.get_config(&key).await? {
                            println!("{value}");
                        } else {
                            eprintln!("Key '{key}' not found");
                            std::process::exit(1);
                        }
                    }
                    cli::ConfigAction::Set { key, value } => {
                        db.set_config(&key, &value).await?;
                        println!("✅ Configuration updated: {key} = {value}");
                    }
                },
            }
            Ok::<(), anyhow::Error>(())
        })?
    } else {
        // GUI mode
        #[cfg(feature = "gui")]
        {
            println!("Starting Claude Scheduler...");
            println!("Initializing Dioxus application...");
            println!("Open http://localhost:8080 in your browser to access the application.");

            dioxus::launch(components::app);

            println!("Dioxus application ended.");
        }

        #[cfg(not(feature = "gui"))]
        {
            eprintln!("GUI mode is not available. Please install with GUI features enabled.");
            eprintln!("Use: cargo install --features gui");
            std::process::exit(1);
        }
    }

    Ok(())
}
