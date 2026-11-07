//! Asset loading and management.

use std::{collections::HashMap, fmt::Debug, fs, hash::Hash, path::{Path, PathBuf}};

pub struct Assets {
    img: HashMap<String, Image>,
    audio: HashMap<String, Sample>,
}

pub struct AssetError(pub String);

impl From<std::io::Error> for AssetError {
    fn from(e: std::io::Error) -> Self {
        Self(e.to_string())
    }
}

impl Assets {
    pub fn load(dir: impl AsRef<Path>) -> Result<Self, AssetError> {
        let dir = dir.as_ref();
        Ok(Self {
            img: Self::load_images(&dir.join("img"))?,
            audio: Self::load_samples(&dir.join("audio"))?,
        })
    }

    fn load_images(dir: &Path) -> Result<HashMap<String, Image>, AssetError> {
        walk_dir(dir)
    }

    fn load_samples(dir: &Path) -> Result<HashMap<String, Sample>, AssetError> {
        walk_dir(dir)
    }
}

/// Recursively get all asset files in a directory (helper function).
fn walk_dir<A: Asset>(dir: &Path) -> Result<HashMap<String, A>, AssetError> {
    let mut output = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            join(&mut output, walk_dir(&path)?)?;
        } else if !path.is_file() { continue; }

        let Some(ext) = path.extension() else { continue; };
        if !ext.eq_ignore_ascii_case(A::FILE_EXT) { continue; }

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| AssetError("non-utf8 filename".into()))?
            .to_owned();

            output.insert(name, A::from_path(&path)?);
        }

    Ok(output)
}

trait Asset : Sized + PartialEq {
    const FILE_EXT: &'static str;
    fn from_path(p: &Path) -> Result<Self, AssetError>;
}

#[derive(PartialEq)]
pub struct Image {
    path: PathBuf,
    width: usize,
    height: usize,
}

impl Asset for Image {
    const FILE_EXT: &'static str = "png";

    fn from_path(path: &Path) -> Result<Self, AssetError> {
        let bytes = fs::read(path)?;
        if bytes.len() < 24 || &bytes[1..4] != Self::FILE_EXT.to_ascii_uppercase().as_bytes() {
            return Err(AssetError(format!("not a {}: {}", Self::FILE_EXT, path.display())));
        }
        let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap()) as usize;
        let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap()) as usize;

        Ok(Self {
            path: path.to_path_buf(),
            width,
            height,
        })
    }
}

#[derive(PartialEq)]
pub struct Sample {
    path: PathBuf,
    sample_rate: usize,
    duration: usize,
    bit_depth: usize,
}

impl Asset for Sample {
    const FILE_EXT: &'static str = "wav";

    fn from_path(p: &Path) -> Result<Self, AssetError> {
        todo!()
    }
}

/// Join two HashMaps returing an error if a key points to different values (helper function).
fn join<K, V>(
    left: &mut HashMap<K, V>, 
    right: HashMap<K, V>,
) -> Result<(), AssetError>
where 
    K: Eq + Hash + Clone + Debug,
    V: PartialEq,
{
    for (key, value) in &right {
        if left.get(key).is_some_and(|existing| existing != value) {
            return Err(AssetError(format!("Key {:#?} points to different values", key.clone())));
        }
    }

    left.extend(right);
    Ok(())
}