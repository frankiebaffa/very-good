// vgc: A single-file compiler built on the Very Good Templating Engine.
// Copyright (C) 2024  Frankie Baffa
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! A single-file compiler built on the Very Good Templating Engine

use {
    clap::Parser as ClapParser,
    vg_core::{ FileCache, Parser, Result, },
    std::{
        collections::HashMap,
        path::PathBuf,
    },
};

const LICENSE_NOTICE: &'static str = include_str!("../../NOTICE-GPL");
const LICENSE_FULL: &'static str = include_str!("../../LICENSE-GPL");

/// Very Good Templating Engine Compiler - Compile vg templates.
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Options {
    /// Disable caching.
    #[arg(short, long)]
    no_cache: bool,
    /// Variable implementations to pass through the parser (key:value).
    #[arg(short, long="implementation", value_name="IMPLEMENTATION")]
    implementations: Option<Vec<String>>,
    /// Faux pages to add to the cache (path-to-file:content).
    #[arg(short, long="cached", value_name="CACHED")]
    cached_items: Option<Vec<String>>,
    /// Print the license notice.
    #[arg(short='l')]
    license_notice: bool,
    /// Print the license in full.
    #[arg(short='L')]
    license_full: bool,
    /// The path to the root directory.
    root: PathBuf,
    /// The path to a vg template.
    target: PathBuf,
}

fn main() -> Result<()> {
    let Options {
        root, target, implementations, no_cache, cached_items, license_notice,
        license_full
    } = Options::parse();

    if license_full {
        println!("{LICENSE_FULL}");
        return Ok(());
    } else if license_notice {
        println!("{LICENSE_NOTICE}");
        return Ok(());
    }

    let implementations = match implementations {
        Some(ii) => ii.into_iter()
            .map(|i| {
                let mut kv_split = i.splitn(2, ":");
                let k = kv_split.next().unwrap_or("");
                let v = kv_split.next().unwrap_or("");
                (k.to_owned(), v.to_owned())
            })
            .collect::<HashMap<String, String>>(),
        None => HashMap::new(),
    };

    let output = if !no_cache {
        let mut cache = FileCache::enabled();

        match cached_items {
            Some(ii) => {
                for i in ii.into_iter() {
                    let mut kv_split = i.splitn(2, ":");
                    let k = kv_split.next().unwrap_or("");
                    let p = PathBuf::from(k);
                    let mut b = p.clone();
                    if b.is_file() {
                        b.pop();
                    }

                    let v = kv_split.next().unwrap_or("");
                    let path = FileCache::rebase_path(&root, b, p);
                    cache.insert(path, v.to_owned());
                }
            },
            None => {},
        };

        Parser::compile_implemented_with_cache(&root, &target, implementations, &mut cache)?
    } else {
        Parser::compile_implemented(root, target, implementations)?
    };

    println!("{output}");

    Ok(())
}
