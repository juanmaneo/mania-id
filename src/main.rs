use std::collections::HashMap;
use more_asserts::debug_assert_ge;
use more_asserts::debug_assert_le;
use nanoid::nanoid;

#[macro_export]
macro_rules! string_to_fixed_char_array_alphabet {
    ($string:expr) => {
        $string.chars().collect::<Vec<_>>().try_into().expect("Wrong alphabet")
    };
}

fn utf8_character_at_index_as_string(potentially_utf8: &str,
                                     index: usize) -> String {
    let size_in_char = potentially_utf8.len();
    let start_char_index = index;
    let end_char_index = index + 1;
    let mut indices = potentially_utf8.char_indices().map(|(i, _)| i);
    let start_index = indices.nth(start_char_index).unwrap_or(size_in_char);
    let end_index = indices.nth(end_char_index - start_char_index - 1).unwrap_or(size_in_char);
    let utf8_code_point_as_string = potentially_utf8[start_index..end_index].to_string();
    return utf8_code_point_as_string;
}

//TODO remove once nanoid as been rewritten only useful for debug
fn utf8_char_to_int(char_as_str: &str) -> u64 {
    let nb_u8 = char_as_str.len();//from 1 to 4
    debug_assert_ne!(nb_u8, 0);
    debug_assert_ge!(nb_u8, 1);
    debug_assert_le!(nb_u8, 4);

    let c0 = char_as_str.as_bytes()[0] as u64;
    let mut as_int = c0;
    if as_int == 2 {
        let c1 = char_as_str.as_bytes()[1] as u64 * 1000;
        as_int += c1;
    }
    if as_int == 3 {
        let c2 = char_as_str.as_bytes()[2] as u64 * 1000 * 1000;
        as_int += c2;
    }
    if as_int == 4 {
        let c3 = char_as_str.as_bytes()[3] as u64 * 1000 * 1000 * 1000;
        as_int += c3;
    }

    return as_int;
}

fn utf8_replace_at_index(replaced: &mut String,
                         replace_with: &str,
                         index: usize) {
    replaced.replace_range(
        replaced
            .char_indices()
            .nth(index)
            .map(|(pos, ch)| pos..pos + ch.len_utf8())
            .unwrap(),
        replace_with,
    );
}

fn fruityfy(alphabet_to_replace_with_potentially_utf8: &str,
            alphabet_replacement_with_potentially_utf8: &str,
            size_of_alphabet: usize,
            where_to_replace: &str) -> String {
    debug_assert_ge!(alphabet_to_replace_with_potentially_utf8.len(), size_of_alphabet);
    debug_assert_ge!(alphabet_replacement_with_potentially_utf8.len(), size_of_alphabet);
    debug_assert_eq!(alphabet_to_replace_with_potentially_utf8.chars().count(), size_of_alphabet);
    debug_assert_eq!(alphabet_replacement_with_potentially_utf8.chars().count(), size_of_alphabet);

    // first create map between alphabet
    let mut replacement_map = HashMap::new();
    for i in 0..size_of_alphabet {
        let to_be_replaced = utf8_character_at_index_as_string(alphabet_to_replace_with_potentially_utf8, i);
        let replace_with = utf8_character_at_index_as_string(alphabet_replacement_with_potentially_utf8, i);
        replacement_map.insert(utf8_char_to_int(to_be_replaced.as_str()), replace_with);
    }
    // then replace all character with UTF-8
    let mut replaced: String = where_to_replace.to_string();
    let len_replaced = replaced.chars().count();
    for i in 0..len_replaced {
        let to_be_replaced = utf8_character_at_index_as_string(where_to_replace, i);
        let key_to_find = utf8_char_to_int(to_be_replaced.as_str());
        let replace_with = replacement_map.get(&key_to_find).expect("Map should be done already...");
        utf8_replace_at_index(&mut replaced, replace_with.as_str(), i);
    }
    return replaced;
}

fn generate_id_with_utf8_emojis<
    const LEN_ALPHABET: usize,
    const NUMBER_OF_SYMBOLS_PER_PART: usize,
    const NUMBER_OF_PARTS: u16>(alphabet_only_ascii: &str,
                                alphabet_with_maybe_utf8_emojis: &str) -> String {
    debug_assert_eq!(alphabet_only_ascii.chars().count(), LEN_ALPHABET);
    debug_assert_eq!(alphabet_only_ascii.len(), LEN_ALPHABET);
    debug_assert!(alphabet_only_ascii.is_ascii());
    debug_assert_eq!(alphabet_with_maybe_utf8_emojis.chars().count(), LEN_ALPHABET);

    let ascii_alphabet_array: [char; LEN_ALPHABET] = string_to_fixed_char_array_alphabet!(alphabet_only_ascii);

    let mut generated: String = "".to_owned();
    for _ in 0..NUMBER_OF_PARTS {
        let id_part = nanoid!(NUMBER_OF_SYMBOLS_PER_PART, &ascii_alphabet_array);
        generated.push_str(id_part.as_str());
    }
    let converted: String = fruityfy(alphabet_only_ascii,
                                     alphabet_with_maybe_utf8_emojis,
                                     LEN_ALPHABET,
                                     generated.as_str());
    return converted;
}

const FRUITY_ALPHABET_AS_UTF8_STRING: &str = "🥭🍇🍈🍉🍊🍋🍌🍍🍎🍏🍐🍑🍒🍓🥝🥥";
const HEXA_ALPHABET_AS_POTENTIALLY_UTF8_STRING: &str = "0123456789abcdef";
const LEN_HEXA: usize = 16;


fn generate_fruity_id() -> String {
    const NUMBER_OF_SYMBOLS_PER_PART: usize = 17;
    const NUMBER_OF_PARTS: u16 = 2;

    return generate_id_with_utf8_emojis::<LEN_HEXA, NUMBER_OF_SYMBOLS_PER_PART, NUMBER_OF_PARTS>(
        HEXA_ALPHABET_AS_POTENTIALLY_UTF8_STRING,
        FRUITY_ALPHABET_AS_UTF8_STRING);
}

const MANIA_ALPHABET_AS_UTF8_STRING: &str = "🤪🤬🤣🤗🤩🤔🤮🤐😤🤫🤧🥳🥵🥶🙊💩\
👏🙏🤞🤝👋👊🖖\
🥭🍇🍈🍉🍊🍋🍌🍍🍎🍏🍐🍑🍒🍓🥝🥥\
🥯🥐🥖🥨🧀🥩🍔🍟🌭🥪🌮🥗🍱🍙🍝🍩🍪🧁🎂🍫🍬🍭🍺🍷🍹🥃🥤\
🦊🦀🐳🦄🐴🐧🦎🐇🐙🦏\
🎷🎸🎺🎻\
🚅🚌🚒🚕🚙🚛🚁🚠🚀🔋📘💼📌📏📐";

const MANIA_ALPHABET_ASCII_STRING: &str =" !\"#$%&'()*+,-./0123456789:;<=>?@\
ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
const LEN_MANIA: usize = 95;

fn generate_mania_id() -> String {
    const NANOID_UUID_LIKE_COLLISION_NUMBER_OF_SYMBOLS_PER_PART: usize = 21;
    const NUMBER_OF_PARTS: u16 = 3;

    return generate_id_with_utf8_emojis::<LEN_MANIA, NANOID_UUID_LIKE_COLLISION_NUMBER_OF_SYMBOLS_PER_PART, NUMBER_OF_PARTS>(
        MANIA_ALPHABET_ASCII_STRING,
        MANIA_ALPHABET_AS_UTF8_STRING);
}

fn main() {
    let fid = generate_fruity_id();

    println!("fruity-ID sample: '{}'", fid);

    let mid = generate_mania_id();
    println!("mania-ID sample: '{}'", mid);
}

#[cfg(test)]
mod test_fruityfy {
    use super::*;

    #[test]
    fn simple() {
        const INPUT_BEFORE: &str = "ab12cd345fff";
        let expected_output: String = "🍐🍑🍇🍈🍒🍓🍉🍊🍋🥥🥥🥥".to_string();

        let converted: String = fruityfy(HEXA_ALPHABET_AS_POTENTIALLY_UTF8_STRING,
                                         FRUITY_ALPHABET_AS_UTF8_STRING,
                                         LEN_HEXA,
                                         INPUT_BEFORE);

        assert_eq!(converted.as_str(), expected_output);// output is OK
    }
    #[test]
    fn all_fruits() {
        let input_before: String = HEXA_ALPHABET_AS_POTENTIALLY_UTF8_STRING.to_string();
        let expected_output: String = FRUITY_ALPHABET_AS_UTF8_STRING.to_string();

        let converted: String = fruityfy(HEXA_ALPHABET_AS_POTENTIALLY_UTF8_STRING,
                                         FRUITY_ALPHABET_AS_UTF8_STRING,
                                         LEN_HEXA,
                                         input_before.as_str());

        assert_eq!(input_before.as_str(), HEXA_ALPHABET_AS_POTENTIALLY_UTF8_STRING);// input did not change
        assert_eq!(converted, expected_output);// output is OK
    }


}
