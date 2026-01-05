use std::path::Path;
use std::process::Command;

pub fn open_path(path: &Path) {
    #[cfg(target_os = "windows")]
    {
        // start "" "<path>"
        let _ = Command::new("cmd")
            .args(["/C", "start", "", path.to_string_lossy().as_ref()])
            .spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(path).spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(path).spawn();
    }
}
