# mdTex
A leightweight converter for **Markdown** (my own flavour!) to **PDF-slides** (Latex Beamer) written in Rust.

Creates `.pdf` and and `.tex` file for slideshows.

>There are many Markdown to PDF Slides converter. 
>But as of my knowledge none of them can position Images  freely on slides.
>To archieve this I wrote my own Markdown-flavour that support a few Latex-Beamer functionalities in easy to read, fast to write Markdown style.

## More Features coming...
* "floating" positionable Text-boxes.
* Code syntax support

# Documentation
An exmaple Markdown file can be found [here](./examples/demo.md).
## Metadata Setup
* At the beginning of the file
* starts and ends with `%%`
* key-value pairs that define global configurations

1. **theme:** Sets the theme of the presentation to one of the Latex Beamer Themes.
2. **title:** Specifies the title of the presentation.
3. **author:** Specifies the author of the presentation.
4. **date:** Inserts the current date. Either a Date (eg. 10.01.2024) or LaTeX's `\today`.
5. **indentation:** Sets the indentation level to 4 spaces.

## Text Formatting
* enclosed by `*` :  Text is *italic*
* enclosed by `**`: Text is **bold**
* enclosed by `_`: Text is underlined
* enclosed by `-`: Text is crossed


## Beamer Features
* `---`: Separates slides.
    * With slide title: `--- This is the slide title`.
* `#`: Huge Heading.
* `##`:  Large.
* `###`: Footnotesized Heading.
* `// This is a comment`: Adds comments.
* `[Box title][[This text is inside a box]]`: Creates a box with a title and content.
* `[][[This box has no Title]]`: Creates a box without a title but with content.
* `[[This is also a box]]`: Also Creates a box.


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

## General Notes

* Paragraphs are separated by an empty line.
* Tabs as indentation do not work.
