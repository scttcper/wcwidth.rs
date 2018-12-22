#[cfg(test)]
mod test {
    fn null_char() -> String {
        let mut buf = String::with_capacity(1);
        let x = std::char::from_u32(0).unwrap_or_default();
        buf.push(x);
        return buf;
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn regular_strings() {
        assert_eq!(wcwidth::wcswidth("abc", None), 3);
    }

    #[test]
    fn multibyte_strings() {
        assert_eq!(wcwidth::wcswidth("字的模块", None), 8);
        assert_eq!(wcwidth::wcswidth("नमस्ते", None), 4);
    }

    #[test]
    fn multibyte_characters_with_regular_characters() {
        assert_eq!(wcwidth::wcswidth("abc 字的模块", None), 12);
    }

    #[test]
    fn ignore_control_characters() {
        assert_eq!(wcwidth::wcswidth("abc\n字的模块\ndef", None), 14);
    }

    #[test]
    fn ignore_bad_input() {
        assert_eq!(wcwidth::wcswidth("", None), 0);
    }

    #[test]
    fn ignore_null_charcode() {
        assert_eq!(wcwidth::wcswidth(&null_char(), None), 0);
    }

    #[test]
    fn ignore_mixed_nulls_and_chars() {
        let s = format!("a{}\n字的", null_char());
        assert_eq!(wcwidth::wcswidth(&s, None), 5);
    }

    #[test]
    fn custom_value_for_nul() {
        let s = format!("a{}\n字的", null_char());
        let opt = wcwidth::WcWidthOptions {
            nul: 10,
            control: 0,
        };
        assert_eq!(wcwidth::wcswidth(&s, Some(opt)), 15);
    }

    #[test]
    fn custom_value_for_control() {
        let opt = wcwidth::WcWidthOptions { nul: 0, control: 1 };
        assert_eq!(wcwidth::wcswidth("abc\n字的模块\ndef", Some(opt)), 16);
    }

    #[test]
    fn negative_custom_value_for_control() {
        let opt = wcwidth::WcWidthOptions {
            nul: 0,
            control: -1,
        };
        assert_eq!(wcwidth::wcswidth("abc\n字的模块\ndef", Some(opt)), -1);
    }
}
