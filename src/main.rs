use std::{collections::BTreeMap, fs, path::PathBuf};
use base64::{engine::general_purpose, Engine};
use clap::{Parser, ValueEnum};
use mime_guess::MimeGuess;
use serde::Serialize;
use walkdir::WalkDir;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to the traits directory
    #[arg(short, long)]
    path: String,

    /// Output format for image data (raw svg or base64)
    #[arg(short, long, value_enum, default_value = "raw")]
    format: ImageFormat,

    /// Output file
    #[arg(short, long, default_value = "traits.json")]
    output: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ImageFormat {
    Raw,
    Base64,
}

#[derive(Serialize)]
struct Trait {
    name: String,
    filename: String,
    data: String,
    rarity: f32,
    order: u32,
}

const SUPPORTED_EXTENSIONS: [&str; 5] = ["svg", "png", "jpg", "jpeg", "webp"];

fn process_traits_directory(args: &Args, traits_map: &mut BTreeMap<String, Vec<Trait>>) -> Result<()> {
    for entry in fs::read_dir(&args.path)? {
        let folder: fs::DirEntry = entry?;
        if !folder.path().is_dir() {
            continue;
        }

        let category: String = folder.file_name().into_string().unwrap();
        let mut order: u32 = 1;

        for file in WalkDir::new(folder.path())
            .into_iter()
            .flatten()
            .filter(|e| e.file_type().is_file())
        {
            process_trait_file(args, traits_map, &category, &mut order, file.path().to_path_buf())?;
        }
    }
    Ok(())
}

fn process_trait_file(
    args: &Args,
    traits_map: &mut BTreeMap<String, Vec<Trait>>,
    category: &str,
    order: &mut u32,
    path: PathBuf,
) -> Result<()> {
    let extension: String = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
        return Ok(());
    }

    let filename: String = path.file_name().unwrap().to_string_lossy().to_string();
    let stem: std::borrow::Cow<'_, str> = path.file_stem().unwrap().to_string_lossy();
    let (name, rarity) = parse_name_and_rarity(&stem);
    let data: String = read_file_data(&path, args.format, &extension)?;

    let trait_obj: Trait = Trait {
        name,
        filename,
        data,
        rarity,
        order: *order,
    };

    traits_map.entry(category.to_string()).or_default().push(trait_obj);
    *order += 1;

    Ok(())
}

fn read_file_data(path: &PathBuf, format: ImageFormat, extension: &str) -> Result<String> {
    if format == ImageFormat::Raw && extension == "svg" {
        return Ok(fs::read_to_string(path)?);
    }

    let bytes: Vec<u8> = fs::read(path)?;
    let mime_type: mime_guess::Mime = MimeGuess::from_path(path).first_or_octet_stream();
    let encoded: String = general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime_type, encoded))
}

fn save_output(output_path: &str, traits_map: &BTreeMap<String, Vec<Trait>>) -> Result<()> {
    let json: String = serde_json::to_string_pretty(traits_map)?;
    fs::write(output_path, json)?;
    Ok(())
}

fn parse_name_and_rarity(stem: &str) -> (String, f32) {
    let parts: Vec<&str> = stem.rsplitn(2, '.').collect();
    if parts.len() == 2 {
        if let Ok(rarity) = parts[0].parse::<f32>() {
            return (capitalize(parts[1]), rarity);
        }
    }
    (capitalize(stem), 1.0)
}

fn capitalize(s: &str) -> String {
    s.replace('_', " ")
    .split_whitespace()
    .filter_map(|word| {
        let mut chars = word.chars();
        let first_char = chars.next()?.to_uppercase();
        Some(first_char.chain(chars).collect::<String>())
    })
    .collect::<Vec<_>>()
    .join(" ")
}


fn main() {
    let args: Args = Args::parse();
    let mut traits_map: BTreeMap<String, Vec<Trait>> = BTreeMap::new();
    process_traits_directory(&args, &mut traits_map).unwrap();
    save_output(&args.output, &traits_map).unwrap();
    println!("✅ Exported traits to {}", args.output);
}