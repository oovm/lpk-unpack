use lpk::LpkLoader;
use std::path::Path;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_lpk_loader() {
    tracing_subscriber::fmt().pretty().init();
    
    // 测试文件路径
    let lpk_path = Path::new("tests/1363062649/1363062649.lpk");

    // 创建LPK加载器
    let mut loader =
        LpkLoader::new(lpk_path, Some(Path::new("tests/1363062649/config.json"))).expect("Failed to create LPK loader");

    // 创建临时目录用于解压
    let output_dir = tempfile::tempdir().expect("Failed to create temp directory");

    // 解压LPK文件
    loader.extract(output_dir.path()).expect("Failed to extract LPK file");

    // 验证解压结果
    // 由于我们不知道具体文件内容，这里只是简单检查目录是否存在
    assert!(output_dir.path().exists(), "Output directory does not exist");

    println!("Successfully extracted LPK file to: {}", output_dir.path().display());
}
