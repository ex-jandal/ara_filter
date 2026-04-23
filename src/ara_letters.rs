use std::collections::HashMap;
use std::sync::Arc;

type RefLetter = Arc<Letter>;

#[derive(Debug)]
pub struct Letter {
    character: char,
    alternative: Option<RefLetter>,
}

impl Letter {
    fn new(offical: char, alteritive: Option<RefLetter>) -> RefLetter {
        Arc::new(Self {
            character: offical,
            alternative: alteritive,
        })
    }
}

pub fn all_letters() -> [Vec<Arc<Letter>>; 2] {
    let mut real_map: HashMap<char, Arc<Letter>> = HashMap::with_capacity(37);

    let real_ranges = [(0x0621, 0x063B), (0x0641, 0x064A)];
    for (start, end) in real_ranges {
        for cp in start..=end {
            if let Some(c) = char::from_u32(cp) {
                real_map.insert(c, Letter::new(c, None));
            }
        }
    }

    let mut fake_letters = Vec::with_capacity(221);
    for cp in 0x0671..=0x06D5 {
        if let Some(c) = char::from_u32(cp) {
            let parent_char = match c {
                // Alif Variants
                'ٱ' | 'ٲ' | 'ٳ' | 'ٵ' => Some('ا'),

                // Ba / Ta / Tha Variants (including retroflex)
                'ٻ' | 'ڀ' | 'پ' => Some('ب'),
                'ٹ' | 'ٺ' | 'ټ' | 'ٽ' => Some('ت'),
                'ٿ' => Some('ث'),

                // Jeem / Ha / Kha Variants
                'ڃ' | 'ڄ' | 'چ' | 'ڇ'  => Some('ج'),
                'څ' | 'ځ' | 'ڂ' => Some('خ'),

                // Dal / Thal Variants
                'ڈ' | 'ډ' | 'ڊ' | 'ڋ' | 'ڌ' | 'ڍ' => Some('د'),
                'ڎ' | 'ڏ' | 'ڐ' => Some('ذ'),

                // Ra / Zay Variants
                'ڑ' | 'ڒ' | 'ړ' | 'ڔ' | 'ڕ' | 'ږ' => Some('ر'),
                'ڗ' | 'ژ' | 'ڙ' => Some('ز'),

                // Seen / Sheen Variants
                'ښ' | 'ڛ' => Some('س'),
                'ڜ' => Some('ش'),

                // Sad / Dad / Taa / Zaa Variants
                'ڝ' => Some('ص'),
                'ڞ' => Some('ض'),
                'ڟ' => Some('ط'),

                // Ain / Ghain Variants
                'ڠ' => Some('غ'),

                // Fa / Qaf Variants
                'ڡ' | 'ڢ' | 'ڣ' | 'ڤ' | 'ڥ' | 'ڦ' => Some('ف'),
                'ڧ' | 'ڨ' => Some('ق'),

                // Kaf / Gaf Variants
                'ک' | 'ڪ' | 'ګ' | 'ڬ' | 'ڭ' | 'ڮ' | 'ػ' | 'گ' | 'ڰ' | 'ڱ' | 'ڲ' | 'ڳ' | 'ڴ' => {
                    Some('ك')
                }

                // Lam Variants
                'ڵ' | 'ڶ' | 'ڷ' | 'ڸ' => Some('ل'),

                // Noon Variants
                'ڹ' | 'ں' | 'ڻ' | 'ڼ' | 'ڽ' => Some('ن'),

                // Ha (End) Variants
                'ھ' | 'ۀ' | 'ہ' | 'ۂ' | 'ۃ' | 'ە' => Some('ه'),

                // Waw Variants
                'ۄ' | 'ۅ' | 'ۆ' | 'ۇ' | 'ۈ' | 'ۉ' | 'ۊ' | 'ۋ' | 'ۏ' => Some('و'),

                // Ya Variants
                'ی' | 'ۍ' | 'ێ' | 'ې' | 'ۑ' | 'ے' | 'ۓ' => Some('ي'),

                // Special Hamza Carriers
                'ٶ' => Some('ؤ'),
                'ٸ' => Some('ئ'),

                _ => None,
            };

            let parent_arc = parent_char.and_then(
                    |p| real_map.get(&p).cloned()
                );
            fake_letters.push(Letter::new(c, parent_arc));
        }
    }

    // for Special Characters :)
    let presentation_ranges = [(0xFB50, 0xFDFF), (0xFE70, 0xFEFF)];
    for (start, end) in presentation_ranges {
        for cp in start..=end {
            if let Some(c) = char::from_u32(cp) {
                let parent_char = match c {
                    // Alif variants
                    'ﺂ' | 'ﺄ' | 'ﺎ' | 'ﺃ' | 'ﺇ' | 'ﺍ' | 'ٱ' => Some('ا'),
                    
                    // Ba / Ta / Tha / Noon / Ya
                    'ﺐ' | 'ﺒ' | 'ﺏ' | 'ﺑ' => Some('ب'),
                    'ﺖ' | 'ﺘ' | 'ﺕ' | 'ﺗ' => Some('ت'),
                    'ﺚ' | 'ﺜ' | 'ﺙ' | 'ﺛ' => Some('ث'),
                    'ﻦ' | 'ﻨ' | 'ﻥ' | 'ﻧ'| 'ﮞ' => Some('ن'),
                    'ﻲ' | 'ﻰ' | 'ﻴ' | 'ﺌ' | 'ﻳ' | 'ﻱ' => Some('ي'),
                    'ﺔ' | 'ﺓ' => Some('ة'),
                    
                    // Jeem / Ha / Kha
                    'ﺝ' | 'ﺠ' | 'ﺞ' | 'ﺟ' => Some('ج'),
                    'ﺣ' | 'ﺤ' | 'ﺢ' | 'ﺡ' => Some('ح'),
                    'ﺥ' | 'ﺨ' | 'ﺦ' | 'ﺧ' => Some('خ'),

                    // Dal / Thal
                    'ﺩ' | 'ﺪ' | 'ﺫ' | 'ﺬ' => Some('د'), 

                    // Ra / Zay
                    'ﺭ' | 'ﺮ' | 'ﺯ' | 'ﺰ' => Some('ر'),

                    // Seen / Sheen
                    'ﺱ' | 'ﺴ' | 'ﺲ' | 'ﺳ' => Some('س'),
                    'ﺵ' | 'ﺸ' | 'ﺶ' | 'ﺷ' => Some('ش'),

                    // Sad / Dad
                    'ﺹ' | 'ﺼ' | 'ﺺ' | 'ﺻ' => Some('ص'),
                    'ﺽ' | 'ﻀ' | 'ﺾ' | 'ﺿ' => Some('ض'),

                    // Taa / Zaa
                    'ﻂ' | 'ﻄ' | 'ﻃ' | 'ﻁ' => Some('ط'),
                    'ﻇ' | 'ﻈ' | 'ﻆ' | 'ﻅ' => Some('ظ'),
                    
                    // Ain / Ghain
                    'ﻋ' | 'ﻌ' | 'ﻊ' | 'ﻉ' => Some('ع'),
                    'ﻏ' | 'ﻐ' | 'ﻎ' | 'ﻍ' => Some('غ'),
                    
                    // Fa / Qaf
                    'ﻑ' | 'ﻔ' | 'ﻒ' | 'ﻓ' => Some('ف'),
                    'ﻕ' | 'ﻘ' | 'ﻖ' | 'ﻗ' => Some('ق'),

                    // Kaf / Gaf / Lam
                    'ﻚ' | 'ﻜ' | 'ﻙ' | 'ﻛ' | 'ﯖ' | 'ﮝ' | 'ﮏ' | 'ﮐ' => Some('ك'),
                    'ڵ' | 'ﻝ' | 'ﻠ' | 'ﻟ' | 'ﻞ' => Some('ل'),

                    // Meem
                    'ﻣ' | 'ﻤ' | 'ﻢ' | 'ﻡ' => Some('م'),

                    // Ha
                    'ﻫ' | 'ﻬ' | 'ﻪ' | 'ﻩ' | 'ﮤ' | 'ﮦ' => Some('ه'),

                    // Waw
                    'ﻭ' | 'ﻮ' | 'ﯜ' | 'ﯛ' => Some('و'),

                    _ => None,
                };

                if let Some(p) = parent_char {
                    let parent_arc = real_map.get(&p).cloned();
                    fake_letters.push(Letter::new(c, parent_arc));
                }
            }
        }
    }

    // FIXME: should be deleted BTW
    // Special Characters also :)
    fake_letters.push(Letter::new('£', real_map.get(&'ك').cloned()));
    fake_letters.push(Letter::new('؏', real_map.get(&'ع').cloned()));
    // fake_letters.push(Letter::new('ﻶ', real_map.get(&'ل').cloned()));
    // fake_letters.push(Letter::new('ﻋ', real_map.get(&'ع').cloned()));
    // fake_letters.push(Letter::new('ﺣ', real_map.get(&'ح').cloned()));
    // fake_letters.push(Letter::new('ﻲ', real_map.get(&'ي').cloned()));
    // fake_letters.push(Letter::new('ﻼ', real_map.get(&'ﻻ').cloned()));
    // fake_letters.push(Letter::new('ﺔ', real_map.get(&'ة').cloned()));
    // fake_letters.push(Letter::new('ﻦ', real_map.get(&'ن').cloned()));
    // fake_letters.push(Letter::new('ﺂ', real_map.get(&'ا').cloned()));

    let mut real_vec: Vec<Arc<Letter>> = real_map.into_values().collect();
    real_vec.sort_by_key(|l| l.character);

    [real_vec, fake_letters]
}

pub fn clear_message(letters: &[RefLetter], message: String) -> Option<String> {
    if message.is_empty() {
        return None;
    }

    // Efficient String
    let mut cleared_string = String::with_capacity(message.len() + 20);

    for c in message.chars() {
        match c {
            // Some workaround for Alif
            'آ' | '|' => {
                cleared_string.push('ا');
                continue;
            },
            '(' | ')' | '{' | '}' | '[' | 
            ']' | '<' | '>' | '∆' |
            // '♭' | '♡' | '☜' | '☞' |
            '\u{1F300}'..='\u{1FAFF}'   |   // Most of emojies
            '\u{1F1E6}'..='\u{1F1FF}'   |   // Flags (Regional Indicators 🇦-🇿)
            '\u{25A0}'..='\u{27BF}'         // Dingbats (✅, ✉️, ✂️)
            => {
                cleared_string.push(' ');
                continue;
            },
            // HANDLE MACRO-SYMBOLS (Like the Bismillah)
            '\u{FDFD}' => {
                // I can just skip but i want to rewrite it..
                cleared_string.push_str("بسم الله الرحمن الرحيم");
                continue;
            },
            'ﻼ' | 'ﻹ' | 'ﻷ' | 'ﻶ' => {
                cleared_string.push_str("لا");
                continue;
            },
            'טּ' => {
                cleared_string.push('ن');
                continue;
            }
            '𝔖' => {
                cleared_string.push('س');
                continue;
            },
            'ℋ' => {
                cleared_string.push('ح');
                continue;
            },
            'ℳ' => {
                cleared_string.push('م');
                continue;
            }
            _ => ()
        }

        // Skip Unneeded Characters
        if is_removable(c) {
            continue
        }

        let replacement = if let Some(found_letter) = letters.iter().find(|l| l.character == c) {
            match &found_letter.alternative {
                Some(parent) => parent.character,
                None => found_letter.character,
            }
        } else {
            // keep it as is, if not in letters list
            c
        };

        cleared_string.push(replacement);
    }

    Some(cleared_string)
}

fn is_removable(c: char) -> bool {
    matches!(c, 
        // 1. Tashkeel (Diacritics)
        '\u{064B}'..='\u{0652}' | 
        '\u{0653}'..='\u{065F}' | 
        '\u{0670}'              |
        '\u{06DC}'              |

        // 2. U+200C (ZWNJ), U+200D (ZWJ), 
        // 3. U+200E/F (Direction marks), 
        // 4. U+FEFF (BOM)
        '\u{2000}'..='\u{200F}' |
        '\u{FEFF}'              |
        '\u{061C}'              |
        // 5. Small high marks/decorations
        '\u{06D6}'..='\u{06ED}' |
        '\u{08D3}'..='\u{08FF}' |
        '\u{0300}'..='\u{036F}' |
        // 6. for emojis hidden colorizer
        '\u{FE00}'..='\u{FE0F}' |
        // 7. Tatweel (Kashida)
        'ـ'                     | 
        // 8. Arabic Punctuation
        '،' | // Arabic Comma (U+060C)
        '؛' | // Arabic Semicolon (U+061B)
        '؟'   // Arabic Question Mark (U+061F)
    ) || c.is_ascii_punctuation() // 4. Standard Punctuation (., ! : etc.)
      || c == '»'
}
