use fastforge_app_analyzer::*;
use fastforge_core::{AnalyzeConfig, AppAnalyzer};

fn main() {
    println!("app_analyzer is under development. Stay tuned.");
    let analyzer = AndroidApkAnalyzer::new();
    let config = AnalyzeConfig::new(String::from("path/to/app"));
    let result = analyzer.analyze(config);
    println!("Analysis result: {:?}", result);
}
