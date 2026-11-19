slint::include_modules!();

/// メモリ上の SQLite 接続を開いて SQLite のバージョンを取得する
fn sqlite_version() -> anyhow::Result<String> {
    let conn = rusqlite::Connection::open_in_memory()?;
    let version: String = conn.query_row("SELECT sqlite_version()", [], |row| row.get(0))?;
    Ok(version)
}

fn main() -> anyhow::Result<()> {
    let version = sqlite_version()?;
    println!("SQLite version: {version}");

    let ui = SpikeWindow::new()?;
    ui.run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqlite_version_returns_v3() {
        let version = sqlite_version().expect("メモリ上の SQLite バージョン取得に失敗");
        assert!(
            version.starts_with("3."),
            "想定外の SQLite バージョン: {version}"
        );
    }
}
