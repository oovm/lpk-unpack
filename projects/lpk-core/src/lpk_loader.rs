use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{LpkConfig, MLveConfig};
use tracing::{debug, info, warn};
use zip::ZipArchive;

mod extractors;

use crate::{
    configs::Live2dConfig,
    errors::{LpkError, Result},
    helpers::{decrypt, hashed_filename, is_encrypted_file, make_key, safe_mkdir},
    LpkError::DecodeError,
};

/// LPK文件加载器，负责解析和解压LPK文件
pub struct LpkLoader {
    /// LPK文件路径
    lpk_path: PathBuf,
    /// 配置文件路径（用于Steam Workshop LPK）
    config_path: PathBuf,
    /// LPK类型
    lpk_type: String,
    /// 是否加密
    encrypted: bool,
    /// 文件名转换映射
    uncompressed: HashMap<String, String>,
    /// 条目映射
    entrys: HashMap<String, String>,
    /// 用户配置（用于Steam Workshop LPK）
    config: LpkConfig,
    /// MLVE配置
    mlve_config: MLveConfig,
}

impl LpkLoader {
    pub fn open(lpk_path: &Path) -> Result<Self> {
        let config = lpk_path.with_file_name("config").with_extension("json");
        LpkLoader::open_with_config(lpk_path, &config)
    }

    /// 创建新的LPK加载器
    pub fn open_with_config(lpk_path: &Path, config_path: &Path) -> Result<Self> {
        let mut loader = LpkLoader {
            lpk_path: lpk_path.to_path_buf(),
            config_path: config_path.to_path_buf(),
            lpk_type: String::new(),
            encrypted: true,
            uncompressed: HashMap::new(),
            entrys: HashMap::new(),
            mlve_config: MLveConfig::default(),
            config: LpkConfig::default(),
        };

        loader.load_lpk()?;
        Ok(loader)
    }

    /// 加载LPK文件
    fn load_lpk(&mut self) -> Result<()> {
        let file = File::open(&self.lpk_path)?;
        let mut archive = ZipArchive::new(file)?;

        // 尝试读取config.mlve文件
        let config_mlve_raw = match archive.by_name(&hashed_filename("config.mlve")) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                contents
            }
            Err(_) => String::new(),
        };

        let config_mlve_raw = if config_mlve_raw.is_empty() {
            // 尝试直接读取未加密的config.mlve
            match archive.by_name("config.mlve") {
                Ok(mut file) => {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    contents
                }
                Err(_) => return Err(LpkError::ConfigMissing),
            }
        }
        else {
            config_mlve_raw
        };

        self.mlve_config = serde_json::from_str(&config_mlve_raw)?;
        debug!("mlve config: {:#?}", self.mlve_config);

        // 获取LPK类型
        self.lpk_type = self.mlve_config.r#type.to_string();
        // 只有 Steam Workshop LPK 需要 config.json来解密
        if self.lpk_type == "STM_1_0" {
            self.load_config()?;
        }
        self.encrypted = self.mlve_config.encrypt == "encrypt";
        Ok(())
    }

    /// 加载配置文件（用于Steam Workshop LPK）
    fn load_config(&mut self) -> Result<()> {
        let mut file = File::open(&self.config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.config = serde_json::from_str(&contents)?;
        Ok(())
    }

    /// 解压LPK文件到指定目录
    pub fn extract(&mut self, output_dir: &Path) -> Result<()> {
        safe_mkdir(output_dir)?;
        match self.lpk_type.as_str() {
            "STD2_0" => self.extract_standard(output_dir),
            "STM_1_0" => self.extract_standard(output_dir),
            _ => self.extract_legacy(output_dir),
        }
    }

    /// 解压服装
    fn extract_costume(&mut self, model_json: &str, dir: &Path) -> Result<()> {
        if model_json.is_empty() {
            return Ok(());
        }

        // 检查解密
        self.check_decrypt(model_json)?;

        // 解压模型JSON
        self.extract_model_json(model_json, dir)
    }

    /// 解密数据
    fn decrypt_data(&self, filename: &str, data: &[u8]) -> Result<Vec<u8>> {
        match self.lpk_type.as_ref() {
            // 标准 LPK 直接使用文件名作为密钥
            "STD_1_0" | "STD2_0" => {
                let key = format!("{}{filename}", self.mlve_config.id);
                debug!("Standalone Key: {}", key);
                Ok(decrypt(make_key(&key), data))
            }
            "STM_1_0" if self.mlve_config.encrypt == "false" => Ok(decrypt(0, data)),
            // Steam Workshop LPK 需要读取 config.json 作为密钥
            "STM_1_0" => {
                let key = format!("{}{}{filename}{}", self.mlve_config.id, self.config.file_id, self.config.meta_data);
                debug!("Steam Key: {}", key);
                Ok(decrypt(make_key(&key), data))
            }
            _ => Err(LpkError::DecodeError { format: self.lpk_type.to_string(), message: "unimplement".to_string() }),
        }
    }

    /// 检查文件是否需要解密，并处理加密文件
    fn check_decrypt(&mut self, model_json: &str) -> Result<()> {
        // 如果不是加密的或者已经处理过，直接返回
        if !self.encrypted || self.uncompressed.contains_key(model_json) {
            return Ok(());
        }

        // 如果是加密文件名，需要先解密
        if is_encrypted_file(model_json) {
            let file = File::open(&self.lpk_path)?;
            let mut archive = ZipArchive::new(file)?;

            // 尝试直接打开文件
            let file = match archive.by_name(model_json) {
                Ok(_) => {
                    // 文件存在，不需要额外处理
                    Ok(())
                }
                Err(_) => {
                    // 文件不存在，可能需要进一步处理
                    Err(LpkError::DecryptionFailed(format!("File not found: {}", model_json)))
                }
            };
            file
        }
        else {
            // 非加密文件名，不需要处理
            Ok(())
        }
    }

    /// 递归处理模型中的所有引用
    fn process_model_references(&mut self, model: &Live2dConfig, dir: &Path, _id: usize) -> Result<()> {
        if !model.model.is_empty() && is_encrypted_file(&model.model) {
            debug!("extracting model: {}", model.model);

            // 解密纹理文件
            let file = File::open(&self.lpk_path)?;
            let mut archive = ZipArchive::new(file)?;
            let mut file = match archive.by_name(&model.model) {
                Ok(file) => file,
                Err(e) => Err(DecodeError { format: "moc3".to_string(), message: e.to_string() })?,
            };

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let decrypted_data = self.decrypt_data(&model.model, &buffer)?;

            // 保存解密后的纹理文件
            let moc = format!("{}.moc3", "character");
            let output_file = dir.join(&moc);

            let mut file = File::create(output_file)?;
            file.write_all(&decrypted_data)?;

            // 更新引用映射
            self.uncompressed.insert(model.model.to_string(), moc);
        }

        // 处理模型中的纹理引用
        for texture in model.textures.as_slice() {
            let path = texture;
            if !path.is_empty() && is_encrypted_file(path) {
                debug!("extracting texture: {}", path);

                // 解密纹理文件
                let file = File::open(&self.lpk_path)?;
                let mut archive = ZipArchive::new(file)?;
                let mut file = match archive.by_name(path) {
                    Ok(file) => file,
                    Err(_) => continue, // 跳过不存在的文件
                };

                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;

                let decrypted_data = self.decrypt_data(path, &buffer)?;

                // 保存解密后的纹理文件
                let texture_id = self.entrys.len();
                let texture_name = format!("texture{}.png", texture_id);
                let output_file = dir.join(&texture_name);

                let mut file = File::create(output_file)?;
                file.write_all(&decrypted_data)?;

                // 更新引用映射
                self.uncompressed.insert(path.to_string(), texture_name);
            }
        }
        // // 处理模型中的其他模型引用
        // if let Some(models) = model.get("models").and_then(|v| v.as_array()) {
        //     for model_ref in models {
        //         if let Some(path) = model_ref.get("path").and_then(|v| v.as_str()) {
        //             if !path.is_empty() && !self.uncompressed.contains_key(path) {
        //                 debug!("found model reference: {}", path);
        //                 self.extract_model_json(path, dir)?;
        //             }
        //         }
        //     }
        // }

        // 处理模型中的动画引用
        // if let Some(animations) = model.get("animations").and_then(|v| v.as_array()) {
        //     for animation in animations {
        //         if let Some(path) = animation.get("path").and_then(|v| v.as_str()) {
        //             if !path.is_empty() && is_encrypted_file(path) {
        //                 debug!("extracting animation: {}", path);
        //
        //                 // 解密动画文件
        //                 let file = File::open(&self.lpk_path)?;
        //                 let mut archive = ZipArchive::new(file)?;
        //                 let mut file = match archive.by_name(path) {
        //                     Ok(file) => file,
        //                     Err(_) => continue, // 跳过不存在的文件
        //                 };
        //
        //                 let mut buffer = Vec::new();
        //                 file.read_to_end(&mut buffer)?;
        //
        //                 let decrypted_data = self.decrypt_data(path, &buffer)?;
        //
        //                 // 保存解密后的动画文件
        //                 let animation_id = self.entrys.len();
        //                 let animation_name = format!("animation{}.json", animation_id);
        //
        //                 // 尝试解析为JSON
        //                 match String::from_utf8(decrypted_data.clone()) {
        //                     Ok(json_str) => {
        //                         if let Ok(json_value) = serde_json::from_str::<Value>(&json_str) {
        //                             let out_s = serde_json::to_string(&json_value)?;
        //                             self.entrys.insert(animation_name.clone(), out_s);
        //                             self.uncompressed.insert(path.to_string(), animation_name);
        //                         }
        //                     }
        //                     Err(_) => {
        //                         // 如果不是JSON，作为二进制文件处理
        //                         let output_file = dir.join(format!("animation{}.bin", animation_id));
        //                         let mut file = File::create(output_file)?;
        //                         file.write_all(&decrypted_data)?;
        //                         self.uncompressed.insert(path.to_string(), format!("animation{}.bin", animation_id));
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        Ok(())
    }

    /// 解压模型JSON文件
    fn extract_model_json(&mut self, model_json: &str, dir: &Path) -> Result<()> {
        debug!("========= extracting model {} =========", model_json);
        // 解密模型JSON文件
        let file = File::open(&self.lpk_path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut file = archive.by_name(model_json)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        for character in self.mlve_config.list.as_slice() {
            debug!("Export character `{}`", character.character);
            for costume in character.costume.as_slice() {
                debug!("Export costume `{}({})`", costume.name, character.character);
                let decrypted_data = self.decrypt_data(&costume.path, &buffer)?;
                let json_str = String::from_utf8(decrypted_data).unwrap();
                let output = format!("{}-{}.model3.json", character.character, costume.name);
                let path = dir.join(output);
                let mut file = File::create(path)?;
                file.write_all(json_str.as_bytes())?;
            }
        }
        Ok(())
    }
}
