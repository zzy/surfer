use unicode_segmentation::UnicodeSegmentation;
use pinyin::ToPinyin;

fn main() {
    let subject = "wHo哈哈，，，哈neW好的 eGg,i aM     中国人， Say chINese"
        .to_lowercase();
    println!("{}", subject);

    let mut subject_seg: Vec<&str> = subject.unicode_words().collect();
    println!("{:?}", &subject_seg);

    for n in 0..subject_seg.len() {
        let seg = subject_seg[n];
        if !seg.is_ascii() {
            let seg_py =
                seg.chars().next().unwrap().to_pinyin().unwrap().plain();
            subject_seg[n] = seg_py;
        }
    }

    let slug = &subject_seg.join("-");
    println!("{}", &slug);
}
