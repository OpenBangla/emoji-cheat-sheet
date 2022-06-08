use std::{env::var_os, path::PathBuf, io::{BufWriter, Write}, fs::{File, read}, collections::BTreeMap, fmt::write};

use serde::Deserialize;

#[derive(Deserialize)]
struct Emoticon {
    emoji: String,
    emoticons: Vec<String>,
}

#[derive(Deserialize)]
struct Emoji {
    emoji: String,
    category: String,
    aliases: Vec<String>,
    tags: Vec<String>,
}

fn main() {
    let root = PathBuf::from(var_os("CARGO_MANIFEST_DIR").unwrap());
    //let parent = root.parent().unwrap();
    let dest = root.join("index.html");

    let mut file = BufWriter::new(File::create(dest).unwrap());
    let source = read(root.join("data/emoji.json")).unwrap();
    let emojis: Vec<Emoji> = serde_json::from_slice(&source).unwrap();

    let mut categorized: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut emoji_table: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for Emoji { emoji, category, aliases, tags } in emojis {
        categorized.entry(category).or_default().push(emoji.clone());

        for alias in aliases {
            emoji_table.entry(emoji.clone()).or_default().push(alias);

        }

        for tag in tags {
            emoji_table.entry(emoji.clone()).or_default().push(tag);
        }
    }

    write!(&mut file, "<html>\n<body>").unwrap();

    for (section, emojis) in categorized {
        write!(&mut file, "\n<p>{section}</p>\n").unwrap();
        write!(&mut file, "<table>\n<tr>\n").unwrap();
        for emoji in emojis {
            write!(&mut file, "<td>{emoji}</td>\n<td>").unwrap();
            for code in emoji_table.get(&emoji).unwrap() {
                write!(&mut file, "{code},").unwrap();
            }
            write!(&mut file, "</td>\n</tr>").unwrap();
        }
        write!(&mut file, "</table>\n").unwrap();
    }

    write!(&mut file, "</body>\n</html>\n").unwrap();
}
