# mdTeX
A leightweight converter for **Markdown** (my own flavour!) to **PDF-slides** (Latex Beamer) written in Rust.

Creates `.pdf` and and `.tex` file for slideshows.

>There are many Markdown to PDF-slides converter. 
>But as of my knowledge none of them can position images freely on slides.
>To archieve this, I wrote my own Markdown-flavour that support a few Latex-Beamer functionalities and is easy to read, fast to write in Markdown style.

## More features coming...
* "floating" positionable Text-boxes.
* Code syntax support

# Documentation
An exmaple Markdown file can be found [here](./examples/demo.md).

You can convert it by `./mdTex demo.md`. This will generate a `demo.tex` and `demo.pdf` file at the same location.

## Metadata Setup
* At the beginning of the file
* starts and ends with `%%`
* key-value pairs that define global configurations

1. **theme:** Sets the theme of the presentation to one of the Latex Beamer Themes.
2. **title:** Specifies the title of the presentation.
3. **author:** Specifies the author of the presentation.
4. **date:** Inserts the current date. Either a date (eg. 10.01.2024) or LaTeX's `\today`.
5. **indentation:** Sets the indentation level to 4 spaces.

## Text formatting
* enclosed by `*` :  Text is *italic*
* enclosed by `**`: Text is **bold**
* enclosed by `_`: Text is underlined
* enclosed by `-`: Text is crossed


## Beamer features
* `---`: Separates slides.
    * With slide title: `--- This is the slide title`.
* `#`: Huge Heading.
* `##`:  Large.
* `###`: Footnotesized heading.
* `// This is a comment`: Adds comments.
* `[Box title][[This text is inside a box]]`: Creates a box with a title and content.
* `[][[This box has no Title]]`: Creates a box without a title but with content.
* `[[This is also a box]]`: Also creates a box.


## Columns

* `::`: Indicates the start and end of column sections.
* `:40%`, `:60%`: Specifies the percentage width of columns.

### Example
```
::
:40%
Left column text
:60%
right column text
::
```
There can be more than two columns.

## Images

* `![x:0.0,y:0.0,w:0.25][This is the Image caption](./test.jpg)`: Inserts an image with parameters for positioning and width, along with an **optional** caption.
* If positioning is not needed, the x,y,w values should be set to 0.0

## General notes

* Paragraphs are separated by an empty line.
* Tabs as indentation do not work.
