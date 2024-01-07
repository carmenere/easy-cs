# Unicode
The **Unicode Consortium** is a non-profit corporation devoted to **developing**, **maintaining**, and promoting software **internationalization standards**.<br>
The **Unicode Standard** is a universal standard that enables consistent encoding of multilingual text.<br>

<br>

The **Unicode Standard** specifies a **code points** and **names** for **each character** and defines **3 Unicode character encoding schemes**.
But **Unicode Standard** **does not** specify which font to use.<br>

<br>

The first **128** code points of **Unicode** mirror the **ASCII** standard.<br>

<br>

# Unicode encodings
**Unicode character encoding scheme** answers on question `How to convert code point of some character to its binary value?`.<br>

The **Unicode Standard** defines **3 Unicode character encoding schemes**:
- **UTF-8**;
- **UTF-16** (aka **UCS-2**);
- **UTF-32** (aka **UCS-4**);

**UTF** is an abbrevation for Unicode Transformation Format.<br>
**UCS** is an abbrevation for Universal Character Set.<br>

<br>

## UTF-8
**UTF-8** uses **variable length encoding** and it can use `1`, `2`, `3` or `4` bytes for codes.<br>

- `0xxxxxxx` - This is **1 byte** code.
- `110xxxxx` `10xxxxxx` - This is **2 bytes** code.
- `1110xxxx` `10xxxxxx` `10xxxxxx` - This is **3 bytes** code.
- `11110xxx` `10xxxxxx` `10xxxxxx` `10xxxxxx` - This is **4 bytes** code.

<br>

So:
- if there are **no leading** `1` it is **ASCII**;
- amount of **leading** `1` before `0` in **first byte** tells how many bytes in sequence and following bytes must start with `10`;

<br>

The **1 byte code** is identical to an **ASCII** code, in other words **UTF-8** is **compatible** with **ASCII**. So, a **UTF-8** file that contains only **ASCII** characters is identical to an **ASCII** file. Legacy programs can generally handle **UTF-8** encoded files, even if they contain **non-ASCII characters**.<br>

**UTF-8** is **more efficient for storage of English** (ASCII), whereas other language data is expanded and can be represented by up to four bytes.

<br>

## UTF-16
**UTF-16** uses **variable length encoding** and it can use `2` or `4` bytes for codes.<br>
**UTF-16** is **incompatible with ASCII** files and thus requires Unicode-aware programs.

<br>

## UTF-32
**UTF-32** stores character using `4` bytes.<br>

<br>

The main **advantages** of **UTF-32**
- the Unicode code points are **directly indexed**. Finding the **Nth** code point in a sequence of code points is a constant-time operation.<br>

<br>

The main **disadvantages** of **UTF-32**:
- it is **space-inefficient**, because it uses `4` bytes for **all code points**;
- it is **incompatible with ASCII** files and thus requires Unicode-aware programs;

<br>

# Grapheme cluster
**Grapheme cluster** is a **sequence of code points** that should be treated as **single unit when processed**.<br>
For example, `CRLF` is **grapheme cluster** for line break.<br>
The **grapheme clusters** is used for rendering and editing purposes.<br>
Unlike *composition*, *grapheme cluster* **doesn't** produce another code point as a result.<br>
**Grapheme cluster** has **length of 1**.

<br>

# Collation
**Collation** is an instruction on how to **compare** or **sort** texts.<br>
How to sort texts?
- `A B C a b c ą Ć` or `a A ą b B c C Ć`;
- `a a ą ą` or `ą ą a a`;

<br>

# Byte order mark
**BOM** (**Byte Order Mark**) is a **special sequence of bytes** at the beginning of file or data stream to notify which byte order is used (**big endian** or **little endian**).<br>

|Encoding|Byte order|Code point|
|:-------|:---------|:---------|
|**UTF-8**|Byte order has no meaning in **UTF-8**|`EF BB BF`|
|**UTF-16**|Big-endian|`FE FF`|
|**UTF-16**|Little-endian|`FF FE`|
|**UTF-32**|Big-endian|`00 00 FE FF`|
|**UTF-32**|Little-endian|`FF FE 00 00`|

<br>

Byte order has no meaning in **UTF-8**, so its only use in **UTF-8** is to signal at the start that the text stream is encoded in **UTF-8**.<br>

BOM idea may sound weird today, because **UTF-8** became prevalent and dominant.
