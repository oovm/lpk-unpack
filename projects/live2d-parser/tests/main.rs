use live2d_parser::{cubism_v3::moc3::Moc3, Live2DModel, Model3Json};
use std::path::Path;

#[test]
fn test_load_model_v3() -> Result<(), serde_json::Error> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let model_dir = here.join("tests").join("model_v3");
    let model_files = std::fs::read_dir(&model_dir).expect("Failed to read model_v3 directory");

    for entry in model_files {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "json") {
            println!("Loading model: {}", path.display());

            // 读取模型文件
            let model_json = std::fs::read_to_string(&path).expect("Failed to read model file");

            let _ = serde_json::from_str::<Model3Json>(&model_json)?;

            // 解析模型数据
            let model = Live2DModel::from_str(&model_json).expect("Failed to parse model");

            let model = match model {
                Live2DModel::V1(o) => {
                    panic!("Model version should be 3: {o:#?}");
                }
                Live2DModel::V3(model) => model,
            };

            // 验证基本属性
            assert_eq!(model.version, 3, "Model version should be 3");
            assert!(!model.file_references.moc.is_empty(), "Moc file reference should not be empty");
            assert!(!model.file_references.textures.is_empty(), "Textures should not be empty");

            println!("Successfully loaded and validated model: {}", path.display());
        }
    }
    Ok(())
}

#[test]
fn test_moc3() -> Result<(), serde_json::Error> {
    tracing_subscriber::fmt().pretty().init();
    let m = Moc3::new(include_bytes!("mao_pro.moc3").to_vec())?;
    // println!("MagicHead: {}", m.magic_head());
    // println!("Elements: {:#?}", m.element_count());
    for p in m.parts().take(4) {
        println!("Part: {:#?}", p);

    }
    for p in m.get_parameters().take(4) {
        println!("Parameter: {:#?}", p);
        
    }
    
    Ok(())
}
