# Unicode text styles

Write text in 22 Unicode styles (bold, italic, script, fraktur, circled, strikethrough...) that **keep accents**. In npm, Kotlin and the MCP server (`stylize_text`, `list_styles`).

### Use it

```js
import { stylize, unstyle, listStyles } from 'lettras';

// all 22 styles
stylize('Hola piña').results.map((r) => r.text);

// one style, or several
stylize('Lettras ejemplo: piña, corazón', { style: 'bold' }).results[0].text;
const out = stylize('Lettras', { style: ['bold', 'circled'] });
out.results.map((r) => r.text);   // [ '𝐋𝐞𝐭𝐭𝐫𝐚𝐬', 'Ⓛⓔⓣⓣⓡⓐⓢ' ]

// each result: { id, label, category, text, notNative, parts }
const bold = stylize('piña', { style: 'bold' }).results[0];
bold.text;       // 𝐩𝐢𝐧̃𝐚   (ñ = styled n + combining tilde)
bold.notNative;  // [ 'ñ' ]
unstyle(bold.text);   // 'piña'
listStyles();         // [ { id: 'bold', label: 'Bold', category: 'bold-italic' }, ... 22 in display order ]
```

**Kotlin**

```kotlin
val bold = Lettras().stylize("piña", style = "bold").results.single()
bold.text; bold.notNative; Lettras().unstyle(bold.text); Lettras().listStyles()
```

**MCP** (tools `stylize_text` and `list_styles`, free, no usage limit): ask *"write 'Lettras ejemplo' in bold and script"*, or call it with

```json
{ "name": "stylize_text", "arguments": { "text": "Lettras ejemplo", "style": ["bold", "script"] } }
```

### Example output

#### The name in every style

`stylize('Lettras')` (real output). The list is in display order, which is also the order of `listStyles()`.

| Style id | Name | Lettras |
| --- | --- | --- |
| `bold` | Bold | 𝐋𝐞𝐭𝐭𝐫𝐚𝐬 |
| `italic` | Italic | 𝐿𝑒𝑡𝑡𝑟𝑎𝑠 |
| `bold-italic` | Bold italic | 𝑳𝒆𝒕𝒕𝒓𝒂𝒔 |
| `sans` | Sans-serif | 𝖫𝖾𝗍𝗍𝗋𝖺𝗌 |
| `sans-bold` | Sans-serif bold | 𝗟𝗲𝘁𝘁𝗿𝗮𝘀 |
| `sans-italic` | Sans-serif italic | 𝘓𝘦𝘵𝘵𝘳𝘢𝘴 |
| `sans-bold-italic` | Sans-serif bold italic | 𝙇𝙚𝙩𝙩𝙧𝙖𝙨 |
| `script` | Script | ℒℯ𝓉𝓉𝓇𝒶𝓈 |
| `bold-script` | Bold script | 𝓛𝓮𝓽𝓽𝓻𝓪𝓼 |
| `fraktur` | Fraktur | 𝔏𝔢𝔱𝔱𝔯𝔞𝔰 |
| `bold-fraktur` | Bold fraktur | 𝕷𝖊𝖙𝖙𝖗𝖆𝖘 |
| `double-struck` | Double-struck | 𝕃𝕖𝕥𝕥𝕣𝕒𝕤 |
| `monospace` | Monospace | 𝙻𝚎𝚝𝚝𝚛𝚊𝚜 |
| `small-caps` | Small caps | ʟᴇᴛᴛʀᴀꜱ |
| `fullwidth` | Fullwidth | Ｌｅｔｔｒａｓ |
| `circled` | Circled | Ⓛⓔⓣⓣⓡⓐⓢ |
| `negative-circled` | Negative circled | 🅛🅔🅣🅣🅡🅐🅢 |
| `squared` | Squared | 🄻🄴🅃🅃🅁🄰🅂 |
| `negative-squared` | Negative squared | 🅻🅴🆃🆃🆁🅰🆂 |
| `parenthesized` | Parenthesized | 🄛⒠⒯⒯⒭⒜⒮ |
| `strikethrough` | Strikethrough | L̶e̶t̶t̶r̶a̶s̶ |
| `underline` | Underline | L̲e̲t̲t̲r̲a̲s̲ |

#### Examples

Every style applied to `Lettras ejemplo: piña` (real output of `stylize`). The accent in `piña` is the styled `n` plus a combining
tilde, so how it looks depends on the font; the decoration styles (strikethrough, underline) mark every character, accents included.

| Style id | Result |
| --- | --- |
| `bold` | 𝐋𝐞𝐭𝐭𝐫𝐚𝐬 𝐞𝐣𝐞𝐦𝐩𝐥𝐨: 𝐩𝐢𝐧̃𝐚 |
| `italic` | 𝐿𝑒𝑡𝑡𝑟𝑎𝑠 𝑒𝑗𝑒𝑚𝑝𝑙𝑜: 𝑝𝑖𝑛̃𝑎 |
| `bold-italic` | 𝑳𝒆𝒕𝒕𝒓𝒂𝒔 𝒆𝒋𝒆𝒎𝒑𝒍𝒐: 𝒑𝒊𝒏̃𝒂 |
| `sans` | 𝖫𝖾𝗍𝗍𝗋𝖺𝗌 𝖾𝗃𝖾𝗆𝗉𝗅𝗈: 𝗉𝗂𝗇̃𝖺 |
| `sans-bold` | 𝗟𝗲𝘁𝘁𝗿𝗮𝘀 𝗲𝗷𝗲𝗺𝗽𝗹𝗼: 𝗽𝗶𝗻̃𝗮 |
| `sans-italic` | 𝘓𝘦𝘵𝘵𝘳𝘢𝘴 𝘦𝘫𝘦𝘮𝘱𝘭𝘰: 𝘱𝘪𝘯̃𝘢 |
| `sans-bold-italic` | 𝙇𝙚𝙩𝙩𝙧𝙖𝙨 𝙚𝙟𝙚𝙢𝙥𝙡𝙤: 𝙥𝙞𝙣̃𝙖 |
| `script` | ℒℯ𝓉𝓉𝓇𝒶𝓈 ℯ𝒿ℯ𝓂𝓅𝓁ℴ: 𝓅𝒾𝓃̃𝒶 |
| `bold-script` | 𝓛𝓮𝓽𝓽𝓻𝓪𝓼 𝓮𝓳𝓮𝓶𝓹𝓵𝓸: 𝓹𝓲𝓷̃𝓪 |
| `fraktur` | 𝔏𝔢𝔱𝔱𝔯𝔞𝔰 𝔢𝔧𝔢𝔪𝔭𝔩𝔬: 𝔭𝔦𝔫̃𝔞 |
| `bold-fraktur` | 𝕷𝖊𝖙𝖙𝖗𝖆𝖘 𝖊𝖏𝖊𝖒𝖕𝖑𝖔: 𝖕𝖎𝖓̃𝖆 |
| `double-struck` | 𝕃𝕖𝕥𝕥𝕣𝕒𝕤 𝕖𝕛𝕖𝕞𝕡𝕝𝕠: 𝕡𝕚𝕟̃𝕒 |
| `monospace` | 𝙻𝚎𝚝𝚝𝚛𝚊𝚜 𝚎𝚓𝚎𝚖𝚙𝚕𝚘: 𝚙𝚒𝚗̃𝚊 |
| `small-caps` | ʟᴇᴛᴛʀᴀꜱ ᴇᴊᴇᴍᴘʟᴏ: ᴘɪɴ̃ᴀ |
| `fullwidth` | Ｌｅｔｔｒａｓ ｅｊｅｍｐｌｏ： ｐｉｎ̃ａ |
| `circled` | Ⓛⓔⓣⓣⓡⓐⓢ ⓔⓙⓔⓜⓟⓛⓞ: ⓟⓘⓝ̃ⓐ |
| `negative-circled` | 🅛🅔🅣🅣🅡🅐🅢 🅔🅙🅔🅜🅟🅛🅞: 🅟🅘🅝̃🅐 |
| `squared` | 🄻🄴🅃🅃🅁🄰🅂 🄴🄹🄴🄼🄿🄻🄾: 🄿🄸🄽̃🄰 |
| `negative-squared` | 🅻🅴🆃🆃🆁🅰🆂 🅴🅹🅴🅼🅿🅻🅾: 🅿🅸🅽̃🅰 |
| `parenthesized` | 🄛⒠⒯⒯⒭⒜⒮ ⒠⒥⒠⒨⒫⒧⒪: ⒫⒤⒩̃⒜ |
| `strikethrough` | L̶e̶t̶t̶r̶a̶s̶ e̶j̶e̶m̶p̶l̶o̶:̶ p̶i̶ñ̶a̶ |
| `underline` | L̲e̲t̲t̲r̲a̲s̲ e̲j̲e̲m̲p̲l̲o̲:̲ p̲i̲ñ̲a̲ |

**What a style cannot draw.** `ñ`, `ó`, `ü`... are flagged in `notNative` for every letter style, because the accent is a combining mark. `ß`, `Æ`, `Œ`, `Ø` and non-Latin scripts have no styled version and stay plain:

```js
stylize('Straße', { style: 'script' }).results[0].parts;
// [ { text: '𝒮𝓉𝓇𝒶', missing: false }, { text: 'ß', missing: true }, { text: 'ℯ', missing: false } ]
// show the missing piece as an empty box, or warn the user
```

**Classic mode** strips accents first, so nothing is flagged:

```js
stylize('piña', { style: 'bold', classic: true }).results[0];
// { id: 'bold', text: '𝐩𝐢𝐧𝐚', notNative: [], ... }
```

**One style, several styles, or all** (`results` is always an array):

```js
stylize('Hola', { style: 'bold' }).results.length;              // 1
stylize('Hola', { style: ['bold', 'script'] }).results.length;  // 2
stylize('Hola').results.length;                                 // 22
```

### How it works

1. Each letter and digit is swapped for its look-alike in the Unicode "Mathematical Alphanumeric Symbols" (and the circled, squared, fullwidth... blocks), so the result is plain text you can paste anywhere that accepts Unicode: bios, posts, names.
2. **Accents are kept.** An accented letter becomes its styled base letter plus the Unicode combining mark (`ñ` = styled `n` + combining tilde), so `piña` stays readable. How an accent looks depends on the font of the place you paste it.
3. **What a style cannot draw is reported, not hidden.** `notNative` lists the input characters the style cannot draw natively (`ß`, `Æ`, `Œ`, `Ø`, non-Latin scripts, and accents that only exist as a combining mark), and `parts` cuts the text into pieces where `missing: true` marks what to show as an empty box.
4. `classic: true` strips the accents first. `unstyle(text)` turns styled text back into plain text. Text is limited to 10,000 characters; unknown style ids throw.
