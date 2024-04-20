// vg-core: The core technologies behind the Very Good Templating Engine.
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

//! The core technologies behind the Very Good Templating Engine.

#[cfg(test)]
mod test;

use {
    std::{
        collections::HashMap,
        error::Error as StdError,
        fmt::{
            Display,
            Formatter,
            Result as FmtResult,
        },
        fs::OpenOptions,
        io::{
            Error as IOError,
            BufReader,
            BufRead,
        },
        path::{
            Path,
            PathBuf,
        },
        time::SystemTime,
    },
    nfm_core::Parser as NfmParser,
};

const TAG: [&str; 2] = [
    "{%",
    "%}"
];

const VARIABLE: [&str; 2] = [
    "{{",
    "}}"
];

const PATH: &str = "\"";

const COMMENT: [&str; 2] = [
    "{#",
    "#}"
];

const CURLY_ESCAPE: [&str; 2] = [
    "\\{",
    "\\}",
];

const PERC_ESCAPE: &str = "\\%";

const HASH_ESCAPE: &str = "\\#";

const KEYWORDS: [&str; 10] = [
    "else",
    "endfor",
    "endif",
    "endblock",
    "extends",
    "for",
    "if",
    "include",
    "block",
    "ignore",
];

fn starts_with_keyword(s: &str) -> Option<String> {
    for keyword in KEYWORDS {
        if s.starts_with(keyword) {
            return Some(keyword.to_owned());
        }
    }

    None
}

const PIPE: &str = "|";

const FILTERS: [&str; 9] = [
    "flatten",
    "trimend",
    "trimstart",
    "trim",
    "detab",
    "replace",
    "lower",
    "upper",
    "md",
];

enum Filter {
    Flatten,
    Trim,
    Detab,
    Replace(String, String),
    Lower,
    Upper,
    Markdown,
    TrimEnd,
    TrimStart
}

fn starts_with_filter(s: &str) -> Option<String> {
    for filter in FILTERS {
        if s.starts_with(filter) {
            return Some(filter.to_owned());
        }
    }

    None
}

#[derive(Debug)]
struct CachedFile {
    hits: usize,
    content: String,
}

impl CachedFile {
    fn new(content: String) -> Self {
        Self { hits: 0, content, }
    }
}

/// The caching mechanism for the parser.
#[derive(Debug)]
pub struct FileCache {
    enabled: bool,
    files: Option<HashMap<PathBuf, CachedFile>>,
}

impl FileCache {
    /// Construct an enabled caching mechanism. All paths will be read once and
    /// retrieved from cache on subsequent hits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vg_core::FileCache;
    ///
    /// let cache = FileCache::enabled();
    /// ```
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            files: None,
        }
    }

    /// Construct a disabled caching mechanism. All paths will be read from file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vg_core::FileCache;
    ///
    /// let cache = FileCache::disabled();
    /// ```
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            files: None,
        }
    }

    /// Retrieve paths and hit counts for all items in cache. Always empty when
    /// cache is disabled.
    ///
    /// # Example 1: Enabled
    ///
    /// ```rust
    /// use vg_core::FileCache;
    ///
    /// let mut cache = FileCache::enabled();
    /// cache.insert("/template.jinja", "Here is some content".to_owned());
    /// let info = cache.info();
    /// assert_eq!(0, info.get(0).unwrap().1);
    /// ```
    ///
    /// # Example 2: Disabled
    ///
    /// ```rust
    /// use vg_core::FileCache;
    ///
    /// let mut cache = FileCache::disabled();
    /// cache.insert("/template.jinja", "Here is some content".to_owned());
    /// let info = cache.info();
    /// assert_eq!(0, info.len());
    /// ```
    pub fn info(&self) -> Vec<(PathBuf, usize)> {
        if !self.enabled {
            Vec::new()
        } else {
            match &self.files {
                Some(files) => {
                    files.iter()
                        .map(|f| (f.0.to_owned(), f.1.hits))
                        .collect::<Vec<(PathBuf, usize)>>()
                },
                None => {
                    Vec::new()
                },
            }
        }
    }

    /// Insert a new path and file-content into the cache.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vg_core::FileCache;
    ///
    /// let mut cache = FileCache::enabled();
    /// cache.insert("/template.jinja", "Here is some content".to_owned());
    /// ```
    pub fn insert<P: AsRef<Path>>(&mut self, p: P, content: String) {
        if self.enabled {
            match &mut self.files {
                Some(files) => {
                    files.insert(p.as_ref().into(), CachedFile::new(content));
                },
                None => {
                    let mut files = HashMap::new();
                    files.insert(p.as_ref().into(), CachedFile::new(content));
                    self.files = Some(files);
                },
            }
        }
    }

    fn read_file<P: AsRef<Path>>(p: P) -> Result<String> {
        let path = p.as_ref();

        if !path.is_file() {
            return Err(Error::NotAFileError(path.into()));
        }

        let mut file = OpenOptions::new().read(true).open(path)
            .map_err(Error::IOError)?;
        let mut source = String::new();
        let br = BufReader::new(&mut file);
        let mut lines = br.lines();
        let mut dlim = "";
        while let Some(Ok(line)) = lines.next() {
            source.push_str(dlim);
            source.push_str(&line);

            if dlim.is_empty() {
                dlim = "\n";
            }
        }

        Ok(source)
    }

    fn get<P: AsRef<Path>>(&mut self, p: P) -> Result<String> {
        if !self.enabled {
            return Self::read_file(p);
        }

        let path: PathBuf = p.as_ref().into();

        match &mut self.files {
            Some(files) => {
                match files.get_mut(&path) {
                    Some(f) => {
                        f.hits += 1;
                        Ok(f.content.clone())
                    },
                    None => {
                        let source = Self::read_file(&path)?;
                        files.insert(path, CachedFile::new(source.clone()));
                        Ok(source)
                    },
                }
            },
            None => {
                let mut files = HashMap::new();
                let source = Self::read_file(&path)?;
                files.insert(path, CachedFile::new(source.clone()));
                self.files = Some(files);
                Ok(source)
            },
        }
    }

    /// Get a reconciled path based on the root-path of the program, the
    /// current base-path of the parser, and the path of the file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     vg_core::FileCache,
    ///     std::path::PathBuf,
    /// };
    ///
    /// let root = PathBuf::from("./in");
    /// let base = PathBuf::from("/");
    /// let path = PathBuf::from("/template.jinja");
    ///
    /// let real = FileCache::rebase_path(root, base, path);
    /// assert_eq!(PathBuf::from("./in/template.jinja"), real);
    /// ```
    pub fn rebase_path<R, B, P>(root: R, base: B, path: P) -> PathBuf
    where
        R: AsRef<Path>,
        B: AsRef<Path>,
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        if path.is_absolute() {
            let mut root: PathBuf = root.as_ref().into();
            let path: PathBuf = path.into();
            let mut p_iter = path.iter();

            p_iter.next(); // consume root directory

            p_iter.for_each(|seg| {
                root.push(seg);
            });

            root
        } else {
            let mut base: PathBuf = base.as_ref().into();
            base.push(path);

            base
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Condition {
    Existence,
    Emptiness,
}

fn starts_with_valid_var_name_char(s: &str) -> bool {
    let c = &s[0..1];

    matches!(
        c,
        "a"|"b"|"c"|"d"|"e"|"f"|"g"|"h"|"i"|"j"|"k"|"l"|"m"|"n"|"o"|"p"|"q"|"r"|
        "s"|"t"|"u"|"v"|"w"|"x"|"y"|"z"|
        "A"|"B"|"C"|"D"|"E"|"F"|"G"|"H"|"I"|"J"|"K"|"L"|"M"|"N"|"O"|"P"|"Q"|"R"|
        "S"|"T"|"U"|"V"|"W"|"X"|"Y"|"Z"|
        "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|
        "_"|"-"|"."
    )
}

#[derive(Debug)]
struct Context {
    implementations: HashMap<String, String>,
    prefix: Option<String>,
    directory: PathBuf,
    nested_within_keyword: String,
    holding: String,
    output: String,
    was_extends: bool,
    is_first: bool,
    extends: Option<PathBuf>,
    trim_start: bool,
    trim_end: bool,
}

impl Context {
    fn new(dir: PathBuf) -> Self {
        Self {
            implementations: HashMap::new(),
            prefix: None,
            directory: dir,
            nested_within_keyword: String::new(),
            holding: String::new(),
            output: String::new(),
            was_extends: false,
            is_first: true,
            extends: None,
            trim_start: false,
            trim_end: false,
        }
    }

    fn shallow_clone(&self) -> Self {
        Self {
            implementations: self.implementations.clone(),
            prefix: self.prefix.clone(),
            directory: self.directory.clone(),
            nested_within_keyword: self.nested_within_keyword.clone(),
            holding: String::new(),
            output: String::new(),
            was_extends: false,
            is_first: true,
            extends: None,
            trim_start: false,
            trim_end: false,
        }
    }

    fn with_keyword(&self, keyword: &str) -> Self {
        let mut cl = self.shallow_clone();
        cl.nested_within_keyword = keyword.to_owned();
        cl
    }

    fn set_keyword(&mut self, keyword: &str) {
        self.nested_within_keyword = keyword.to_owned();
    }

    fn clear_holding(&mut self) {
        self.holding.clear();
    }

    fn flush_holding(&mut self) {
        self.output.push_str(&self.holding);
        self.holding.clear();
    }

    fn flip_first(&mut self) {
        if self.is_first {
            self.is_first = false;
        }
    }

    fn push_holding(&mut self, s: &str) {
        self.holding.push_str(s);
    }

    fn push_output(&mut self, s: &str) {
        self.output.push_str(s);
    }
}

fn handle_trim(s: &mut String, trim_start: bool, trim_end: bool) {
    if trim_start && trim_end {
        let mut out = s.trim().to_owned();
        std::mem::swap(&mut out, s);
    } else if trim_start {
        let mut out = s.trim_start().to_owned();
        std::mem::swap(&mut out, s);
    } else if trim_end {
        let mut out = s.trim_end().to_owned();
        std::mem::swap(&mut out, s);
    }
}

struct ForItem {
    path: PathBuf,
    name: String,
    created: SystemTime,
    modified: SystemTime,
}

const SORTS: [&str; 3] = [
    "name",
    "created",
    "modified"
];

fn starts_with_sort(s: &str) -> Option<String> {
    for method in SORTS {
        if s.starts_with(method) {
            return Some(method.to_owned());
        }
    }

    None
}

/// A vg error.
#[derive(Debug)]
pub enum Error {
    NotADirectoryError(PathBuf),
    NotAFileError(PathBuf),
    IOError(IOError),
    IsIgnored,
}

impl Display for Error {
    fn fmt(&self, fmtr: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NotADirectoryError(d) => {
                fmtr.write_fmt(format_args!("{:?} is not a directory", d))
            },
            Self::NotAFileError(f) => {
                fmtr.write_fmt(format_args!("{:?} is not a file", f))
            },
            Self::IsIgnored => {
                fmtr.write_str("File is ignored")
            },
            Self::IOError(e) => e.fmt(fmtr),
        }
    }
}

impl StdError for Error {}

/// A vg result.
pub type Result<T> = std::result::Result<T, Error>;

/// A parser for vg templates.
///
/// # Examples
///
/// ```rust
/// use vg_core::Parser;
///
/// let output = Parser::compile("./test/full/3", "./test/full/3/pages/page.jinja").unwrap();
/// let against = include_str!("../test/full/3/against.jinja");
/// assert_eq!(against[0..against.len()-1], output);
/// ```
#[derive(Debug)]
pub struct Parser {
    position: usize,
    src: String,
    root_dir: PathBuf,
    base_dir: PathBuf,
}

impl Parser {
    fn from_content(source: String, root_dir: PathBuf, base_dir: PathBuf) -> Self {
        Self { position: 0, src: source, root_dir, base_dir, }
    }

    fn from_file<R: AsRef<Path>, P: AsRef<Path>>(r: R, p: P, cache: &mut FileCache) -> Result<Self> {
        let root = r.as_ref();

        if !root.is_dir() {
            return Err(Error::NotADirectoryError(root.into()));
        }

        let path = p.as_ref();

        let source = cache.get(path)?;

        let mut base_dir: PathBuf = path.into();
        base_dir.pop();

        Ok(Self::from_content(source, root.into(), base_dir))
    }

    fn len(&self) -> usize {
        self.src.len()
    }

    fn shallow_clone(&self, from: usize, to: usize) -> Self {
        let src = &self.src[from..to];
        Self {
            position: 0,
            src: src.to_owned(),
            root_dir: self.root_dir.clone(),
            base_dir: self.base_dir.clone(),
        }
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn source(&self) -> &str {
        &self.src[self.position..]
    }

    fn copy_into(&self, n: usize, into: &mut String) {
        into.push_str(&self.src[self.position..self.position+n]);
    }

    fn advance_into(&mut self, n: usize, into: &mut String) {
        self.copy_into(n, into);
        self.advance(n);
    }

    fn starts_with(&self, s: &str) -> bool {
        self.source().starts_with(s)
    }

    fn is_empty(&self) -> bool {
        self.source().is_empty()
    }

    fn starts_with_valid_var_name_char(&self) -> bool {
        starts_with_valid_var_name_char(self.source())
    }

    fn starts_with_sort(&self) -> Option<String> {
        starts_with_sort(self.source())
    }

    fn trim_start_into(&mut self, into: &mut String) {
        while self.starts_with(" ") || self.starts_with("\t") {
            self.advance_into(1, into);
        }
    }

    fn end_tag(&mut self, keyword: &str, context: &mut Context) -> bool {
        self.trim_start_into(&mut context.holding);

        match keyword {
            // only else can have trim start and end
            "else" => {
                context.trim_start = self.starts_with("-");
                if context.trim_start {
                    self.advance_into(1, &mut context.holding);
                }
            },
            // only endblock can be followed by a name
            "endblock" => {
                while self.starts_with_valid_var_name_char() {
                    self.advance_into(1, &mut context.holding);
                    self.trim_start_into(&mut context.holding);
                }
            },
            _ => {},
        }

        if !self.starts_with(TAG[1]) {
            return false;
        }

        self.advance_into(TAG[1].len(), &mut context.holding);

        context.set_keyword(keyword);

        true
    }

    fn variable(&mut self, context: &mut Context) -> bool {
        context.flush_holding();
        self.advance_into(VARIABLE[0].len(), &mut context.holding);
        self.trim_start_into(&mut context.holding);

        let mut name = String::new();

        while self.starts_with_valid_var_name_char() || self.starts_with(".") {
            self.copy_into(1, &mut name);
            self.advance_into(1, &mut context.holding);
        }

        let start_dot = name.starts_with('.');
        let end_dot = name.ends_with('.');

        if start_dot || end_dot {
            return false;
        }

        self.trim_start_into(&mut context.holding);

        // check for nullability
        let nullable = if self.starts_with("?") {
            self.advance_into(1, &mut context.holding);
            self.trim_start_into(&mut context.holding);
            true
        } else {
            false
        };

        let mut filters = Vec::new();
        let mut do_replace = false;

        while self.starts_with(PIPE) {
            self.advance_into(1, &mut context.holding);
            self.trim_start_into(&mut context.holding);

            if let Some(filter) = starts_with_filter(self.source()) {
                let filter = filter.as_str();

                match filter {
                    "flatten" => filters.push(Filter::Flatten),
                    "detab" => filters.push(Filter::Detab),
                    "trim" => filters.push(Filter::Trim),
                    "upper" => filters.push(Filter::Upper),
                    "lower" => filters.push(Filter::Lower),
                    "replace" => {
                        do_replace = true;
                    },
                    "md" => filters.push(Filter::Markdown),
                    "trimend" => filters.push(Filter::TrimEnd),
                    "trimstart" => filters.push(Filter::TrimStart),
                    _ => return false,
                }

                self.advance_into(filter.len(), &mut context.holding);
                self.trim_start_into(&mut context.holding);

                if do_replace {
                    if !self.starts_with(PATH) {
                        return false;
                    }

                    self.advance_into(PATH.len(), &mut context.holding);

                    let mut replace_this = String::new();

                    while !self.is_empty() && !self.starts_with(PATH) {
                        self.copy_into(1, &mut replace_this);
                        self.advance_into(1, &mut context.holding);
                    }

                    if !self.starts_with(PATH) || replace_this.is_empty() {
                        return false;
                    }

                    self.advance_into(PATH.len(), &mut context.holding);

                    self.trim_start_into(&mut context.holding);

                    if !self.starts_with(PATH) {
                        return false;
                    }

                    self.advance_into(PATH.len(), &mut context.holding);

                    let mut with = String::new();

                    while !self.is_empty() && !self.starts_with(PATH) {
                        self.copy_into(1, &mut with);
                        self.advance_into(1, &mut context.holding);
                    }

                    if !self.starts_with(PATH) {
                        return false;
                    }

                    self.advance_into(1, &mut context.holding);

                    self.trim_start_into(&mut context.holding);

                    filters.push(Filter::Replace(replace_this, with));
                    do_replace = false;
                }
            } else {
                return false;
            }
        }

        if !self.starts_with(VARIABLE[1]) {
            return false;
        }

        self.advance_into(VARIABLE[1].len(), &mut context.holding);

        name = match &context.prefix {
            Some(prefix) => format!("{prefix}.{name}"),
            None => name,
        };

        match context.implementations.get(&name).map(|v| v.to_owned()) {
            Some(mut i) => {
                filters.into_iter().for_each(|f| {
                    match f {
                        Filter::Flatten => i = i.replace('\n', " "),
                        Filter::Trim => i = i.trim().to_owned(),
                        Filter::Detab => i = i.replace('\t', ""),
                        Filter::Upper => i = i.to_uppercase(),
                        Filter::Lower => i = i.to_lowercase(),
                        Filter::Replace(this, with) => i = i.replace(&this, &with),
                        Filter::Markdown => i = NfmParser::parse_str(&i),
                        Filter::TrimEnd => i = i.trim_end().to_owned(),
                        Filter::TrimStart => i = i.trim_start().to_owned(),
                    }
                });

                context.push_output(&i);
                context.clear_holding();
            },
            None => if !nullable {
                context.flush_holding();
            } else {
                context.clear_holding();
            },
        }

        context.flip_first();

        true
    }

    fn ignore(&mut self, context: &mut Context) -> Result<bool> {
        if !context.is_first || context.trim_end {
            return Ok(false);
        }

        if !self.starts_with(TAG[1]) {
            return Ok(false);
        }

        self.advance_into(TAG[1].len(), &mut context.holding);

        context.clear_holding();
        context.flip_first();

        Err(Error::IsIgnored)
    }

    fn extends(&mut self, context: &mut Context) -> bool {
        // this keyword accepts a path value
        if !context.is_first || context.trim_end || context.extends.is_some() ||
            !self.starts_with(PATH)
        {
            return false;
        }

        self.advance_into(PATH.len(), &mut context.holding);

        let mut path = String::new();

        if self.starts_with(VARIABLE[0]) {
            let mut var_ctx = context.shallow_clone();

            if !self.variable(&mut var_ctx) {
                return false;
            }

            path = var_ctx.output;
        } else {
            while !self.starts_with(PATH) && !self.source().is_empty() {
                self.copy_into(1, &mut path);
                self.advance_into(1, &mut context.holding);
            }
        }

        if path.is_empty() || !self.starts_with(PATH) {
            return false;
        }

        self.advance_into(PATH.len(), &mut context.holding);

        self.trim_start_into(&mut context.holding);

        if !self.starts_with(TAG[1]) {
            return false;
        }

        self.advance_into(TAG[1].len(), &mut context.holding);

        let rebased = FileCache::rebase_path(
            &self.root_dir,
            &context.directory,
            &path
        );
        context.extends = Some(rebased);
        context.clear_holding();
        context.was_extends = true;
        context.flip_first();

        true
    }

    fn include(&mut self, context: &mut Context, cache: &mut FileCache) -> Result<bool> {
        let mut is_raw = false;
        let mut is_md = false;
        // can be included raw
        if self.starts_with("raw") {
            self.advance_into(3, &mut context.holding);
            self.trim_start_into(&mut context.holding);
            is_raw = true;
            if self.starts_with("md") {
                self.advance_into(2, &mut context.holding);
                self.trim_start_into(&mut context.holding);
                is_md = true;
            }
        } else if self.starts_with("md") {
            self.advance_into(2, &mut context.holding);
            self.trim_start_into(&mut context.holding);
            is_md = true;
        }

        // this keyword accepts a path value
        if !self.starts_with(PATH) {
            return Ok(false);
        }

        self.advance_into(PATH.len(), &mut context.holding);

        let mut path = String::new();

        if self.starts_with(VARIABLE[0]) {
            let mut var_ctx = context.shallow_clone();

            if !self.variable(&mut var_ctx) {
                return Ok(false);
            }

            path = var_ctx.output;
        } else {
            while !self.starts_with(PATH) && !self.source().is_empty() {
                self.copy_into(1, &mut path);
                self.advance_into(1, &mut context.holding);
            }
        }

        if path.is_empty() || !self.starts_with(PATH) {
            return Ok(false);
        }

        self.advance_into(PATH.len(), &mut context.holding);

        self.trim_start_into(&mut context.holding);

        // handle as
        const AS: &str = "as";

        let mut as_name = String::new();

        if self.starts_with(AS) && !is_raw {
            self.advance_into(AS.len(), &mut context.holding);

            self.trim_start_into(&mut context.holding);

            while self.starts_with_valid_var_name_char() && !self.source().is_empty() {
                self.copy_into(1, &mut as_name);
                self.advance_into(1, &mut context.holding);
            }
        }
        // "as" keyword not allowed in conjunction with raw
        else if self.starts_with(AS) && is_raw {
            return Ok(false);
        }

        self.trim_start_into(&mut context.holding);

        context.trim_start = self.starts_with("-");

        if context.trim_start {
            self.advance_into(1, &mut context.holding);
        }

        if !self.starts_with(TAG[1]) {
            return Ok(false);
        }

        self.advance_into(TAG[1].len(), &mut context.holding);

        let rebased = FileCache::rebase_path(&self.root_dir, &self.base_dir, &path);

        // raw included content is directly injected into output
        if is_raw {
            match cache.get(&rebased) {
                Ok(c) => {
                    if is_md {
                        context.push_output(&NfmParser::parse_str(&c));
                    } else {
                        context.push_output(&c);
                    }
                },
                Err(e) => match e {
                    Error::IsIgnored => {},
                    e => return Err(e),
                },
            };

            // include raw was valid
            context.clear_holding();
            context.flip_first();
            return Ok(true);
        }

        // if as name is empty
        let (mut this_prefix, had_as) = if as_name.is_empty() {
            // then use the existing prefix
            (context.prefix.as_ref().map(|p| p.to_owned()), false)
        } else {
            // if as name is not empty, combine with existing prefixing
            (context.prefix.as_ref().map(|p| format!("{p}.{as_name}")).or(Some(as_name)), true)
        };

        // set prefix for includes
        std::mem::swap(&mut context.prefix, &mut this_prefix);

        // include gets tokenized here and the raw tokens are included in the
        // output
        let mut include_parser = Self::from_file(&self.root_dir, &rebased, cache)?;

        // give context the same base directory as the new parser
        let mut old_directory = include_parser.base_dir.clone();
        std::mem::swap(&mut old_directory, &mut context.directory);

        // we don't want the parsing of includes to finish by extending this
        // templates extends block.
        let mut tmp_extends = None;
        std::mem::swap(&mut tmp_extends, &mut context.extends);

        // include was valid, clear holding/flip first now
        context.clear_holding();
        context.flip_first();

        // we don't want that value to be taken into account in this parsing tho
        let mut tmp_is_first = true;
        std::mem::swap(&mut tmp_is_first, &mut context.is_first);

        // hold onto the output for now
        let mut tmp_output = String::new();
        std::mem::swap(&mut context.output, &mut tmp_output);

        match include_parser.parse(context, cache) {
            Ok(_) => {},
            Err(e) => match e {
                Error::IsIgnored => {},
                e => return Err(e),
            },
        }

        // revert directory of context
        std::mem::swap(&mut old_directory, &mut context.directory);

        // revert prefix for includes
        std::mem::swap(&mut context.prefix, &mut this_prefix);

        // revert extends
        std::mem::swap(&mut tmp_extends, &mut context.extends);

        // revert is_first
        std::mem::swap(&mut tmp_is_first, &mut context.is_first);

        // put output back
        std::mem::swap(&mut context.output, &mut tmp_output);

        // if was markdown, parse output
        if is_md {
            tmp_output = NfmParser::parse_str(&tmp_output);
        }

        // if had 'as' keyword, put new output into an implementation
        if had_as {
            context.implementations.insert(this_prefix.unwrap(), tmp_output);
        }
        // otherwise, append new output onto the end of the original output
        else {
            context.push_output(&tmp_output);
        }

        Ok(true)
    }

    fn for_tag(&mut self, context: &mut Context, cache: &mut FileCache) -> Result<bool> {
        // first value is the variable name
        let mut variable = String::new();

        while self.starts_with_valid_var_name_char() {
            self.copy_into(1, &mut variable);
            self.advance_into(1, &mut context.holding);
        }

        if variable.is_empty() || context.trim_end {
            return Ok(false);
        }

        variable = match &context.prefix {
            Some(prefix) => format!("{prefix}.{variable}"),
            None => variable,
        };

        self.trim_start_into(&mut context.holding);

        const IN: &str = "in";

        // next value should be "in"
        if !self.starts_with(IN) {
            return Ok(false);
        }

        self.advance_into(IN.len(), &mut context.holding);

        self.trim_start_into(&mut context.holding);

        // next value is the path to the collection
        if !self.starts_with(PATH) {
            return Ok(false);
        }

        self.advance_into(PATH.len(), &mut context.holding);

        let mut path = String::new();

        if self.starts_with(VARIABLE[0]) {
            let mut var_ctx = context.shallow_clone();

            if !self.variable(&mut var_ctx) {
                return Ok(false);
            }

            path = var_ctx.output;
        } else {
            while !self.starts_with(PATH) && !self.source().is_empty() {
                self.copy_into(1, &mut path);
                self.advance_into(1, &mut context.holding);
            }
        }

        if path.is_empty() || !self.starts_with(PATH) {
            return Ok(false);
        }

        self.advance_into(PATH.len(), &mut context.holding);
        self.trim_start_into(&mut context.holding);

        let (sort, reverse) = if self.starts_with(PIPE) {
            self.advance_into(PIPE.len(), &mut context.holding);
            self.trim_start_into(&mut context.holding);

            let reverse = self.starts_with("!");
            if reverse {
                self.advance_into(1, &mut context.holding);
            }

            if let Some(s) = self.starts_with_sort() {
                self.advance_into(s.len(), &mut context.holding);
                self.trim_start_into(&mut context.holding);
                (s, reverse)
            } else {
                return Ok(false);
            }
        } else {
            (SORTS[0].to_owned(), false)
        };

        context.trim_start = self.starts_with("-");

        if context.trim_start {
            self.advance_into(1, &mut context.holding);
        }

        if !self.starts_with(TAG[1]) {
            return Ok(false);
        }

        self.advance_into(TAG[1].len(), &mut context.holding);

        let rebased = FileCache::rebase_path(
            &self.root_dir,
            &self.base_dir,
            &path,
        );

        let mut is_loop = false;

        let items = if rebased.is_dir() {
            match rebased.read_dir() {
                Ok(mut read_dir) => {
                    let mut items = Vec::new();

                    while let Some(Ok(entry)) = read_dir.next() {
                        let path = entry.path();

                        if !path.is_file() {
                            continue;
                        }

                        let name = path.file_name().unwrap().to_str().unwrap().to_owned();

                        let metadata = path.metadata().map_err(Error::IOError)?;
                        let created = metadata.created().map_err(Error::IOError)?;
                        let modified = metadata.modified().map_err(Error::IOError)?;

                        items.push(ForItem {
                            path,
                            name,
                            created,
                            modified,
                        });

                        // mark as loop to set loop context implementations
                        if !is_loop {
                            is_loop = true;
                        }
                    }

                    match sort.as_str() {
                        "name" => items.sort_unstable_by(|a, b| a.name.cmp(&b.name)),
                        "created" => items.sort_unstable_by(|a, b| a.created.cmp(&b.created)),
                        "modified" => items.sort_unstable_by(|a, b| a.modified.cmp(&b.modified)),
                        _ => panic!("HOW!?"),
                    }

                    if reverse {
                        items.reverse();
                    }

                    items.into_iter()
                        .map(|i| i.path)
                        .collect::<Vec<PathBuf>>()
                },
                Err(_) => Vec::new(),
            }
        } else if rebased.is_file() {
            vec![rebased]
        } else {
            Vec::new()
        };

        if !items.is_empty() {
            // perform a dummy run through the file to check validity.
            // we only need the source from the current position forward
            let start_position = self.position;
            let mut dummy_parser = self.shallow_clone(self.position, self.len());
            let mut dummy_ctx = context.with_keyword("for");
            dummy_parser.parse(&mut dummy_ctx, cache)?;

            std::mem::take(&mut dummy_ctx.output);

            let (is_valid, end_idx) = match dummy_ctx.nested_within_keyword.as_str() {
                "else" => {
                    let mut else_ctx = dummy_ctx.with_keyword("for");
                    dummy_parser.parse(&mut else_ctx, cache)?;

                    match else_ctx.nested_within_keyword.as_str() {
                        "endfor" => (true, dummy_parser.position + start_position),
                        _ => (false, 0),
                    }
                },
                "endfor" => (true, dummy_parser.position + start_position),
                _ => (false, 0),
            };

            if !is_valid {
                return Ok(false);
            }

            let size = items.len();
            let max = items.len() - 1;

            let mut aug_idx = 0;

            for (idx, i) in items.into_iter().enumerate() {
                let idx = idx - aug_idx;
                let size = size - aug_idx;
                let max = max - aug_idx;

                // parse item from file
                let mut item_parser = Self::from_file(&self.root_dir, i, cache)?;
                let mut item_ctx = context.shallow_clone();

                let mut old_prefix = Some(variable.clone());
                std::mem::swap(&mut old_prefix, &mut item_ctx.prefix);

                let mut old_dir = item_parser.base_dir.clone();
                std::mem::swap(&mut old_dir, &mut item_ctx.directory);

                match item_parser.parse(&mut item_ctx, cache) {
                    Ok(_) => {},
                    Err(e) => match e {
                        Error::IsIgnored => {
                            aug_idx += 1;
                            continue;
                        },
                        e => return Err(e),
                    },
                }

                // item_ctx will now have all content and implementations
                // from item.

                let mut item_content = String::new();
                std::mem::swap(&mut item_content, &mut item_ctx.output);

                // revert directory and prefix
                std::mem::swap(&mut old_dir, &mut item_ctx.directory);
                std::mem::swap(&mut old_prefix, &mut item_ctx.prefix);

                item_ctx.implementations
                    .insert(variable.clone(), item_content);

                // handle loop context
                if is_loop {
                    let loop_prefix = match &context.prefix {
                        Some(p) => format!("{p}.loop"),
                        None => "loop".to_owned(),
                    };

                    item_ctx.implementations
                        .insert(loop_prefix.clone(), variable.clone());
                    item_ctx.implementations
                        .insert(format!("{loop_prefix}.index"), format!("{idx}"));
                    item_ctx.implementations
                        .insert(format!("{loop_prefix}.position"), format!("{}", idx + 1));
                    if idx == 0 {
                        item_ctx.implementations
                            .insert(format!("{loop_prefix}.first"), format!("{}", idx == 0));
                    } else {
                        item_ctx.implementations.remove(&format!("{loop_prefix}.first"));
                    }
                    if idx == max {
                        item_ctx.implementations
                            .insert(format!("{loop_prefix}.last"), format!("{}", idx == max));
                    } else {
                        item_ctx.implementations.remove(&format!("{loop_prefix}.last"));
                    }
                    item_ctx.implementations
                        .insert(format!("{loop_prefix}.size"), format!("{}", size));
                    item_ctx.implementations
                        .insert(format!("{loop_prefix}.max"), format!("{}", max));
                }

                let mut for_ctx = item_ctx.with_keyword("for");

                let mut parser_cl = self.shallow_clone(self.position, end_idx);
                parser_cl.parse(&mut for_ctx, cache)?;

                let mut for_content = std::mem::take(&mut for_ctx.output);

                match for_ctx.nested_within_keyword.as_str() {
                    "else" => {
                        let mut else_ctx = for_ctx.with_keyword("for");
                        parser_cl.parse(&mut else_ctx, cache)?;

                        match else_ctx.nested_within_keyword.as_str() {
                            "endfor" => {
                                handle_trim(
                                    &mut for_content,
                                    context.trim_start,
                                    for_ctx.trim_end
                                );

                                context.push_output(&for_content);

                                context.clear_holding();
                                context.flip_first();

                                if idx == max {
                                    std::mem::swap(&mut parser_cl.src, &mut self.src);
                                    std::mem::swap(&mut parser_cl, self);
                                    self.position = end_idx;
                                }
                            },
                            _ => panic!("HOW!?"),
                        }
                    },
                    "endfor" => {
                        handle_trim(
                            &mut for_content,
                            context.trim_start,
                            for_ctx.trim_end
                        );

                        context.push_output(&for_content);

                        context.clear_holding();
                        context.flip_first();

                        if idx == max {
                            std::mem::swap(&mut parser_cl.src, &mut self.src);
                            std::mem::swap(&mut parser_cl, self);
                            self.position = end_idx;
                        }
                    },
                    _ => panic!("HOW!?"),
                }
            }

            Ok(true)
        } else {
            let mut for_ctx = context.with_keyword("for");

            self.parse(&mut for_ctx, cache)?;

            match for_ctx.nested_within_keyword.as_str() {
                "else" => {
                    let mut else_ctx = context.with_keyword("for");
                    self.parse(&mut else_ctx, cache)?;
                    let mut else_content = else_ctx.output;

                    match else_ctx.nested_within_keyword.as_str() {
                        "endfor" => {
                            handle_trim(&mut else_content, for_ctx.trim_start, else_ctx.trim_end);
                            context.push_output(&else_content);

                            context.clear_holding();
                            context.flip_first();

                            Ok(true)
                        },
                        _ => Ok(false),
                    }
                },
                "endfor" => {
                    context.clear_holding();
                    context.flip_first();

                    Ok(true)
                },
                _ => Ok(false),
            }
        }
    }

    fn if_tag(&mut self, context: &mut Context, cache: &mut FileCache) -> Result<bool> {
        // first value is the variable name
        let mut variable = String::new();

        // get variable name
        while self.starts_with_valid_var_name_char() || self.starts_with(".") ||
            self.starts_with("!")
        {
            self.copy_into(1, &mut variable);
            self.advance_into(1, &mut context.holding);
        }

        self.trim_start_into(&mut context.holding);

        const EMPTY: &str = "empty";
        const NOT: &str = "not";

        // check for non-default condition
        let neg_cdn_opt = if self.starts_with(NOT) {
            self.advance_into(NOT.len(), &mut context.holding);

            self.trim_start_into(&mut context.holding);

            if self.starts_with(EMPTY) {
                self.advance_into(EMPTY.len(), &mut context.holding);
                Some((true, Condition::Emptiness))
            } else {
                None
            }
        } else if self.starts_with(EMPTY) {
            self.advance_into(EMPTY.len(), &mut context.holding);
            Some((false, Condition::Emptiness))
        } else {
            Some((false, Condition::Existence))
        };

        let start_dot = variable.starts_with('.');
        let end_dot = variable.ends_with('.');
        let invalid_excl = match variable.find('!') {
            Some(i) => i != 0,
            None => false,
        };

        let v_is_empty = variable.is_empty();

        if neg_cdn_opt.is_none() || v_is_empty || context.trim_end ||
            start_dot || end_dot || invalid_excl
        {
            return Ok(false);
        }

        let (mut negative, condition) = neg_cdn_opt
            .unwrap();

        self.trim_start_into(&mut context.holding);

        context.trim_start = self.starts_with("-");

        if context.trim_start {
            self.advance_into(1, &mut context.holding);
        }

        if !self.starts_with(TAG[1]) {
            return Ok(false);
        }

        self.advance_into(TAG[1].len(), &mut context.holding);

        let this_neg = variable.starts_with('!');

        if this_neg {
            variable = variable[1..].to_owned();
            negative = !negative;
        }

        // can't have any other ! characters
        if variable.find('!').is_some() || variable.starts_with('.') {
            return Ok(false);
        }

        self.copy_into(TAG[1].len(), &mut context.holding);

        variable = match &context.prefix {
            Some(prefix) => format!("{prefix}.{variable}"),
            None => variable,
        };

        let mut if_ctx = context.with_keyword("if");
        self.parse(&mut if_ctx, cache)?;
        let mut if_content = if_ctx.output;

        match if_ctx.nested_within_keyword.as_str() {
            "else" => {
                let mut else_ctx = context.with_keyword("if");
                self.parse(&mut else_ctx, cache)?;

                match else_ctx.nested_within_keyword.as_str() {
                    "endif" => {
                        let mut else_content = else_ctx.output;

                        match context.implementations.get(&variable) {
                            Some(implementation) => match condition {
                                Condition::Existence => if !negative {
                                    handle_trim(&mut if_content, context.trim_start, if_ctx.trim_end);
                                    context.push_output(&if_content);
                                } else {
                                    handle_trim(&mut else_content, if_ctx.trim_start, else_ctx.trim_end);
                                    context.push_output(&else_content);
                                },
                                Condition::Emptiness => if (implementation.is_empty() && !negative) ||
                                    (!implementation.is_empty() && negative)
                                {
                                    handle_trim(&mut if_content, context.trim_start, if_ctx.trim_end);
                                    context.push_output(&if_content);
                                } else {
                                    handle_trim(&mut else_content, if_ctx.trim_start, else_ctx.trim_end);
                                    context.push_output(&else_content);
                                },
                            },
                            None => match condition {
                                Condition::Existence => if !negative {
                                    handle_trim(&mut else_content, if_ctx.trim_start, else_ctx.trim_end);
                                    context.push_output(&else_content);
                                } else {
                                    handle_trim(&mut if_content, context.trim_start, if_ctx.trim_end);
                                    context.push_output(&if_content);
                                },
                                Condition::Emptiness => if !negative {
                                    handle_trim(&mut if_content, context.trim_start, if_ctx.trim_end);
                                    context.push_output(&if_content);
                                } else {
                                    handle_trim(&mut else_content, if_ctx.trim_start, else_ctx.trim_end);
                                    context.push_output(&else_content);
                                },
                            },
                        }

                        context.clear_holding();
                        context.flip_first();

                        Ok(true)
                    },
                    _ => {
                        Ok(false)
                    },
                }
            },
            "endif" => {
                match context.implementations.get(&variable) {
                    Some(implementation) => match condition {
                        Condition::Existence => if !negative {
                            handle_trim(&mut if_content, context.trim_start, if_ctx.trim_end);
                            context.push_output(&if_content);
                        },
                        Condition::Emptiness => if (implementation.is_empty() && !negative) ||
                            (!implementation.is_empty() && negative)
                        {
                            handle_trim(&mut if_content, context.trim_start, if_ctx.trim_end);
                            context.push_output(&if_content);
                        },
                    },
                    None => match condition {
                        Condition::Existence => if negative {
                            handle_trim(&mut if_content, context.trim_start, if_ctx.trim_end);
                            context.push_output(&if_content);
                        },
                        Condition::Emptiness => if !negative {
                            handle_trim(&mut if_content, context.trim_start, if_ctx.trim_end);
                            context.push_output(&if_content);
                        },
                    },
                }

                context.clear_holding();
                context.flip_first();

                Ok(true)
            },
            _ => {
                Ok(false)
            },
        }
    }

    fn block(&mut self, context: &mut Context, cache: &mut FileCache) -> Result<bool> {
        // first value is the variable name
        let mut variable = String::new();

        while self.starts_with_valid_var_name_char() {
            self.copy_into(1, &mut variable);
            self.advance_into(1, &mut context.holding);
        }

        if variable.is_empty() || context.trim_end {
            return Ok(false);
        }

        self.trim_start_into(&mut context.holding);

        context.trim_start = self.starts_with("-");

        if context.trim_start {
            self.advance_into(1, &mut context.holding);
        }

        if !self.starts_with(TAG[1]) {
            return Ok(false);
        }

        let variable = match &context.prefix {
            Some(prefix) => {
                format!("{prefix}.{variable}")
            },
            None => variable,
        };

        self.advance_into(TAG[1].len(), &mut context.holding);

        let mut block_ctx = context.with_keyword("block");
        self.parse(&mut block_ctx, cache)?;
        let mut block_content = block_ctx.output;
        context.push_holding(&block_ctx.holding);

        match block_ctx.nested_within_keyword.as_str() {
            "endblock" => {
                handle_trim(&mut block_content, context.trim_start, block_ctx.trim_end);
                context.implementations.insert(variable, block_content);

                context.clear_holding();
                context.flip_first();

                Ok(true)
            },
            _ => {
                Ok(false)
            },
        }
    }

    fn comment(&mut self) -> bool {
        while !self.starts_with(COMMENT[1]) {
            self.advance(1);
        }

        self.advance(COMMENT[1].len());

        true
    }

    fn escaped(&mut self, context: &mut Context) -> bool {
        if self.starts_with(CURLY_ESCAPE[0]) {
            self.advance(CURLY_ESCAPE[0].len());
            context.push_holding(&CURLY_ESCAPE[0][1..]);
            true
        } else if self.starts_with(CURLY_ESCAPE[1]) {
            self.advance(CURLY_ESCAPE[1].len());
            context.push_holding(&CURLY_ESCAPE[1][1..]);
            true
        } else if self.starts_with(PERC_ESCAPE) {
            self.advance(PERC_ESCAPE.len());
            context.push_holding(&PERC_ESCAPE[1..]);
            true
        } else if self.starts_with(HASH_ESCAPE) {
            self.advance(PERC_ESCAPE.len());
            context.push_holding(&HASH_ESCAPE[1..]);
            true
        } else {
            false
        }
    }

    fn parse(&mut self, context: &mut Context, cache: &mut FileCache) -> Result<()> {
        while !self.source().is_empty() {
            if self.starts_with(COMMENT[0]) && self.comment() ||
                self.starts_with(VARIABLE[0]) && self.variable(context) ||
                self.escaped(context)
            {
                continue;
            } else if self.starts_with(TAG[0]) {
                context.flush_holding();

                self.advance_into(TAG[0].len(), &mut context.holding);

                context.trim_end = self.starts_with("-");

                if context.trim_end {
                    self.advance_into(1, &mut context.holding);
                }

                self.trim_start_into(&mut context.holding);

                if let Some(keyword) = starts_with_keyword(self.source()) {
                    self.advance_into(keyword.len(), &mut context.holding);

                    self.trim_start_into(&mut context.holding);

                    match keyword.as_str() {
                        "endif" => if context.nested_within_keyword.eq("if") &&
                            self.end_tag("endif", context)
                        {
                            return Ok(());
                        },
                        "endfor" => if context.nested_within_keyword.eq("for") &&
                            self.end_tag("endfor", context)
                        {
                            return Ok(());
                        },
                        "endblock" => if context.nested_within_keyword.eq("block") &&
                            self.end_tag("endblock", context)
                        {
                            return Ok(());
                        },
                        "else" => match context.nested_within_keyword.as_str() {
                            "if"|"for" => if self.end_tag("else", context) {
                                return Ok(());
                            },
                            _ => {},
                        },
                        "extends" => if self.extends(context) {
                            continue;
                        },
                        "include" => if self.include(context, cache)? {
                            continue;
                        },
                        "for" => if self.for_tag(context, cache)? {
                            continue;
                        },
                        "if" => if self.if_tag(context, cache)? {
                            continue;
                        },
                        "block" => if self.block(context, cache)? {
                            continue;
                        },
                        "ignore" => if self.ignore(context)? {
                            continue;
                        },
                        _ => {},
                    }
                }
            }

            if !self.source().is_empty() {
                self.advance_into(1, &mut context.holding);
            }

            context.flip_first();
        }

        if !context.was_extends {
            context.push_holding("\n");
        } else {
            context.was_extends = false;
        }

        context.flip_first();

        if !context.holding.is_empty() {
            if context.holding.ends_with('\n') {
                context.holding = context.holding[0..context.holding.len() - 1].to_owned();
            }

            if !context.holding.is_empty() {
                context.flush_holding();
            }
        }

        let mut extends = None;

        std::mem::swap(&mut context.extends, &mut extends);

        if let Some(extends) = extends {
            let mut extends_parser = Self::from_file(&self.root_dir, extends, cache)?;

            // prep context
            context.directory = extends_parser.base_dir.clone();
            context.is_first = true;
            context.trim_start = false;
            context.trim_end = false;
            context.was_extends = false;
            context.output.clear();

            extends_parser.parse(context, cache)?;

            std::mem::swap(&mut extends_parser, self);
        }

        Ok(())
    }

    /// Compile a template with a given caching mechanism.
    ///
    /// # Arguments
    ///
    /// * `r` - The path to the root directory.
    /// * `p` - The path to the vg template.
    /// * `c` - The caching mechanism.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vg_core::{ FileCache, Parser, };
    ///
    /// let mut cache = FileCache::enabled();
    /// cache.insert("/page.jinja", "This is the page content".to_owned());
    /// let output = Parser::compile_with_cache("/", "/page.jinja", &mut cache).unwrap();
    /// assert_eq!("This is the page content", output);
    /// ```
    pub fn compile_with_cache<R: AsRef<Path>, P: AsRef<Path>>(r: R, p: P, c: &mut FileCache) -> Result<String> {
        let mut parser = Self::from_file(r, p, c)?;
        let mut context = Context::new(parser.base_dir.clone());
        parser.parse(&mut context, c)?;
        Ok(context.output)
    }

    /// Compile a template with caching enabled.
    ///
    /// # Arguments
    ///
    /// * `r` - The path to the root directory.
    /// * `p` - The path to the vg template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vg_core::Parser;
    ///
    /// let output = Parser::compile("./test/full/2", "./test/full/2/page.jinja").unwrap();
    /// let against = include_str!("../test/full/2/against.jinja");
    /// assert_eq!(against[0..against.len()-1], output);
    /// ```
    pub fn compile<R: AsRef<Path>, P: AsRef<Path>>(r: R, p: P) -> Result<String> {
        let mut cache = FileCache::enabled();
        Self::compile_with_cache(r, p, &mut cache)
    }

    /// Compile a template with caching disabled.
    ///
    /// # Arguments
    ///
    /// * `r` - The path to the root directory.
    /// * `p` - The path to the vg template.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vg_core::Parser;
    ///
    /// let output = Parser::compile_no_cache("./test/full/1", "./test/full/1/home.jinja")
    ///     .unwrap();
    /// let against = include_str!("../test/full/1/against_home.jinja");
    /// assert_eq!(against[0..against.len()-1], output);
    /// ```
    pub fn compile_no_cache<R: AsRef<Path>, P: AsRef<Path>>(r: R, p: P) -> Result<String> {
        let mut cache = FileCache::disabled();
        Self::compile_with_cache(r, p, &mut cache)
    }

    /// Compile a template with given implementations and caching mechanism.
    ///
    /// # Arguments
    ///
    /// * `r` - The path to the root directory.
    /// * `p` - The path to the vg template.
    /// * `i` - Implemented variables prior to parsing.
    /// * `c` - The caching mechanism.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vg_core::{ Parser, FileCache, };
    ///
    /// let mut cache = FileCache::enabled();
    /// cache.insert("/template.jinja", "{{ text }}".to_owned());
    /// let impls = [ ("text", "Hello, World!") ];
    /// let output = Parser::compile_implemented_with_cache(
    ///     "/",
    ///     "/template.jinja",
    ///     impls,
    ///     &mut cache
    /// ).unwrap();
    /// assert_eq!("Hello, World!", output);
    /// ```
    pub fn compile_implemented_with_cache<R, P, K, V, Impls>(r: R, p: P, i: Impls, c: &mut FileCache) -> Result<String>
    where
        R: AsRef<Path>,
        P: AsRef<Path>,
        K: AsRef<str>,
        V: AsRef<str>,
        Impls: IntoIterator<Item = (K, V)>
    {
        let mut parser = Self::from_file(r, p, c)?;
        let mut context = Context::new(parser.base_dir.clone());

        i.into_iter().for_each(|(k, v)| {
            context.implementations.insert(
                k.as_ref().to_owned(),
                v.as_ref().to_owned()
            );
        });

        parser.parse(&mut context, c)?;
        Ok(context.output)
    }

    /// Compile a template with caching enabled and the given variable implementations.
    ///
    /// # Arguments
    ///
    /// * `r` - The path to the root directory.
    /// * `p` - The path to the vg template.
    /// * `i` - Implemented variables prior to parsing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vg_core::Parser;
    ///
    /// let impls = [ ("test", "Hello, World!") ];
    /// let output = Parser::compile_implemented(
    ///     "./test/variable/2", "./test/variable/2/template.jinja", impls
    /// ).unwrap();
    /// assert_eq!("Hello, World!", output);
    /// ```
    pub fn compile_implemented<R, P, K, V, Impls>(r: R, p: P, implementations: Impls) -> Result<String>
    where
        R: AsRef<Path>,
        P: AsRef<Path>,
        K: AsRef<str>,
        V: AsRef<str>,
        Impls: IntoIterator<Item = (K, V)>
    {
        let mut cache = FileCache::enabled();
        Self::compile_implemented_with_cache(r, p, implementations, &mut cache)
    }

    /// Compile a template with caching disabled and the given variable implementations.
    ///
    /// # Arguments
    ///
    /// * `r` - The path to the root directory.
    /// * `p` - The path to the vg template.
    /// * `i` - Implemented variables prior to parsing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vg_core::Parser;
    ///
    /// let impls = [ ("test", "Hello, World!") ];
    /// let output = Parser::compile_implemented_no_cache(
    ///     "./test/variable/2", "./test/variable/2/template.jinja", impls
    /// ).unwrap();
    /// assert_eq!("Hello, World!", output);
    /// ```
    pub fn compile_implemented_no_cache<R, P, K, V, Impls>(r: R, p: P, implementations: Impls) -> Result<String>
    where
        R: AsRef<Path>,
        P: AsRef<Path>,
        K: AsRef<str>,
        V: AsRef<str>,
        Impls: IntoIterator<Item = (K, V)>
    {
        let mut cache = FileCache::disabled();
        Self::compile_implemented_with_cache(r, p, implementations, &mut cache)
    }
}
