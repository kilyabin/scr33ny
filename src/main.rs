mod app;
mod canvas;
mod config;
mod screensaver;
mod widget;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "scr33ny", about = "OLED screensaver with Material 3 widgets", version)]
struct Cli {
    /// Path to config file (default: ~/.config/scr33ny/config.toml)
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Monitor index to display on (0 = primary)
    #[arg(short, long, default_value_t = 0)]
    monitor: usize,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the default config path
    ConfigPath,
    /// List all connected monitors
    Monitors,
    /// Start the idle daemon (auto-launch on inactivity)
    Daemon,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::ConfigPath) => {
            println!("{}", config::default_config_path().display());
            return Ok(());
        }
        Some(Commands::Monitors) => {
            print_monitors();
            return Ok(());
        }
        Some(Commands::Daemon) => {
            let cfg = config::load(cli.config.as_deref())?;
            return run_daemon(cfg);
        }
        None => {}
    }

    let cfg = config::load(cli.config.as_deref())?;

    // Tokio runtime for async widgets (weather fetch)
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()?;
    let _guard = rt.enter();

    app::run(cfg, cli.monitor)
}

fn print_monitors() {
    use winit::event_loop::EventLoop;
    let el = EventLoop::new().unwrap();
    for (i, m) in el.available_monitors().enumerate() {
        let size = m.size();
        let name = m.name().unwrap_or_else(|| "Unknown".to_string());
        println!("[{i}] {name}  {}×{}", size.width, size.height);
    }
}

fn run_daemon(cfg: config::Config) -> Result<()> {
    if !cfg.daemon.enabled {
        anyhow::bail!(
            "daemon.enabled is false in config — set it to true to use daemon mode"
        );
    }

    let timeout = cfg.daemon.idle_timeout;
    log::info!("daemon: idle timeout = {timeout}s");

    // Prefer swayidle (standard Wayland idle tool)
    let exe = std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(str::to_string))
        .unwrap_or_else(|| "scr33ny".to_string());

    let result = std::process::Command::new("swayidle")
        .args([
            "-w",
            "timeout", &timeout.to_string(), &exe,
            "resume",  "killall scr33ny",
        ])
        .status();

    match result {
        Ok(s) if s.success() => Ok(()),
        Ok(s)  => anyhow::bail!("swayidle exited: {s}"),
        Err(_) => {
            log::warn!("swayidle not found — using simple sleep loop");
            loop {
                std::thread::sleep(std::time::Duration::from_secs(timeout));
                let _ = std::process::Command::new(&exe).status();
            }
        }
    }
}
