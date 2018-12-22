use unicode_segmentation::UnicodeSegmentation;

mod combining;

pub struct WcWidthOptions {
    /**
     * size of nul character
     */
    pub nul: i32,
    /**
     * size of control character
     */
    pub control: i32,
}

const DEFAULTS: WcWidthOptions = WcWidthOptions { nul: 0, control: 0 };

pub fn wcswidth(s: &str, opts: Option<WcWidthOptions>) -> i32 {
    let settings = opts.unwrap_or(DEFAULTS);
    let g = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    let mut s = 0;
    for i in 0..g.len() {
        let chars: Vec<char> = g[i].chars().collect();
        let mut c: i32 = 0;
        for x in 0..chars.len() {
            c += chars[x] as i32;
        }
        let n = wcwidth(c, &settings);
        if n < 0 {
            return -1;
        }
        s += n;
    }
    return s;
}

pub fn wcwidth(ucs: i32, opts: &WcWidthOptions) -> i32 {
    // Test for 8-bit control characters
    if ucs == 0 {
        return opts.nul;
    }
    if ucs < 32 || (ucs >= 0x7f && ucs < 0xa0) {
        return opts.control;
    }

    // Binary search in table of non-spacing characters
    if bisearch(ucs) {
        return 0;
    }

    return 1
        + (ucs >= 0x1100
            && (ucs <= 0x115f || // Hangul Jamo init. consonants
        ucs == 0x2329 ||
        ucs == 0x232a ||
        (ucs >= 0x2e80 && ucs <= 0xa4cf && ucs != 0x303f) || // CJK ... Yi
        (ucs >= 0xac00 && ucs <= 0xd7a3) || // Hangul Syllables
        (ucs >= 0xf900 && ucs <= 0xfaff) || // CJK Compatibility Ideographs
        (ucs >= 0xfe10 && ucs <= 0xfe19) || // Vertical forms
        (ucs >= 0xfe30 && ucs <= 0xfe6f) || // CJK Compatibility Forms
        (ucs >= 0xff00 && ucs <= 0xff60) || // Fullwidth Forms
            (ucs >= 0xffe0 && ucs <= 0xffe6) ||
            (ucs >= 0x20000 && ucs <= 0x2fffd) ||
            (ucs >= 0x30000 && ucs <= 0x3fffd))) as i32;
}

fn bisearch(ucs: i32) -> bool {
    let mut min: i32 = 0;
    let mut max = combining::COMBINING.len() - 1;
    let mut mid: i32;

    if ucs < combining::COMBINING[0][0] || ucs > combining::COMBINING[max][1] {
        return false;
    }

    while max >= min as usize {
        // mid = Math.floor((min + max) / 2);
        mid = (min + max as i32) / 2 as i32;
        if ucs > combining::COMBINING[mid as usize][1] {
            min = mid + 1;
        } else if ucs < combining::COMBINING[mid as usize][0] {
            max = mid as usize - 1;
        } else {
            return true;
        }
    }

    return false;
}
