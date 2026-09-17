/// Information parsed from a Flutter project's `pubspec.yaml`.
#[derive(Debug, Clone)]
pub struct PubspecInfo {
    pub build_name: String,
    pub build_number: String,
}
