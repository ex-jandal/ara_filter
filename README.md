# ara-filter

`ara-filter` is a Rust-based tool designed to normalize and clean Arabic text. It handles various Arabic character variants, presentation forms, diacritics (Tashkeel), and special symbols, mapping them back to their standard Arabic counterparts.

## Features

- **Character Normalization**: Maps extended Arabic characters and presentation forms (ligatures) to their base letters (e.g., Alif variants to 'ا', Kaf variants to 'ك').
- **Diacritic Removal**: Automatically strips Tashkeel (Fatha, Damma, Kasra, etc.) and Tatweel (Kashida).
- **Punctuation Cleaning**: Removes both Arabic and standard ASCII punctuation.
- **Trap Handling**: Filters out invisible formatting characters (ZWNJ, ZWJ, BOM, etc.) often used to bypass filters.
- **Macro Expansion**: Expands special symbols like the Bismillah (`\u{FDFD}`) into full text.
- **File Processing**: Reads from `file_income.txt` and writes the cleaned output to `file_outcome.txt`.

## Usage

### Prerequisites

- Rust (latest stable version recommended)

### Running the Project

1. Ensure you have a file named `file_income.txt` in the root directory containing the Arabic text you want to filter.
2. Run the project using Cargo:

```bash
cargo run
```

3. The normalized text will be saved in `file_outcome.txt`.

## How it Works

The tool utilizes a predefined map of Arabic characters (`ara_letters.rs`) that categorizes characters into:
- **Real Letters**: Standard Arabic alphabet (U+0621 to U+064A).
- **Fake/Alternative Letters**: Regional variants, decorative forms, and presentation ligatures that are mapped to a "parent" real letter.

The filtering process iterates through the input string, expanding macros, removing unwanted symbols, and replacing decorative variants with their base forms.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
