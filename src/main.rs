use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{env::temp_dir, fs::rename, iter::repeat, path::PathBuf};

fn main() -> Result<(), String> {
    let folder = std::env::args()
        .nth(1)
        .map(|s| s.trim_end_matches('/').to_owned())
        .map(PathBuf::from)
        .and_then(|path| if path.is_dir() { Some(path) } else { None })
        .ok_or_else(|| String::from("You must pass a folder as a second argument!"))?;

    debug_assert!(folder.exists());

    let folder_name = folder
        .to_str()
        .map(String::from)
        .ok_or_else(|| format!("Could not convert {} to a string", folder.display()))?;

    let with_noise = folder_name.clone() + "-" + &get_noise();
    let path = temp_dir().join(with_noise);

    match rename(&folder, path) {
        Ok(()) => {
            println!("👋 bye '{folder_name}'!");
            Ok(())
        }
        Err(e) => Err(format!("Could not throw away {folder_name}: {e}")),
    }
}

fn get_noise() -> String {
    let mut rng = thread_rng();
    repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(16)
        .collect()
}
