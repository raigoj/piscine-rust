pub fn to_url(s: &str) -> String {
    str::replace(s, " ", "%20")
}