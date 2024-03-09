# Very Good Templating Engine

File-system-based Jinja-like templating.

## Templates

A vg template is just a text file. A template can contain **variables**
(ex: `{{ text }}`), **tags** (ex: `{% block body %}{% endblock body %}`),
**comments** (ex: `{# a comment #}`), and **content**. Variables are placeholders
waiting to be replaced by the output of block-tags. Tags can define content and
control the flow of a document. Comments are completely ignored. Content is
everything else that is not recognized by the parser.

```htmldjango
<!DOCTYPE html>
<html lang="en">
    <head>
        <title>{% if title %}{{ title }}{% else %}Home{% endif %}</title>
    </head>
    <body>
        <ul>
            {% for section in "./sections" -%}
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

If it were compiled directory using `vgc`, assuming that nothing exists
within the `./sections` directory yet, the following would be the output:

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
{% extends "./template.jinja" %}
{% block title %}Child{% endblock %}
{% block body -%}
    <p>Here is a paragraph defined by the child-template!</p>
{%- endblock %}
```

If this child-template were compiled using `vgc`, still assuming that nothing
exists within the `./sections` directory, the following would be the output:

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

## Tags

Defined as `{% keyword [...] %}`.

### `extends`

```htmldjango
{% extends "<PATH>" %}
```

Sets the implementing template as a child of the template specified in the PATH
value. The implementing template's `block` tags will be used to set the values
for variables found within the PATH template.

### `include`/`include as`

```htmldjango
{% include "<PATH>" %}
```

Includes the file content of PATH. This inclusion occurs in during the parsing
pass on the implementing template, so included tags will be honored.

```htmldjango
{% include "<PATH>" as item %}
```

The `as` keyword allows scoping the templating items found in the included
template. For example: if the template found at `<PATH>` contained a block
named `name`, it will now be effectively named `item.name`.

### `block`/`endblock`

```htmldjango
{% block text %}Here is some text{% endblock text %}
```

Defines content used to set variables in parent templates or in the implementing
template occuring after the `block` tag. The trailing name in the `endblock` tag
is optional and matching the opening tag is completely ignored.

### `if`/`else`/`endif`

#### `exists` condition

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

#### `empty`/`not empty` condition

```htmldjango
{% if NAME not empty %}{{ NAME }}{% else %}Default name.{% endif %}
```

Checks for not only the existence of an implementation of `NAME`, but also
verifies that the value of the implementation is not empty.

```htmldjango
{% if NAME empty %}Default name.{% else %}{{ NAME }}{% endif %}
```

The inverse of the aforementioned example.

### `for`/`else`/`endfor`

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
the file will be completely compiled before handling the inner content. If the
file(s) found only contain(s) `block` tags, then these tags can be accessed
as implemented variables.

## Variables

Defined as `{{ NAME }}`. Variables expect to be implemented by tags.

### Filters

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

```htmldjango

Here is some text. And here is some more.
```

#### `flatten`

Replaces all newlines with spaces.

#### `detab`

Removes all tabs.

#### `trim`

Trims the start and end of the content.

## Comments

Defined as `{# CONTENT #}`. Comments are ignored after the initial parsing pass.

## Tricks

### Meta Paths

Consider the following template at `./template.jinja`.

```htmldjango
{% for item in "{{ item-directory }}" %}
    {{ item }}
{% endfor %}
```

The **variable** found within the `PATH` portion of the for-tag definition can
be templated in a single pass of the compiler to allow for dynamic extends,
loops, and inclusions.

