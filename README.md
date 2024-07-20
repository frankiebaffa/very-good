**Warning**: This project has been flagged as _absolute trash_ by it's creator.
While he does have the power to destroy it, he wishes to show mercy and let it
hang out as a reminder that some ideas are bad.

Superceeded ideologically by [arcana](https://github.com/frankiebaffa/arcana).

> _Use Very Good Templating Engine for all of your templating needs. Or do not.
> I am not a beggar._  
> - **Ron Swanson**, _Parks and Recreation_

## Installation

### From Source

To install from source, you must have `git` and `cargo` installed.

```bash
git clone "https://github.com/frankiebaffa/very-good" NAME
cd NAME
cargo install --path vgc
cargo install --path vgd
```

## Crates

### vg-core

The core functionality of the templating engine.

### vgc

The command-line compiler.

### vgd

A command line deployment program driven by a configuration file. Useful for
moving/compiling files in bulk to a distribution/deployment directory.

## Documentation

### Templates

A `vg` template is just a text file. A template can contain **variables**
(ex: `{{ text }}`), **tags** (ex: `{% block body %}{% endblock body %}`),
**comments** (ex: `{# a comment #}`), and **content**. Variables are
placeholders waiting to be replaced by the output of tags. Tags can define
content and control the flow of a document. Comments are completely ignored.
Content is everything else that is not recognized by the parser.

```htmldjango
<!DOCTYPE html>
<html lang="en">
    <head>
        <title>{% if title %}{{ title }}{% else %}Home{% endif %}</title>
    </head>
    <body>
        <ul>
            {% for section in "/objects/sections" -%}
                <li><a href="{{ section.localurl }}">{{ section.name }}</a></li>
            {% else %}
                <li>No sections have been added... yet.</li>
            {%- endfor %}
        </ul>

        {% if title %}<h1>{{ title }}</h1>{% endif %}

        {% if body %}{{ body }}{% endif %}
    </body>
</html>
```

The above example shows an extendable template. It has variables which could be
defined by tags within a child-template, but it will also compile to viewable
output on its own.

If it were compiled directory using `vgc`, assuming that nothing exists within
the `/objects/sections` directory yet, the following would be the output:

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Home</title>
    </head>
    <body>
        <ul>
            <li>No sections have been added... yet.</li>
        </ul>

        

        
    </body>
</html>
```

A child-template could extend the above template to define some of the variables
such as `{{ title }}` and `{{ body }}`:

```htmldjango
{% extends "/objects/template.jinja" %}
{% block title %}Child{% endblock %}
{% block body -%}
    <p>Here is a paragraph defined by the child-template!</p>
{%- endblock %}
```

If this child-template were compiled using `vgc`, still assuming that nothing
exists within the `/objects/sections` directory, the following would be the
output:

```html
<html lang="en">
    <head>
        <title>Child</title>
    </head>
    <body>
        <ul>
            <li>No sections have been added... yet.</li>
        </ul>

        <h1>Child</h1>

        <p>Here is a paragraph defined by the child-template!</p>
    </body>
</html>
```

### Tags

Defined as `{% keyword [...] %}`.

#### Ignore

```htmldjango
{% ignore %}
```

Instructs the parser to ignore the file entirely. If used on a page it will
return an error. `vgd` will handle this error peacefully and
gives the optional functionality of deleting the destination file when the
source is ignored. If found in an included file or a for-loop item, the file
will be ignored.

##### Constraints

- Must be the first definition of the file, else will be handled as content.
    - All subsequent `ignore` tags will be handled as content, which shouldn't
      be an issue given the file is ignored.

#### Extends

```htmldjango
{% extends "<PATH>" %}
```

Sets the implementing template as a child of the template specified in the
`PATH` value. The implementing template's `block` tags will be used to set the
values for variables found within the `PATH` template.

##### Constraints

- Must be the first definition of the file.
- Only 1 can be defined per file.
    - All subsequent `extends` tags will be handled as content.

#### Include

```htmldjango
{% include "<PATH>" %}
```

Includes the file content of `PATH`. This inclusion occurs in during the parsing
pass on the implementing template, so included tags will be honored.

Consider the file `/section.jinja`:

```htmldjango
{# /section.jinja #}
{% block text %}
    Here is some text.
{% endblock %}
```

As well as the file `/page.jinja`:

```htmldjango
{# /page.jinja #}
{% include "/section.jinja" %}
{{ text }}
```

Compiling `/page.jinja` would result in the following:

```html

Here is some text.
```

This is because the `include` tag reads the content of the target file into the
vg parser, so blocks, loops, etc. are honored.

##### Include As

```htmldjango
{% include "<PATH>" as item %}
```

The `as` keyword allows scoping the templating items found in the included
template. For example: if the template found at `<PATH>` contained a block named
`name`, it will now be effectively named `item.name`.

##### Include Raw

```htmldjango
{% include raw "<PATH>" %}
```

The `raw` keyword reads the file directly with no parsing. This can be used
when a file contains `very-good` syntax.

##### Include Md

```htmldjango
{% include md "<PATH>" %}
```

The `md` keyword reads the file directly into the
[no-flavor markdown](http://frankiebaffa.com/projects/nfm.html) parser and
outputs the result directly with no parsing. This is used to generate these
docs. If the processing of `very-good` tags is needed, consider including within
a block and displaying using the [markdown filter](#variable_filter_md) on the
variable.

#### Block/Endblock

```htmldjango
{% block text %}Here is some text{% endblock text %}
```

Defines content used to set variables in parent templates or in the implementing
template occuring after the `block` tag. The trailing name in the `endblock` tag
is optional and matching the opening tag is completely ignored.

#### If/Else/Endif

##### Exists

```htmldjango
{% if NAME %}{{ NAME }}{% else %}Default thing.{% endif %}
```

Checks for the existence of an implementation of `NAME` and uses the contents
between the `if` and `else` tags if the implementation exists. Otherwise, it
will use the contents between the `else` and `endif` tags.

```htmldjango
{% if !NAME %}Default thing.{% else %}{{ NAME }}{% endif %}
```

The inverse of the aforementioned example. Checks for the absence of an
implementation of `NAME` and uses the contents between `if` and `else` if the
implementation does not exist. Otherwise, it will use the contents between the
`else` and `endif` tags.

##### Empty/Not Empty

```htmldjango
{% if NAME not empty %}{{ NAME }}{% else %}Default name.{% endif %}
```

Checks for not only the existence of an implementation of `NAME`, but also
verifies that the value of the implementation is not empty.

```htmldjango
{% if NAME empty %}Default name.{% else %}{{ NAME }}{% endif %}
```

The inverse of the aforementioned example.

#### For/Else/Endfor

```htmldjango
{% for item in "<PATH-TO-DIR>" %}
    {% if item.text %}
        <p>{{ item.text }}</p>
    {% else %}
        <p>{{ item }}</p>
    {% endif %}
{% else %}
    <p>No items.</p>
{% endfor %}
```

```htmldjango
{% for item in "<PATH-TO-FILE>" %}
    {% if item.text %}
        <p>{{ item.text }}</p>
    {% else %}
        <p>{{ item }}</p>
    {% endif %}
{% else %}
    <p>No items.</p>
{% endfor %}
```

Clones the inner content and implements for the specified file or for each file
in the specified directory. If the file(s) found extend another template, then
the file will be completely compiled before handling the inner content.

### Variables

Defined as `{{ NAME }}`. Variables expect to be implemented by tags. The
output of the variable's implementation can be augmented with
[filters](#variable_filters). If a variable is not wrapped in an `if` block
checking for it's existence, then its definition will be included in the output
when it is not implemented.

#### Nullability

Variables can be defined as nullable by succeeding the name with a
question-mark. This is effectively syntactic-sugar for wrapping the variable in
an if block with an exists condition. Consider the following file at
`/page.jinja`:

```htmldjango
{{ text? }}
```

If this file was compiled, its output would be blank.

#### Filters

Filters modify the content implementing the variable. They can be triggered by
using a pipe after a variable name.

```htmldjango
{% block text %}
    Here is some text.
    And here is some more.
{% endblock %}
{{ text | detab | flatten | trim }}
```

This example would compile to the following.

```html

Here is some text. And here is some more.
```

##### Flatten

Replaces all newlines with spaces.

##### Detab

Removes all tabs.

##### Trim

Trims the start and end of the content.

##### Upper

Makes the content uppercase.

##### Lower

Makes the content lowercase.

##### Replace

```htmldjango
{{ item | replace " " "_" }}
```

Replaces the first quoted item with the second.

##### Md

```htmldjango
{{ item | md }}
```

Parses the item from No-Flavor markdown to html.

### Comments

Defined as `{# CONTENT #}`. Comments are ignored after the initial parsing
pass.

### Tricks

#### Meta Paths

Consider the following template at `/objects/template.jinja`.

```htmldjango
{% for item in "{{ item-directory }}" %}
    {{ item }}
{% endfor %}
```

The **variable** found within the `PATH` portion of the for-tag definition can
be templated in a single pass of the compiler to allow for dynamic extends,
loops, and inclusions.

