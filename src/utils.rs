use hugr::HugrView;
use urlencoding;
use webbrowser;

// Adapted from tket2/src/utils.rs
pub fn viz_hugr(hugr: &impl HugrView) {
    let mut base: String = "https://dreampuf.github.io/GraphvizOnline/#".into();
    base.push_str(&urlencoding::encode(hugr.dot_string().as_ref()));
    webbrowser::open(&base).unwrap();
}