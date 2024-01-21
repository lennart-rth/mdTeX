%%
theme: Darmstadt
title: This is the title
author: Your name
date: \today
indentation: 4
%%

--- Slide with a title

// This is a comment

[Box title][[This text is inside a box]]
[][[This box has no Title]]
[[This is also a box]]


--- 

# This is a large Heading
Unfortunately headings can´t have text styling in them.

* This is a **bold** word. 
    * lists can be unordered or ordered
* This is a *italic* word.

1. This word is _underlined_
    1. this is a ordered and nested list
2. This word is -crossed-

---

# There are large Headings
A paragraph is ended by a empty line.

This is the next paragraph.
## Medium headings
This paragraph is 
escaped with a **double backslash**.
### Small headings

--- Vertical Columns

* Column sections are started and ended with a ::
* a new Column is started with a : following a percentage of the columns width. (eg. ":40%")

::
:40%
Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.
:60%
At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. 
::

--- Images

Images have 3 parameters:
1. x coordinate
2. y coordinate
3. width

x,y,w are necessary and are delimited by a ",". If no positioning is needed they can be left as 0
The Box for the Caption can also be empty or leaved out.

![x:0.0,y:0.0,w:0.25][This is the Image caption](./test.jpg)