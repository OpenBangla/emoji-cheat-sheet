use std::{
    collections::BTreeMap,
    env::var_os,
    fs::{read, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use askama::Template;
use emojicon::internal::{bn_emojis, emojis as gh_emojis, emoticons};
use serde::Deserialize;

#[derive(Deserialize)]
struct Emoji {
    emoji: String,
    category: String,
}

struct Data {
    emoji: String,
    codes: Vec<String>,
}

struct Section {
    section: String,
    emojis: Vec<Data>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Page {
    sections: Vec<Section>,
}

fn main() {
    let root = PathBuf::from(var_os("CARGO_MANIFEST_DIR").unwrap());
    let dest = root.join("render").join("index.html");

    let mut file = BufWriter::new(File::create(dest).unwrap());
    let source = read(root.join("data/emoji.json")).unwrap();
    let emojis: Vec<Emoji> = serde_json::from_slice(&source).unwrap();

    let mut categorized: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut emoji_table: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for Emoji { emoji, category } in emojis {
        categorized.entry(category).or_default().push(emoji.clone());
    }

    // Github inspired shortcodes
    for (code, emoji) in gh_emojis() {
        for item in emoji {
            emoji_table
                .entry(item.to_string())
                .or_default()
                .push(code.to_string());
        }
    }

    // Bengali Emoji
    for (code, emoji) in bn_emojis() {
        for item in emoji {
            emoji_table
                .entry(item.to_string())
                .or_default()
                .push(code.to_string());
        }
    }

    // Emoticon
    for (code, emoji) in emoticons() {
        emoji_table
            .entry(emoji.to_string())
            .or_default()
            .push(code.to_string());
    }

    // Closure to add a section to sections.
    let add_section = |section: &str, sections: &mut Vec<Section>| {
        let mut datas = Vec::new();
        for emoji in categorized.get(section).unwrap() {
            if let Some(codes) = emoji_table.get(emoji) {
                let data = Data {
                    emoji: emoji.to_owned(),
                    codes: codes.clone(),
                };
                datas.push(data);
            }
        }
        sections.push(Section {
            section: section.to_owned(),
            emojis: datas,
        });
    };

    // Sections
    let mut sections = Vec::new();
    add_section("Smileys & Emotion", &mut sections);
    add_section("People & Body", &mut sections);
    add_section("Animals & Nature", &mut sections);
    add_section("Activities", &mut sections);
    add_section("Objects", &mut sections);
    add_section("Food & Drink", &mut sections);
    add_section("Symbols", &mut sections);
    add_section("Travel & Places", &mut sections);
    add_section("Flags", &mut sections);

    let page = Page { sections };

    let page = page.render().unwrap();
    write!(&mut file, "{page}").unwrap();
}
