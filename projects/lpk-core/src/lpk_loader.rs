use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{LpkConfig, MLveConfig};
use tracing::{debug, error, info, trace, warn};
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
            lpk_type: String::new(),
            encrypted: true,
            uncompressed: HashMap::new(),
            entrys: HashMap::new(),
            mlve_config: MLveConfig::default(),
            config: LpkConfig::default(),
        };
        loader.load_lpk()?;
        // 只有 Steam Workshop LPK 需要 config.json来解密
        if loader.lpk_type == "STM_1_0" {
            loader.load_config(config_path)?;
        }
        Ok(loader)
    }

    /// 加载LPK文件
    fn load_lpk(&mut self) -> Result<()> {
        let file = File::open(&self.lpk_path)?;
        let mut archive = ZipArchive::new(file)?;

        let mut contents = String::new();
        // 尝试读取 config.mlve 文件
        match archive.by_name(&hashed_filename("config.mlve")) {
            Ok(mut file) => {
                file.read_to_string(&mut contents)?;
            }
            Err(_) => {}
        };
        // 尝试直接读取未加密的 config.mlve
        if contents.is_empty() {
            match archive.by_name("config.mlve") {
                Ok(mut file) => {
                    file.read_to_string(&mut contents)?;
                }
                Err(_) => Err(LpkError::ConfigMissing)?,
            }
        }
        self.mlve_config = serde_json::from_str(&contents)?;
        debug!("mlve config: {:#?}", self.mlve_config);
        self.lpk_type = self.mlve_config.r#type.to_string();
        self.encrypted = self.mlve_config.encrypt == "encrypt";
        Ok(())
    }

    /// 加载配置文件（用于Steam Workshop LPK）
    fn load_config(&mut self, path: &Path) -> Result<()> {
        let contents = std::fs::read_to_string(path)?;
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
        self.check_decrypt(model_json)?;
        self.decrypt_model_json(model_json, dir)?;
        self.decrypt_all(dir)
    }

    /// 解密数据
    fn decrypt_data(&self, filename: &str, data: &[u8]) -> Result<Vec<u8>> {
        match self.lpk_type.as_ref() {
            // 标准 LPK 直接使用文件名作为密钥
            "STD_1_0" | "STD2_0" => {
                let key = format!("{}{filename}", self.mlve_config.id);
                trace!("Standalone Key: {}", key);
                Ok(decrypt(make_key(&key), data))
            }
            "STM_1_0" if self.mlve_config.encrypt == "false" => Ok(decrypt(0, data)),
            // Steam Workshop LPK 需要读取 config.json 作为密钥
            "STM_1_0" => {
                let key = format!("{}{}{filename}{}", self.mlve_config.id, self.config.file_id, self.config.meta_data);
                trace!("Steam Key: {}", key);
                Ok(decrypt(make_key(&key), data))
            }
            _ => Err(DecodeError { format: self.lpk_type.to_string(), message: "unimplement".to_string() }),
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

    /// 解压模型 JSON 文件
    fn decrypt_model_json(&mut self, model_json: &str, dir: &Path) -> Result<()> {
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
                let mut file = File::create(&path)?;
                file.write_all(json_str.as_bytes())?;
                debug!("Exported {}", path.canonicalize()?.display());
            }
        }
        Ok(())
    }

    fn decrypt_all(&self, output: &Path) -> Result<()> {
        let file = File::open(&self.lpk_path)?;
        let mut archive = ZipArchive::new(file)?;
        let all_files = archive.file_names().map(|s| s.to_string()).collect::<Vec<_>>();
        for file in all_files {
            let mut encrypted_file = archive.by_name(&file)?;
            let mut buffer = Vec::new();
            encrypted_file.read_to_end(&mut buffer)?;
            let decrypted_data = self.decrypt_data(&file, &buffer)?;
            let output = output.join(&file);
            match std::fs::write(&output, decrypted_data) {
                Ok(_) => {
                    debug!("Exported {}", output.canonicalize()?.display());
                }
                Err(err) => {
                    error!("Failed to write file {}: {}", output.display(), err);
                }
            }
        }
        Ok(())
    }
}
