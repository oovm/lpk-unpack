use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use log::{debug, info, warn};
use serde_json::{json, Value};
use zip::ZipArchive;

use crate::errors::{LpkError, Result};
use crate::utils::{decrypt, find_encrypted_file, genkey, get_encrypted_file, hashed_filename, is_encrypted_file, safe_mkdir};

/// LPK文件加载器，负责解析和解压LPK文件
pub struct LpkLoader {
    /// LPK文件路径
    lpk_path: PathBuf,
    /// 配置文件路径（用于Steam Workshop LPK）
    config_path: Option<PathBuf>,
    /// LPK类型
    lpk_type: Option<String>,
    /// 是否加密
    encrypted: bool,
    /// 文件名转换映射
    trans: HashMap<String, String>,
    /// 条目映射
    entrys: HashMap<String, String>,
    /// MLVE配置
    mlve_config: Value,
    /// 用户配置（用于Steam Workshop LPK）
    config: Option<Value>,
}

impl LpkLoader {
    /// 创建新的LPK加载器
    pub fn new<P: AsRef<Path>>(lpk_path: P, config_path: Option<P>) -> Result<Self> {
        let lpk_path = lpk_path.as_ref().to_path_buf();
        let config_path = config_path.map(|p| p.as_ref().to_path_buf());
        
        let mut loader = LpkLoader {
            lpk_path,
            config_path,
            lpk_type: None,
            encrypted: true,
            trans: HashMap::new(),
            entrys: HashMap::new(),
            mlve_config: json!({}),
            config: None,
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
            },
            Err(_) => {
                // 尝试直接读取未加密的config.mlve
                match archive.by_name("config.mlve") {
                    Ok(mut file) => {
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)?;
                        contents
                    },
                    Err(_) => return Err(LpkError::ConfigMissing),
                }
            }
        };
        
        self.mlve_config = serde_json::from_str(&config_mlve_raw)?;
        debug!("mlve config: {:?}", self.mlve_config);
        
        // 获取LPK类型
        if let Some(lpk_type) = self.mlve_config.get("type").and_then(|v| v.as_str()) {
            self.lpk_type = Some(lpk_type.to_string());
            
            // 只有Steam Workshop LPK需要config.json来解密
            if lpk_type == "STM_1_0" && self.config_path.is_some() {
                self.load_config()?;
            }
        }
        
        // 检查是否加密
        if let Some(encrypt) = self.mlve_config.get("encrypt").and_then(|v| v.as_str()) {
            self.encrypted = encrypt == "true";
        }
        
        Ok(())
    }
    
    /// 加载配置文件（用于Steam Workshop LPK）
    fn load_config(&mut self) -> Result<()> {
        if let Some(config_path) = &self.config_path {
            let mut file = File::open(config_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            self.config = Some(serde_json::from_str(&contents)?);
            Ok(())
        } else {
            Err(LpkError::ConfigMissing)
        }
    }
    
    /// 解压LPK文件到指定目录
    pub fn extract<P: AsRef<Path>>(&mut self, output_dir: P) -> Result<()> {
        let output_dir = output_dir.as_ref();
        
        // 确保输出目录存在
        safe_mkdir(output_dir)?;
        
        match self.lpk_type.as_deref() {
            Some("STD2_0") | Some("STM_1_0") => self.extract_standard(output_dir),
            _ => self.extract_legacy(output_dir),
        }
    }
    
    /// 解压标准格式的LPK文件（STD2_0或STM_1_0）
    fn extract_standard<P: AsRef<Path>>(&mut self, output_dir: P) -> Result<()> {
        let output_dir = output_dir.as_ref();
        
        if let Some(list) = self.mlve_config.get("list").and_then(|v| v.as_array()) {
            for chara in list {
                // 获取角色名称
                let chara_name = match chara.get("character").and_then(|v| v.as_str()) {
                    Some("") | None => "character",
                    Some(name) => name,
                };
                
                let subdir = output_dir.join(chara_name);
                safe_mkdir(&subdir)?;
                
                // 解压角色的所有服装
                if let Some(costumes) = chara.get("costume").and_then(|v| v.as_array()) {
                    for (i, costume) in costumes.iter().enumerate() {
                        info!("extracting {}_costume_{}", chara_name, i);
                        if let Some(path) = costume.get("path").and_then(|v| v.as_str()) {
                            if !path.is_empty() {
                                self.extract_costume(path, &subdir)?;
                            }
                        }
                    }
                }
                
                // 替换加密文件名为解密后的文件名
                for (name, content) in &self.entrys {
                    let mut out_s = content.clone();
                    for (k, v) in &self.trans {
                        out_s = out_s.replace(k, v);
                    }
                    
                    let output_file = subdir.join(name);
                    let mut file = File::create(output_file)?;
                    file.write_all(out_s.as_bytes())?;
                }
            }
        }
        
        Ok(())
    }
    
    /// 解压旧版格式的LPK文件
    fn extract_legacy<P: AsRef<Path>>(&mut self, output_dir: P) -> Result<()> {
        let output_dir = output_dir.as_ref();
        
        warn!("Deprecated/unknown lpk format detected. Attempting with STD_1_0 format...");
        warn!("Decryption may not work for some packs, even though this script outputs all files.");
        
        // 如果不是加密的，直接解压所有文件
        if !self.encrypted {
            info!("lpk is not encrypted, extracting all files...");
            let file = File::open(&self.lpk_path)?;
            let mut archive = ZipArchive::new(file)?;
            archive.extract(output_dir)?;
            return Ok(());
        }
        
        // 处理加密文件
        let file = File::open(&self.lpk_path)?;
        let mut archive = ZipArchive::new(file)?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };
            
            // 跳过没有扩展名的文件
            if outpath.extension().is_none() {
                continue;
            }
            
            let subdir = output_dir.join(outpath.parent().unwrap_or_else(|| Path::new("")).to_path_buf());
            let output_file_path = subdir.join(outpath.file_name().unwrap());
            
            safe_mkdir(&subdir)?;
            
            // 文本文件直接解压
            let extension = outpath.extension().and_then(|s| s.to_str()).unwrap_or("");
            if extension == "json" || extension == "mlve" || extension == "txt" {
                info!("Extracting {} -> {}", outpath.display(), output_file_path.display());
                let mut outfile = File::create(&output_file_path)?;
                std::io::copy(&mut file, &mut outfile)?;
            } else {
                // 解密其他文件
                info!("Decrypting {} -> {}", outpath.display(), output_file_path.display());
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                
                let decrypted_data = self.decrypt_data(outpath.to_str().unwrap_or(""), &buffer)?;
                let mut outfile = File::create(&output_file_path)?;
                outfile.write_all(&decrypted_data)?;
            }
        }
        
        Ok(())
    }
    
    /// 解压服装
    fn extract_costume<P: AsRef<Path>>(&mut self, model_json: &str, dir: P) -> Result<()> {
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
        // 如果不是加密的，直接返回原始数据
        if !self.encrypted {
            return Ok(data.to_vec());
        }
        
        // 根据LPK类型选择不同的解密方式
        match self.lpk_type.as_deref() {
            Some("STM_1_0") => {
                // Steam Workshop LPK需要使用config.json中的密钥
                if let Some(config) = &self.config {
                    if let Some(key) = config.get("key").and_then(|v| v.as_str()) {
                        let key_int = genkey(key);
                        return Ok(decrypt(key_int, data));
                    }
                }
                Err(LpkError::DecryptionFailed("Missing key in config".to_string()))
            },
            Some("STD2_0") | _ => {
                // 标准LPK使用文件名作为密钥
                let key = genkey(filename);
                Ok(decrypt(key, data))
            }
        }
    }
    
    /// 检查文件是否需要解密，并处理加密文件
    fn check_decrypt(&mut self, model_json: &str) -> Result<()> {
        // 如果不是加密的或者已经处理过，直接返回
        if !self.encrypted || self.trans.contains_key(model_json) {
            return Ok(());
        }
        
        // 如果是加密文件名，需要先解密
        if is_encrypted_file(model_json) {
            let file = File::open(&self.lpk_path)?;
            let mut archive = ZipArchive::new(file)?;
            
            // 尝试直接打开文件
            match archive.by_name(model_json) {
                Ok(_) => {
                    // 文件存在，不需要额外处理
                    Ok(())
                },
                Err(_) => {
                    // 文件不存在，可能需要进一步处理
                    Err(LpkError::DecryptionFailed(format!("File not found: {}", model_json)))
                }
            }
        } else {
            // 非加密文件名，不需要处理
            Ok(())
        }
    }
    
    /// 递归处理模型中的所有引用
    fn process_model_references(&mut self, model: &Value, dir: &Path, id: usize) -> Result<()> {
        // 处理模型中的纹理引用
        if let Some(textures) = model.get("textures").and_then(|v| v.as_array()) {
            for texture in textures {
                if let Some(path) = texture.get("path").and_then(|v| v.as_str()) {
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
                        self.trans.insert(path.to_string(), texture_name);
                    }
                }
            }
        }
        
        // 处理模型中的其他模型引用
        if let Some(models) = model.get("models").and_then(|v| v.as_array()) {
            for model_ref in models {
                if let Some(path) = model_ref.get("path").and_then(|v| v.as_str()) {
                    if !path.is_empty() && !self.trans.contains_key(path) {
                        debug!("found model reference: {}", path);
                        self.extract_model_json(path, dir)?;
                    }
                }
            }
        }
        
        // 处理模型中的动画引用
        if let Some(animations) = model.get("animations").and_then(|v| v.as_array()) {
            for animation in animations {
                if let Some(path) = animation.get("path").and_then(|v| v.as_str()) {
                    if !path.is_empty() && is_encrypted_file(path) {
                        debug!("extracting animation: {}", path);
                        
                        // 解密动画文件
                        let file = File::open(&self.lpk_path)?;
                        let mut archive = ZipArchive::new(file)?;
                        let mut file = match archive.by_name(path) {
                            Ok(file) => file,
                            Err(_) => continue, // 跳过不存在的文件
                        };
                        
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer)?;
                        
                        let decrypted_data = self.decrypt_data(path, &buffer)?;
                        
                        // 保存解密后的动画文件
                        let animation_id = self.entrys.len();
                        let animation_name = format!("animation{}.json", animation_id);
                        
                        // 尝试解析为JSON
                        match String::from_utf8(decrypted_data) {
                            Ok(json_str) => {
                                if let Ok(json_value) = serde_json::from_str::<Value>(&json_str) {
                                    let out_s = serde_json::to_string(&json_value)?;
                                    self.entrys.insert(animation_name.clone(), out_s);
                                    self.trans.insert(path.to_string(), animation_name);
                                }
                            },
                            Err(_) => {
                                // 如果不是JSON，作为二进制文件处理
                                let output_file = dir.join(format!("animation{}.bin", animation_id));
                                let mut file = File::create(output_file)?;
                                file.write_all(&decrypted_data)?;
                                self.trans.insert(path.to_string(), format!("animation{}.bin", animation_id));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 解压模型JSON文件
    fn extract_model_json<P: AsRef<Path>>(&mut self, model_json: &str, dir: P) -> Result<()> {
        let dir = dir.as_ref();
        debug!("========= extracting model {} =========", model_json);
        
        // 如果已经解压过，直接返回
        if self.trans.contains_key(model_json) {
            return Ok(());
        }
        
        // 解密模型JSON文件
        let file = File::open(&self.lpk_path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut file = archive.by_name(model_json)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        let decrypted_data = self.decrypt_data(model_json, &buffer)?;
        let entry_s = String::from_utf8(decrypted_data)
            .map_err(|e| LpkError::DecryptionFailed(e.to_string()))?;
        
        let entry: Value = serde_json::from_str(&entry_s)?;
        let out_s = serde_json::to_string(&entry)?;
        let id = self.entrys.len();
        
        let entry_name = format!("model{}.json", id);
        self.entrys.insert(entry_name.clone(), out_s);
        self.trans.insert(model_json.to_string(), entry_name);
        
        debug!("model{}.json: {:?}", id, entry);
        
        // 递归处理模型中的所有引用
        self.process_model_references(&entry, dir, id)?;
        
        debug!("========= end of model {} =========", model_json);
        Ok(())
    }
}