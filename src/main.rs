// Utility to generate JSON files for use by PPSSPP build directory listings

use std::path::{Path, PathBuf};
use std::fs;
use structopt::StructOpt;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Deserialize, Serialize)]
struct File {
    name: String,  // Actual filename
    is_dir: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    children: Vec<File>,
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    root: String,
}

fn recurse(root: &Path, dir: &Path) -> Vec<File> {
    let mut children = vec![];

    for entry in fs::read_dir(dir).unwrap() {
        if let Ok(entry) = entry {
            let name = entry.file_name().to_str().unwrap().to_owned();

            let mut f = File {
                name,
                is_dir: entry.file_type().unwrap().is_dir(),
                children: vec![],
            };

            if f.is_dir {
                f.children = recurse(root, &dir.join(entry.file_name()));
            }

            children.push(f);
        }
    }

    children
}

fn run(opt: &Opt) -> anyhow::Result<()> {
    let root = PathBuf::from(&opt.root).canonicalize()?;

    let mut root_file = File {
        name: root.file_name().unwrap().to_str().unwrap().to_owned(),
        is_dir: true,
        children: vec![]
    };

    root_file.children = recurse(&root, &root);

    let json = serde_json::to_string_pretty(&root_file)?;
    println!("{}", json);
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    run(&opt).unwrap();
}
