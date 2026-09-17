/// Flutter version information obtained from `flutter --version --machine`.
#[derive(Debug, Clone)]
pub struct FlutterVersion {
    pub flutter_version: Option<String>,
}

impl FlutterVersion {
    /// Check if the Flutter version is greater than or equal to `expected`.
    pub fn is_greater_or_equal(&self, expected: &str) -> bool {
        let Some(current) = self.flutter_version.as_ref() else {
            return false;
        };

        let current = current.split('-').next().unwrap_or(current);
        compare_semver_like(current, expected).is_some_and(|ord| ord >= 0)
    }
}

fn compare_semver_like(current: &str, expected: &str) -> Option<i8> {
    let parse = |value: &str| -> Option<Vec<u64>> {
        let mut out = Vec::new();
        for segment in value.split('.') {
            out.push(segment.parse::<u64>().ok()?);
        }
        Some(out)
    };

    let mut left = parse(current)?;
    let mut right = parse(expected)?;
    let max_len = left.len().max(right.len());
    left.resize(max_len, 0);
    right.resize(max_len, 0);

    for (l, r) in left.iter().zip(right.iter()) {
        if l > r {
            return Some(1);
        }
        if l < r {
            return Some(-1);
        }
    }
    Some(0)
}
