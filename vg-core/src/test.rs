// vg-core::test: The primary tests for the Very Good Templating Engine.
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

//! The primary tests behind the Very Good Templating Engine.

use crate::{ FileCache, Parser, };

#[test]
fn escape_1() {
    let output = Parser::compile(
        "./test/escape/1",
        "./test/escape/1/template.jinja"
    ).unwrap();

    let against = include_str!("../test/escape/1/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn regular_1() {
    let output = Parser::compile(
        "./test/normal/1",
        "./test/normal/1/template.jinja"
    ).unwrap();

    let against = include_str!("../test/normal/1/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn regular_2() {
    let output = Parser::compile(
        "./test/normal/2",
        "./test/normal/2/template.jinja"
    ).unwrap();

    let against = include_str!("../test/normal/2/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn block_1() {
    let output = Parser::compile(
        "./test/block/1",
        "./test/block/1/template.jinja"
    ).unwrap();

    let against = include_str!("../test/block/1/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn block_2() {
    let output = Parser::compile(
        "./test/block/2",
        "./test/block/2/template.jinja"
    ).unwrap();

    let against = include_str!("../test/block/2/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn block_3() {
    let output = Parser::compile(
        "./test/block/3",
        "./test/block/3/template.jinja"
    ).unwrap();

    let against = include_str!("../test/block/3/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn block_4() {
    let output = Parser::compile(
        "./test/block/4",
        "./test/block/4/template.jinja"
    ).unwrap();

    let against = include_str!("../test/block/4/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn block_5() {
    let output = Parser::compile(
        "./test/block/5",
        "./test/block/5/template.jinja"
    ).unwrap();

    let against = include_str!("../test/block/5/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn block_6() {
    let output = Parser::compile(
        "./test/block/6",
        "./test/block/6/template.jinja"
    ).unwrap();

    let against = include_str!("../test/block/6/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn block_7() {
    let output = Parser::compile(
        "./test/block/7",
        "./test/block/7/template.jinja"
    ).unwrap();

    let against = include_str!("../test/block/7/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn comment_1() {
    let output = Parser::compile(
        "./test/comment/1",
        "./test/comment/1/template.jinja"
    ).unwrap();

    let against = include_str!("../test/comment/1/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_1() {
    let output = Parser::compile(
        "./test/if/1",
        "./test/if/1/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/1/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_2() {
    let output = Parser::compile(
        "./test/if/2",
        "./test/if/2/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/2/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_3() {
    let output = Parser::compile(
        "./test/if/3",
        "./test/if/3/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/3/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_4() {
    let output = Parser::compile(
        "./test/if/4",
        "./test/if/4/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/4/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_5() {
    let output = Parser::compile(
        "./test/if/5",
        "./test/if/5/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/5/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_6() {
    let output = Parser::compile(
        "./test/if/6",
        "./test/if/6/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/6/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_7() {
    let output = Parser::compile(
        "./test/if/7",
        "./test/if/7/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/7/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_8() {
    let output = Parser::compile(
        "./test/if/8",
        "./test/if/8/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/8/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_9() {
    let output = Parser::compile(
        "./test/if/9",
        "./test/if/9/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/9/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_10() {
    let output = Parser::compile(
        "./test/if/10",
        "./test/if/10/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/10/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_11() {
    let output = Parser::compile(
        "./test/if/11",
        "./test/if/11/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/11/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn if_12() {
    let output = Parser::compile(
        "./test/if/12",
        "./test/if/12/template.jinja"
    ).unwrap();

    let against = include_str!("../test/if/12/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn include_1() {
    let output = Parser::compile(
        "./test/include/1",
        "./test/include/1/template.jinja"
    ).unwrap();

    let against = include_str!("../test/include/1/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn include_2() {
    let output = Parser::compile(
        "./test/include/2",
        "./test/include/2/template.jinja"
    ).unwrap();

    let against = include_str!("../test/include/2/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn include_3() {
    let output = Parser::compile(
        "./test/include/3",
        "./test/include/3/template.jinja"
    ).unwrap();

    let against = include_str!("../test/include/3/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn include_4() {
    let output = Parser::compile(
        "./test/include/4",
        "./test/include/4/template.jinja"
    ).unwrap();

    let against = include_str!("../test/include/4/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn include_5() {
    let output = Parser::compile(
        "./test/include/5",
        "./test/include/5/template.jinja"
    ).unwrap();

    let against = include_str!("../test/include/5/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn include_6() {
    let root = "./test/include/6";
    let path = FileCache::rebase_path(root, "/", "/include.jinja");
    let mut cache = FileCache::enabled();
    cache.insert(path, "Manually included in cache.".to_owned());
    println!("{cache:#?}");

    let output = Parser::compile_with_cache(
        "./test/include/6",
        "./test/include/6/template.jinja",
        &mut cache
    ).unwrap();

    let against = include_str!("../test/include/6/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn for_1() {
    let output = Parser::compile(
        "./test/for/1",
        "./test/for/1/template.jinja"
    ).unwrap();

    let against = include_str!("../test/for/1/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn for_2() {
    let output = Parser::compile(
        "./test/for/2",
        "./test/for/2/template.jinja"
    ).unwrap();

    let against = include_str!("../test/for/2/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn for_3() {
    let output = Parser::compile(
        "./test/for/3",
        "./test/for/3/template.jinja"
    ).unwrap();

    let against = include_str!("../test/for/3/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn for_4() {
    let output = Parser::compile(
        "./test/for/4",
        "./test/for/4/template.jinja"
    ).unwrap();

    let against = include_str!("../test/for/4/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn for_5() {
    let output = Parser::compile(
        "./test/for/5",
        "./test/for/5/template.jinja"
    ).unwrap();

    let against = include_str!("../test/for/5/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn for_6() {
    let output = Parser::compile(
        "./test/for/6",
        "./test/for/6/template.jinja"
    ).unwrap();

    let against = include_str!("../test/for/6/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn for_7() {
    let output = Parser::compile(
        "./test/for/7",
        "./test/for/7/template.jinja"
    ).unwrap();

    let against = include_str!("../test/for/7/against.jinja");

    assert_eq!(against, output);
}

#[test]
fn for_8_name_default() {
    let output = Parser::compile(
        "./test/for/8",
        "./test/for/8/by_name_default.jinja"
    ).unwrap();

    let against = include_str!("../test/for/8/against_reverse.jinja");

    assert_eq!(against[0..against.len()-1], output);
}

#[test]
fn for_8_name() {
    let output = Parser::compile(
        "./test/for/8",
        "./test/for/8/by_name.jinja"
    ).unwrap();

    let against = include_str!("../test/for/8/against_reverse.jinja");

    assert_eq!(against[0..against.len()-1], output);
}

#[test]
fn for_8_name_reverse() {
    let output = Parser::compile(
        "./test/for/8",
        "./test/for/8/by_name_reverse.jinja"
    ).unwrap();

    let against = include_str!("../test/for/8/against.jinja");

    assert_eq!(against[0..against.len()-1], output);
}

#[test]
fn extends_1() {
    let output = Parser::compile(
        "./test/extends/1",
        "./test/extends/1/fragment.jinja"
    ).unwrap();

    let against = include_str!("../test/extends/1/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn extends_2() {
    let output = Parser::compile(
        "./test/extends/2",
        "./test/extends/2/fragment.jinja"
    ).unwrap();

    let against = include_str!("../test/extends/2/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn extends_3() {
    let output = Parser::compile(
        "./test/extends/3",
        "./test/extends/3/sub_fragment.jinja"
    ).unwrap();

    let against = include_str!("../test/extends/3/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn extends_4() {
    let output = Parser::compile_implemented(
        "./test/extends/4",
        "./test/extends/4/template.jinja",
        vec![
            ("header".to_owned(), "The header".to_owned()),
            ("text".to_owned(), "Here is some text.".to_owned()),
        ]
    ).unwrap();

    let against = include_str!("../test/extends/4/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn extends_5() {
    let output = Parser::compile_implemented(
        "./test/extends/5",
        "./test/extends/5/page.jinja",
        vec![
            ("extends".to_owned(), "./template.jinja".to_owned()),
        ]
    ).unwrap();

    let against = include_str!("../test/extends/5/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn extends_6() {
    let output = Parser::compile_implemented(
        "./test/extends/6",
        "./test/extends/6/pages/fragment.jinja",
        vec![
            ("extends".to_owned(), "./template.jinja".to_owned()),
        ]
    ).unwrap();

    let against = include_str!("../test/extends/6/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn full_1_home() {
    let output = Parser::compile(
        "./test/full/1",
        "./test/full/1/home.jinja"
    ).unwrap();

    let against = include_str!("../test/full/1/against_home.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn full_1_home_trim() {
    let output = Parser::compile(
        "./test/full/1",
        "./test/full/1/home_trim.jinja"
    ).unwrap();

    let against = include_str!("../test/full/1/against_home_trim.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn full_1_items() {
    let output = Parser::compile(
        "./test/full/1",
        "./test/full/1/items.jinja"
    ).unwrap();

    let against = include_str!("../test/full/1/against_items.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn full_2() {
    let output = Parser::compile(
        "./test/full/2",
        "./test/full/2/page.jinja"
    ).unwrap();

    let against = include_str!("../test/full/2/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn full_3() {
    let output = Parser::compile(
        "./test/full/3/pages",
        "./test/full/3/pages/page.jinja"
    ).unwrap();

    let against = include_str!("../test/full/3/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
#[should_panic]
fn ignore_1() {
    Parser::compile(
        "./test/ignore/1",
        "./test/ignore/1/template.jinja"
    ).unwrap();
}

#[test]
fn ignore_2() {
    let output = Parser::compile(
        "./test/ignore/2",
        "./test/ignore/2/template.jinja"
    ).unwrap();

    let against = include_str!("../test/ignore/2/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn ignore_3() {
    let output = Parser::compile(
        "./test/ignore/3",
        "./test/ignore/3/template.jinja"
    ).unwrap();

    let against = include_str!("../test/ignore/3/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn variable_1() {
    let output = Parser::compile(
        "./test/variable/1",
        "./test/variable/1/template.jinja"
    ).unwrap();

    let against = include_str!("../test/variable/1/against.jinja");

    assert_eq!(&against[0..against.len()-1], output);
}

#[test]
fn variable_2() {
    let output = Parser::compile(
        "./test/variable/2",
        "./test/variable/2/template.jinja"
    ).unwrap();

    let against = include_str!("../test/variable/2/against.jinja");

    assert_eq!(against, output);
}
