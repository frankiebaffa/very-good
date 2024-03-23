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
    args::{ Arguments, OptionType, },
    vg_core::{ FileCache, Parser, Error, Result, },
    std::{
        collections::HashMap,
        io::{ Error as IOError, ErrorKind, },
        path::PathBuf,
    },
};

const HELP: &str = include_str!("../resources/help.txt");
const LICENSE_NOTICE: &str = include_str!("../../NOTICE-GPL");
const LICENSE_FULL: &str = include_str!("../../LICENSE-GPL");

#[derive(Default)]
struct Options {
    no_cache: bool,
    implementations: Vec<String>,
    cached_items: Vec<String>,
    root: Option<PathBuf>,
    target: Option<PathBuf>,
}

fn main() -> Result<()> {
    let mut opts = Options::default();
    Arguments::with_args(&mut opts, |args, opts, arg| {
        match arg.option_type() {
            OptionType::Argument(_) => match arg.qualifier() {
                "h"|"help" => {
                    println!("{HELP}");
                    std::process::exit(0);
                },
                "n"|"no-cache" => opts.no_cache = true,
                "i"|"implementation" => {
                    opts.implementations.push(args.enforce_next_value(&arg)?);
                },
                "c"|"--cached" => {
                    opts.cached_items.push(args.enforce_next_value(&arg)?);
                },
                "l"|"license-notice" => {
                    println!("{LICENSE_NOTICE}");
                    std::process::exit(0);
                },
                "L"|"license-full" => {
                    println!("{LICENSE_FULL}");
                    std::process::exit(0);
                },
                c => {
                    return Err(IOError::new(
                        ErrorKind::Other,
                        format!("{c} is not a valid argument."),
                    ));
                },
            },
            OptionType::Value(_) => if arg.is_n_from_last(1) {
                opts.root = Some(PathBuf::from(arg.qualifier()));
            } else if arg.is_last() {
                opts.target = Some(PathBuf::from(arg.qualifier()));
            },
        }
        Ok(())
    }).map_err(|e| Error::IOError(e))?;

    let Options {
        root, target, implementations, no_cache, cached_items
    } = opts;

    let root = root.map_or(
        Err(Error::IOError(IOError::new(
            ErrorKind::Other,
            "-r|--root must be defined.".to_owned(),
        ))),
        |v| Ok(v)
    )?;

    let target = target.map_or(
        Err(Error::IOError(IOError::new(
            ErrorKind::Other,
            "-r|--root must be defined.".to_owned(),
        ))),
        |v| Ok(v)
    )?;

    let implementations = implementations.into_iter()
        .map(|i| {
            let mut kv_split = i.splitn(2, ':');
            let k = kv_split.next().unwrap_or("");
            let v = kv_split.next().unwrap_or("");
            (k.to_owned(), v.to_owned())
        })
        .collect::<HashMap<String, String>>();

    let output = if !no_cache {
        let mut cache = FileCache::enabled();

        cached_items.into_iter().for_each(|c| {
            let mut kv_split = c.splitn(2, ':');
            let k = kv_split.next().unwrap_or("");
            let p = PathBuf::from(k);
            let mut b = p.clone();
            if b.is_file() {
                b.pop();
            }

            let v = kv_split.next().unwrap_or("");
            let path = FileCache::rebase_path(&root, b, p);
            cache.insert(path, v.to_owned());
        });

        Parser::compile_implemented_with_cache(&root, &target, implementations, &mut cache)?
    } else {
        Parser::compile_implemented(root, target, implementations)?
    };

    println!("{output}");

    Ok(())
}
