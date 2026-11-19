// Slint の UI ファイルをビルド時にコンパイルする
fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint_build::compile("ui/main.slint")?;
    Ok(())
}
