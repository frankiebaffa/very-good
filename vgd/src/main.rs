// vgd: A multi-file compiler/copier built on the Very Good Templating Engine.
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

//! A multi-file compiler/copier built on the Very Good Templating Engine.

use {
    args::{ Arguments, OptionType, },
    serde::{ Deserialize, Serialize, },
    std::{
        collections::HashMap,
        env,
        fs::{ OpenOptions, create_dir_all, copy, remove_file, },
        io::{ Error as IOError, ErrorKind, Read, Write, },
        path::PathBuf,
        time::{ Duration, Instant, },
    },
    vg_core::{ Error, FileCache, Parser, Result, },
};

const HELP: &str = include_str!("../resources/help.txt");
const LICENSE_NOTICE: &str = include_str!("../../NOTICE-GPL");
const LICENSE_FULL: &str = include_str!("../../LICENSE-GPL");

#[derive(Default)]
struct Options {
    example_config: bool,
    implementations: Vec<String>,
    cached_items: Vec<String>,
    no_cache: bool,
    read_only: bool,
    timing: bool,
    benchmark: Option<usize>,
    cache_info: bool,
    verbose: bool,
    config: Option<PathBuf>,
}

const fn default_true() -> bool { true }

#[derive(Clone, Deserialize, Serialize)]
struct CompileFileOptions {
    source: PathBuf,
    implementations: Option<HashMap<String, String>>,
    destination: PathBuf,
    #[serde(default = "default_true")]
    delete_if_ignored: bool,
}

#[derive(Clone, Deserialize, Serialize)]
struct CopyFileOptions {
    source: PathBuf,
    destination: PathBuf,
}

#[derive(Clone, Deserialize, Serialize)]
struct CopyDirectoryOptions {
    source: PathBuf,
    destination: PathBuf,
    extension: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
struct CompileFromSourceOptions {
    directory: PathBuf,
    implementations: Option<HashMap<String, String>>,
    extension: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct CompileToDestinationOptions {
    directory: PathBuf,
    extension: String,
    #[serde(default = "default_true")]
    delete_if_ignored: bool,
}

#[derive(Clone, Deserialize, Serialize)]
struct CompileDirectoryOptions {
    source: CompileFromSourceOptions,
    destination: CompileToDestinationOptions,
}

#[derive(Clone, Deserialize, Serialize)]
enum Action {
    CompileFile(CompileFileOptions),
    CompileDirectory(CompileDirectoryOptions),
    CopyFile(CopyFileOptions),
    CopyDirectory(CopyDirectoryOptions),
}

#[derive(Clone, Deserialize, Serialize)]
struct Actions {
    root: PathBuf,
    actions: Vec<Action>
}

fn example_config() -> Actions {
    Actions {
        root: "path/to/root/dir".into(),
        actions: vec![
            Action::CompileFile(CompileFileOptions {
                source: "path/to.source".into(),
                implementations: Some(vec![
                    ("variable".to_owned(), "Value".to_owned(),)
                ].into_iter().collect()),
                destination: "path/to/destination".into(),
                delete_if_ignored: false,
            }),
            Action::CompileDirectory(CompileDirectoryOptions {
                source: CompileFromSourceOptions {
                    directory: "./path/to/source/directory".into(),
                    implementations: Some(vec![
                        ("variable".to_owned(), "Value".to_owned(),)
                    ].into_iter().collect()),
                    extension: "extension_to_compile".into(),
                },
                destination: CompileToDestinationOptions {
                    directory: "./path/to/destination/directory".into(),
                    extension: "extension_to_compile_to".into(),
                    delete_if_ignored: true,
                },
            }),
            Action::CopyFile(CopyFileOptions {
                source: "./path/to/source.file".into(),
                destination: "./path/to/destination.file".into(),
            }),
            Action::CopyDirectory(CopyDirectoryOptions {
                source: "./path/to/source/directory".into(),
                destination: "./path/to/destination/directory".into(),
                extension: Some("an_optional_file_ext".to_owned()),
            }),
        ],
    }
}

fn copy_all_to(src: PathBuf, dst: PathBuf, src_ext: &Option<String>) {
    create_dir_all(&dst).unwrap();

    let mut read_dir = src.read_dir().unwrap();

    while let Some(Ok(entry)) = read_dir.next() {
        let path = entry.path();

        let ext_pass = match path.extension() {
            Some(pe) => match pe.to_str() {
                Some(pe) => match src_ext {
                    Some(se) => se.eq(pe),
                    None => true,
                },
                None => match src_ext {
                    Some(e) => e.is_empty(),
                    None => true,
                },
            },
            None => match src_ext {
                Some(se) => se.is_empty(),
                None => true,
            },
        };

        if path.is_file() && ext_pass {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let mut filepath = dst.clone();
            filepath.push(filename);
            copy(&path, &filepath).unwrap();
        } else if path.is_dir() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let mut filepath = dst.clone();
            filepath.push(filename);
            copy_all_to(path, filepath, src_ext);
        } else {
            continue;
        }
    }
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
                "e"|"example-config" => {
                    let cfg = example_config();
                    println!(
                        "{}",
                        ron::ser::to_string_pretty(
                            &cfg, ron::ser::PrettyConfig::default()
                        ).unwrap()
                    );
                    std::process::exit(0);
                },
                "i"|"implementation" => {
                    opts.implementations.push(args.enforce_next_value(&arg)?);
                },
                "c"|"cached" => {
                    opts.cached_items.push(args.enforce_next_value(&arg)?);
                },
                "n"|"no-cache" => opts.no_cache = true,
                "r"|"read-only" => opts.read_only = true,
                "t"|"timing" => opts.timing = true,
                "b"|"benchmark" => opts.benchmark = Some(arg.qualifier().parse::<usize>().unwrap()),
                "o"|"cache-info" => opts.cache_info = true,
                "v"|"verbose" => opts.verbose = true,
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
                        format!("Invalid argument {}", c),
                    ));
                },
            },
            OptionType::Value(_) => if arg.is_last() {
                opts.config = Some(PathBuf::from(arg.qualifier()));
            } else {
                return Err(IOError::new(
                    ErrorKind::Other,
                    "Value in illegal position.".to_owned(),
                ));
            },
        }
        Ok(())
    }).map_err(|e| Error::IOError(e))?;

    let Options {
        example_config, config, read_only, timing, verbose, implementations,
        benchmark, cache_info, no_cache, cached_items
    } = opts;

    let implementations = implementations
        .into_iter()
        .map(|v| {
            let mut split = v.splitn(2, ':');
            let k = split.next().unwrap();
            let v = split.next().unwrap_or("");
            (k.to_owned(), v.to_owned())
        })
        .collect::<HashMap<String, String>>();

    let cached_items = cached_items.into_iter()
        .map(|i| {
            let mut kv_split = i.splitn(2, ':');
            let k = kv_split.next().unwrap_or("");
            let p = PathBuf::from(k);
            let mut b = p.clone();
            if b.is_file() {
                b.pop();
            }

            let v = kv_split.next().unwrap_or("");
            (b, p, v.to_owned())
        })
        .collect::<Vec<(PathBuf, PathBuf, String)>>();

    macro_rules! vprintln {
        ($($parms:tt)*) => {
            if verbose {
                println!($($parms)*);
            }
        }
    }

    if example_config {
        let cfg = Actions {
            root: "path/to/root/dir".into(),
            actions: vec![
                Action::CompileFile(CompileFileOptions {
                    source: "path/to.source".into(),
                    implementations: Some(vec![
                        ("variable".to_owned(), "Value".to_owned(),)
                    ].into_iter().collect()),
                    destination: "path/to/destination".into(),
                    delete_if_ignored: false,
                }),
                Action::CompileDirectory(CompileDirectoryOptions {
                    source: CompileFromSourceOptions {
                        directory: "./path/to/source/directory".into(),
                        implementations: Some(vec![
                            ("variable".to_owned(), "Value".to_owned(),)
                        ].into_iter().collect()),
                        extension: "extension_to_compile".into(),
                    },
                    destination: CompileToDestinationOptions {
                        directory: "./path/to/destination/directory".into(),
                        extension: "extension_to_compile_to".into(),
                        delete_if_ignored: true,
                    },
                }),
                Action::CopyFile(CopyFileOptions {
                    source: "./path/to/source.file".into(),
                    destination: "./path/to/destination.file".into(),
                }),
                Action::CopyDirectory(CopyDirectoryOptions {
                    source: "./path/to/source/directory".into(),
                    destination: "./path/to/destination/directory".into(),
                    extension: Some("an_optional_file_ext".to_owned()),
                }),
            ],
        };

        println!(
            "{}",
            ron::ser::to_string_pretty(&cfg, ron::ser::PrettyConfig::default()).unwrap()
        );
        return Ok(());
    }

    let pb = config.unwrap_or_else(|| {
        let mut pb = env::current_dir().unwrap();
        pb.push("vg.ron");
        pb
    });

    if !pb.is_file() {
        eprintln!("{pb:?} is not a file.");
        std::process::exit(1);
    }

    let mut config_file = String::new();

    OpenOptions::new()
        .read(true)
        .open(&pb)
        .unwrap()
        .read_to_string(&mut config_file)
        .unwrap();

    let config = ron::from_str::<Actions>(&config_file)
        .unwrap();

    let root = config.root;

    if read_only {
        return Ok(());
    }

    let mut root_dir = pb.clone();
    root_dir.pop();

    let mut benches = Vec::new();

    let mut cache_details = None;

    let n = benchmark.unwrap_or(1);

    for _ in 0..n {
        let actions = config.actions.clone().into_iter();
        let mut dur = Duration::ZERO;
        let mut cache = if no_cache {
            FileCache::disabled()
        } else {
            FileCache::enabled()
        };

        cached_items.clone().into_iter().for_each(|(base, path, content)| {
            let path = FileCache::rebase_path(&root, base, path);
            cache.insert(path, content);
        });

        for action in actions {
            match action {
                Action::CompileFile(opts) => {
                    vprintln!(
                        "Compiling {:?} to {:?}",
                        opts.source,
                        opts.destination,
                    );

                    let mut dst_no_file = opts.destination.clone();
                    dst_no_file.pop();

                    create_dir_all(&dst_no_file).unwrap();

                    let mut global = implementations.clone();
                    let local = opts.implementations.unwrap_or(HashMap::default());
                    local.into_iter().for_each(|i| {
                        global.insert(i.0, i.1);
                    });

                    let start = Instant::now();
                    let source_res = Parser::compile_implemented_with_cache(
                        root.clone(), &opts.source, global, &mut cache
                    );

                    let source = match source_res {
                        Ok(s) => s,
                        Err(e) => match e {
                            Error::IsIgnored => {
                                if opts.delete_if_ignored && !opts.destination.is_file() {
                                    vprintln!(
                                        concat!(
                                            "{:?} is ignored, but destination ",
                                            "does not yet exist for deletion",
                                        ),
                                        opts.source
                                    );

                                    continue;
                                } else if !opts.delete_if_ignored {
                                    vprintln!("{:?} is ignored", &opts.source);
                                    continue;
                                }

                                remove_file(&opts.destination).unwrap();

                                vprintln!(
                                    "{:?} is ignored and {:?} was deleted",
                                    opts.source,
                                    opts.destination,
                                );

                                continue;
                            },
                            e => return Err(e),
                        },
                    };

                    dur += Instant::now() - start;

                    let mut destination = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .create(true)
                        .open(&opts.destination)
                        .unwrap();

                    destination.write_all(source.as_bytes()).unwrap();
                },
                Action::CopyFile(opts) => {
                    vprintln!(
                        "Copying {:?} to {:?}",
                        opts.source,
                        opts.destination,
                    );

                    let mut dst_no_file = opts.destination.clone();
                    dst_no_file.pop();

                    create_dir_all(&dst_no_file).unwrap();

                    copy(opts.source, opts.destination).unwrap();
                },
                Action::CompileDirectory(opts) => {
                    let CompileDirectoryOptions { source, destination, } = opts;

                    vprintln!(
                        "Compiling all files in {:?} with extension {} to {:?} with {:?}.",
                        source.directory,
                        source.extension,
                        destination.directory,
                        destination.extension,
                    );

                    create_dir_all(&destination.directory).unwrap();

                    let mut read_dir = source.directory.read_dir().unwrap();

                    while let Some(Ok(file)) = read_dir.next() {
                        let path = file.path();

                        let chk_ext = match path.extension() {
                            Some(os) => match os.to_str() {
                                Some(ext) => ext,
                                None => continue,
                            },
                            None => continue,
                        };

                        if !path.is_file() || !chk_ext.eq(&source.extension) {
                            continue;
                        }

                        let mut global = implementations.clone();
                        let local = source.implementations.clone()
                            .unwrap_or(HashMap::default());
                        local.into_iter().for_each(|i| {
                            global.insert(i.0, i.1);
                        });

                        let filename = path.file_name()
                            .map(|osstr| osstr.to_str().unwrap())
                            .unwrap();

                        let mut dest: PathBuf = destination.directory.clone();
                        dest.push(filename);
                        dest = dest.with_extension(&destination.extension);

                        let start = Instant::now();
                        let source_res = Parser::compile_implemented_with_cache(
                            root.clone(), &path, global, &mut cache
                        );

                        let source = match source_res {
                            Ok(s) => s,
                            Err(e) => match e {
                                Error::IsIgnored => {
                                    if destination.delete_if_ignored && !dest.is_file() {
                                        vprintln!(
                                            concat!(
                                                "{:?} is ignored, but destination ",
                                                "does not yet exist for deletion",
                                            ),
                                            path
                                        );

                                        continue;
                                    } else if !destination.delete_if_ignored {
                                        vprintln!("{:?} is ignored", &path);
                                        continue;
                                    }

                                    remove_file(&dest).unwrap();

                                    vprintln!(
                                        "{:?} is ignored and {:?} was deleted",
                                        path, dest,
                                    );

                                    continue;
                                },
                                e => return Err(e),
                            },
                        };

                        dur += Instant::now() - start;

                        let mut destination = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open(&dest)
                            .unwrap();

                        destination.write_all(source.as_bytes()).unwrap();
                    }
                },
                Action::CopyDirectory(opts) => {
                    match &opts.extension {
                        Some(ext) => {
                            vprintln!(
                                "Copying all files from {:?} with extension {ext} to {:?}",
                                opts.source, opts.destination,
                            );
                        },
                        None => {
                            vprintln!(
                                "Copying all files from {:?} to {:?}",
                                opts.source, opts.destination,
                            );
                        },
                    }

                    copy_all_to(opts.source, opts.destination, &opts.extension);
                },
            }
        }

        if timing {
            let seconds = dur.as_nanos() as f64 / 1000000000_f64;
            benches.push(seconds);
        }

        if cache_info && cache_details.is_none() {
            cache_details = Some(cache.info());
        }
    }

    let mut did_bench = false;
    if benchmark.is_some() && timing {
        let total_key = "Total";
        let avg_key = "Average";
        let max_key = "Maximum";
        let min_key = "Minimum";

        let total = benches.iter().sum::<f64>();
        let min = benches.clone().into_iter().reduce(f64::min).unwrap();
        let max = benches.clone().into_iter().reduce(f64::max).unwrap();
        let runs = benches.len();
        let width = format!("{runs}").len();

        let padding = [
            width, total_key.len(), avg_key.len(), min_key.len(), max_key.len()
        ].into_iter().max().unwrap();

        for (idx, bench) in benches.into_iter().enumerate() {
            let run = idx + 1;
            println!("{run:<padding$}: {bench}s");
        }

        println!("{total_key:<padding$}: {total}s");
        let avg = total / runs as f64;
        println!("{avg_key:<padding$}: {avg}s");
        println!("{min_key:<padding$}: {min}s");
        println!("{max_key:<padding$}: {max}s");
        did_bench = true;
    } else if timing {
        let total = benches.iter().sum::<f64>();
        println!("{total}s");
    }

    if cache_info && cache_details.is_some() && !no_cache {
        let pre = if did_bench { "\n" } else { "" };

        let details = cache_details.unwrap();
        let padding = details.iter()
            .map(|d| format!("{d:?}"))
            .map(|d| d.len())
            .max()
            .unwrap_or(0) + 1;

        for detail in details.into_iter() {
            let path = format!("{:?}", detail.0);
            let hits = detail.1;

            println!("{pre}{path:<padding$}: {hits}");
        }
    }

    Ok(())
}
