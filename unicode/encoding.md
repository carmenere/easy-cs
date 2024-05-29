# Encoding
**Character encoding** (or just **encoding**) is the process of assigning numbers to characters to represent them within a computer.<br>
**Code point** is a **numerical value** that **maps** to a **specific character**.<br>
**All code points** of all chracters comprise **code space** (aka **code page** or **character map**).<br>

<br>

A **glyph** is an **image** that used to represent **graphemes**.<br>
A **font** is a **collection** of **glyphs**. In other words **font** is a design of letters, numbers and other symbols, to be used in printing.<br>
A **typeface** is a **font family**. A **proportional typeface** uses glyphs of **varying widths**, while a **monospaced typeface** uses glyphs of **fixed-width**.<br>
Every **glyph** has **code point**.<br>
A **grapheme** is a **sequence** of **one** or **more code points** that are displayed as a **single graphical unit**.<br>
For example, both `a` and `ä` are graphemes, but they may consist of **multiple code points**.<br>

<br>

A *characters* **haven't one-to-one maping** between *graphemes*. For example, **control characters** (aka **non-printable characters**) have **code points** but they are **not** functional units of *writing system*, therefore they are **not** *graphemes*. So, some code points are **not** mapped to graphemes, they are for control.<br>

<br>

**Every character** (*grapheme*, *control character*) has some **code point** that rendered as some **glyph** in given **font**.

<br>

# ASCII
**ASCII** is an abbrevation for American Standard Code for Information Interchange.<br>

The **ASCII table** or just **ASCII** is a **7-bit character encoding system** that can store `2^7=128` characters:
- *lowercase* and *uppercase* Latin **letters**;
- **arabic numerals**;
- **punctuations**;
- **special symbols**;
- **control characters** which do not have graphical representation but have some effect like moving to new line;

<br>

So, **ASCII** has **128** **code points**, of which only **95** are **printable** characters and **33** are **non-printing** control characters.

<br>

# ISO-8859-1
**ISO-8859-1** (aka **ISO/IEC 8859-1**, aka **Latin-1**) **8-bit character encoding system** that extends **ASCII** and can store **256** unique characters.
The first **128** code points of **ISO-8859-1** mirror the **ASCII** standard.
