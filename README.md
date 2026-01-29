# Important links
- [Logos Documentation](target/doc/logos/index.html)
  - [Codegen](target/doc/logos_codegen/index.html)
  - [Derive](target/doc/logos_derive/index.html)
- [`EBNF` Syntax](src/main.ebnf)
# Markup Language
## Languages
|Written in|Output|
|:-:|:-:|
|`Rust`|`HTML`|
## Features
Every feature is inline, unless applied to a block.
Inline `HTML` is not allowed & will not be read as `HTML`.
### Bold
```html
<b>CONTENT</b>
```
### Italic
```html
<i>CONTENT</i>
```
### Underline
```html
<u>CONTENT</u>
```
### Code
```html
<code
	data-lang="LANGUAGE"
	escaped
>CONTENT</code>
```
### Header
```html
<h_>CONTENT</h_>
```
### Id + Classes
Element is a `<span>`, unless any other element can overwrite (`<span>` is a fallback).
Only 1 `#id` is allowed; any others are ignored.
There is no limit to the amount of `.classes`. They must each be prefixed by a `.`.
```html
<span
	id=ID
	class=CLASSES
>CONTENT</span>
```
### Blocks
Blocks allow features to be applied to a block of text, rather than inline.
Any styling suffixed to the initial pipe (`|`) will be applied to the whole block.
Nesting can be achieved by adding pipes (more pipes towards the bottom-level).
```html
<div STYLING>CONTENT</div>
```
### Escaping
Escaping only applies to individual characters.
```ts
entity
	? `&${entity as string};`
	: `&#x${hex as number};`
```
### Commands
All commands are prefixed with `/`.
If the following keyword is not recognized as a command, `/` will be treated as a literal.
```ts
CMD(Array<string | number>: ...ARG)
```
