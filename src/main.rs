use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "fledge-tz",
    version,
    about = "Show, convert, and manage timezone preferences",
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Add a zone to your saved list (e.g. `tz add PST` or `tz add Asia/Tokyo`)
    Add { zone: String },
    /// Remove a zone from your saved list
    Rm { zone: String },
    /// List your saved zones
    List,
    /// Show current time in the given zones (does not modify saved list)
    Now { zones: Vec<String> },
    /// Convert a time across zones — e.g. `tz convert "3pm PST" EST UTC`
    Convert {
        /// The time to convert (e.g. "3pm PST", "15:00 UTC", "2026-04-25 09:30 America/New_York")
        time: String,
        /// Target zones to display the converted time in
        targets: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => show_saved(),
        Some(Commands::Add { zone }) => add_zone(&zone),
        Some(Commands::Rm { zone }) => remove_zone(&zone),
        Some(Commands::List) => list_zones(),
        Some(Commands::Now { zones }) => show_now(&zones),
        Some(Commands::Convert { time, targets }) => convert(&time, &targets),
    }
}

fn show_saved() -> Result<()> {
    let zones = load_zones()?;
    if zones.is_empty() {
        println!("No saved zones. Add one with: fledge tz add <ZONE>");
        println!("Showing local time and UTC:");
        let defaults = vec!["Local".to_string(), "UTC".to_string()];
        return show_now(&defaults);
    }
    show_now(&zones)
}

fn show_now(zones: &[String]) -> Result<()> {
    if zones.is_empty() {
        bail!("Provide at least one zone (e.g. `tz now UTC PST`)");
    }
    let now_utc = Utc::now();
    let label_width = zones.iter().map(|z| z.len()).max().unwrap_or(0);

    for zone in zones {
        let formatted = format_in_zone(now_utc, zone)?;
        println!("  {:<width$}  {}", zone, formatted, width = label_width);
    }
    Ok(())
}

fn add_zone(input: &str) -> Result<()> {
    let resolved = resolve_zone(input)?;
    let mut zones = load_zones()?;
    if zones.iter().any(|z| z.eq_ignore_ascii_case(&resolved)) {
        println!("'{}' is already saved.", resolved);
        return Ok(());
    }
    zones.push(resolved.clone());
    save_zones(&zones)?;
    println!("Added '{}'.", resolved);
    Ok(())
}

fn remove_zone(input: &str) -> Result<()> {
    let mut zones = load_zones()?;
    let before = zones.len();
    zones.retain(|z| !z.eq_ignore_ascii_case(input));
    if zones.len() == before {
        println!("'{}' was not saved.", input);
    } else {
        save_zones(&zones)?;
        println!("Removed '{}'.", input);
    }
    Ok(())
}

fn list_zones() -> Result<()> {
    let zones = load_zones()?;
    if zones.is_empty() {
        println!("No saved zones.");
    } else {
        for zone in &zones {
            println!("  {}", zone);
        }
    }
    Ok(())
}

fn convert(time_input: &str, targets: &[String]) -> Result<()> {
    let (parsed_utc, source_zone) = parse_time(time_input)?;

    let display_targets: Vec<String> = if targets.is_empty() {
        let saved = load_zones()?;
        if saved.is_empty() {
            vec!["Local".to_string(), "UTC".to_string()]
        } else {
            saved
        }
    } else {
        targets.to_vec()
    };

    println!("  {:<width$}  {}", source_zone, time_input, width = 12);
    let label_width = display_targets.iter().map(|z| z.len()).max().unwrap_or(0);
    for zone in &display_targets {
        let formatted = format_in_zone(parsed_utc, zone)?;
        println!("  {:<width$}  {}", zone, formatted, width = label_width);
    }
    Ok(())
}

fn parse_time(input: &str) -> Result<(DateTime<Utc>, String)> {
    let trimmed = input.trim();

    // Split off the trailing zone token (last whitespace-separated chunk).
    let (time_part, zone_part) = match trimmed.rsplit_once(char::is_whitespace) {
        Some((t, z)) => (t.trim(), z.trim()),
        None => bail!(
            "Could not parse '{}'. Expected `<time> <zone>`, e.g. `3pm PST` or `15:00 UTC`.",
            input
        ),
    };

    let resolved_zone = resolve_zone(zone_part)?;
    let tz: Tz = resolved_zone
        .parse()
        .map_err(|_| anyhow!("Unknown IANA zone: {}", resolved_zone))?;

    let naive = parse_naive_time(time_part)?;
    let local = tz
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| anyhow!("Ambiguous or invalid local time '{}'", time_part))?;

    Ok((local.with_timezone(&Utc), resolved_zone))
}

fn parse_naive_time(input: &str) -> Result<NaiveDateTime> {
    let today = Local::now().date_naive();
    let trimmed = input.trim();

    let iso_formats: &[&str] = &["%Y-%m-%d %H:%M", "%Y-%m-%d %H:%M:%S"];
    for fmt in iso_formats {
        if let Ok(dt) = NaiveDateTime::parse_from_str(trimmed, fmt) {
            return Ok(dt);
        }
    }

    if let Ok(t) = chrono::NaiveTime::parse_from_str(trimmed, "%H:%M") {
        return Ok(today.and_time(t));
    }

    let lower = trimmed.to_lowercase().replace(' ', "");
    let (time_part, am_pm_offset): (&str, u32) =
        if let Some(stripped) = lower.strip_suffix("pm") {
            (stripped, 12)
        } else if let Some(stripped) = lower.strip_suffix("am") {
            (stripped, 0)
        } else {
            bail!(
                "Could not parse time '{}'. Try formats like '3pm', '15:00', or '2026-04-25 09:30'.",
                input
            );
        };

    let (hour, minute): (u32, u32) = if let Some((h, m)) = time_part.split_once(':') {
        let h: u32 = h
            .parse()
            .with_context(|| format!("invalid hour in '{input}'"))?;
        let m: u32 = m
            .parse()
            .with_context(|| format!("invalid minute in '{input}'"))?;
        (h, m)
    } else {
        let h: u32 = time_part
            .parse()
            .with_context(|| format!("invalid hour in '{input}'"))?;
        (h, 0)
    };

    if hour == 0 || hour > 12 {
        bail!("12-hour time must use 1–12 (got {})", hour);
    }
    let hour_24 = if hour == 12 {
        am_pm_offset
    } else {
        hour + am_pm_offset
    };

    let t = chrono::NaiveTime::from_hms_opt(hour_24, minute, 0)
        .ok_or_else(|| anyhow!("Invalid time '{}'", input))?;
    Ok(today.and_time(t))
}

fn format_in_zone(utc: DateTime<Utc>, zone_input: &str) -> Result<String> {
    if zone_input.eq_ignore_ascii_case("local") {
        let local = utc.with_timezone(&Local);
        return Ok(local.format("%Y-%m-%d %H:%M %Z").to_string());
    }

    let resolved = resolve_zone(zone_input)?;
    let tz: Tz = resolved
        .parse()
        .map_err(|_| anyhow!("Unknown IANA zone: {}", resolved))?;
    let in_zone = utc.with_timezone(&tz);
    Ok(in_zone.format("%Y-%m-%d %H:%M %Z").to_string())
}

/// Map common shortcuts (PST, EST, JST, ...) to IANA zone names.
/// IANA names pass through unchanged after a validation lookup.
fn resolve_zone(input: &str) -> Result<String> {
    let upper = input.to_uppercase();

    let alias = match upper.as_str() {
        "LOCAL" => return Ok("Local".to_string()),
        "PST" | "PDT" | "PT" => Some("America/Los_Angeles"),
        "MST" | "MDT" | "MT" => Some("America/Denver"),
        "CST" | "CDT" | "CT" => Some("America/Chicago"),
        "EST" | "EDT" | "ET" => Some("America/New_York"),
        "AKST" | "AKDT" => Some("America/Anchorage"),
        "HST" => Some("Pacific/Honolulu"),
        "UTC" | "Z" => Some("UTC"),
        "GMT" => Some("GMT"),
        "BST" => Some("Europe/London"),
        "CET" | "CEST" => Some("Europe/Paris"),
        "EET" | "EEST" => Some("Europe/Helsinki"),
        "IST" => Some("Asia/Kolkata"),
        "JST" => Some("Asia/Tokyo"),
        "KST" => Some("Asia/Seoul"),
        "AEST" | "AEDT" => Some("Australia/Sydney"),
        "NZST" | "NZDT" => Some("Pacific/Auckland"),
        _ => None,
    };

    if let Some(iana) = alias {
        return Ok(iana.to_string());
    }

    // Try as an IANA name directly.
    let _: Tz = input.parse().map_err(|_| {
        anyhow!(
            "Unknown zone '{}'. Use an IANA name (America/Los_Angeles) or shortcut (PST, JST).",
            input
        )
    })?;
    Ok(input.to_string())
}

#[derive(Serialize, Deserialize, Default)]
struct State {
    zones: Vec<String>,
}

fn state_path() -> Result<PathBuf> {
    let dir = std::env::var("FLEDGE_PLUGIN_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".fledge")
                .join("plugins")
                .join("fledge-plugin-tz")
        });
    fs::create_dir_all(&dir).context("creating plugin state directory")?;
    Ok(dir.join("state.json"))
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

fn load_zones() -> Result<Vec<String>> {
    let path = state_path()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let raw = fs::read_to_string(&path).context("reading state.json")?;
    let state: State = serde_json::from_str(&raw).unwrap_or_default();
    Ok(state.zones)
}

fn save_zones(zones: &[String]) -> Result<()> {
    let path = state_path()?;
    let state = State {
        zones: zones.to_vec(),
    };
    let json = serde_json::to_string_pretty(&state).context("serializing state")?;
    fs::write(&path, json).context("writing state.json")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias_pst_resolves() {
        assert_eq!(resolve_zone("PST").unwrap(), "America/Los_Angeles");
        assert_eq!(resolve_zone("pst").unwrap(), "America/Los_Angeles");
    }

    #[test]
    fn alias_jst_resolves() {
        assert_eq!(resolve_zone("JST").unwrap(), "Asia/Tokyo");
    }

    #[test]
    fn iana_passes_through() {
        assert_eq!(
            resolve_zone("America/New_York").unwrap(),
            "America/New_York"
        );
    }

    #[test]
    fn unknown_zone_errors() {
        assert!(resolve_zone("XYZ").is_err());
    }

    #[test]
    fn parse_time_3pm_pst() {
        let (utc, zone) = parse_time("3pm PST").unwrap();
        assert_eq!(zone, "America/Los_Angeles");
        // Hour-of-day in PST should be 15:00 local, which is 22:00 or 23:00 UTC depending on DST.
        let pst: Tz = zone.parse().unwrap();
        assert_eq!(utc.with_timezone(&pst).format("%H:%M").to_string(), "15:00");
    }

    #[test]
    fn parse_time_15_00_utc() {
        let (utc, zone) = parse_time("15:00 UTC").unwrap();
        assert_eq!(zone, "UTC");
        assert_eq!(utc.format("%H:%M").to_string(), "15:00");
    }

    #[test]
    fn parse_time_full_iso() {
        let (utc, _) = parse_time("2026-04-25 09:30 UTC").unwrap();
        assert_eq!(utc.format("%Y-%m-%d %H:%M").to_string(), "2026-04-25 09:30");
    }

    #[test]
    fn parse_time_missing_zone_errors() {
        assert!(parse_time("3pm").is_err());
    }
}
