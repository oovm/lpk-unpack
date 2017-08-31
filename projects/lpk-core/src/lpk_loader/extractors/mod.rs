use super::*;

impl LpkLoader {
    /// 解压标准格式的LPK文件（STD2_0或STM_1_0）
    pub(crate) fn extract_standard<P: AsRef<Path>>(&mut self, output_dir: P) -> Result<()> {
        let output_dir = output_dir.as_ref();

        // 先收集所有需要处理的角色和服装信息
        let mut extraction_tasks = Vec::new();

        if let Some(list) = self.mlve_config.get("list").and_then(|v| v.as_array()) {
            for chara in list {
                // 获取角色名称
                let chara_name = match chara.get("character").and_then(|v| v.as_str()) {
                    Some("") | None => "character",
                    Some(name) => name,
                }
                .to_string();

                let subdir = output_dir.join(&chara_name);
                safe_mkdir(&subdir)?;

                // 收集服装信息
                if let Some(costumes) = chara.get("costume").and_then(|v| v.as_array()) {
                    for (i, costume) in costumes.iter().enumerate() {
                        if let Some(path) = costume.get("path").and_then(|v| v.as_str()) {
                            if !path.is_empty() {
                                extraction_tasks.push((
                                    path.to_string(),
                                    subdir.clone(),
                                    format!("{}_costume_{}", chara_name, i),
                                ));
                            }
                        }
                    }
                }
            }
        }

        // 处理所有服装
        for (path, subdir, costume_name) in extraction_tasks {
            info!("extracting {}", costume_name);
            self.extract_costume(&path, &subdir)?;
        }

        // 处理所有条目
        if let Some(list) = self.mlve_config.get("list").and_then(|v| v.as_array()) {
            for chara in list {
                let chara_name = match chara.get("character").and_then(|v| v.as_str()) {
                    Some("") | None => "character",
                    Some(name) => name,
                };

                let subdir = output_dir.join(chara_name);

                // 替换加密文件名为解密后的文件名
                for (name, content) in &self.entrys {
                    let mut out_s = content.clone();
                    for (k, v) in &self.uncompressed {
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
}

impl LpkLoader {
    /// 解压旧版格式的LPK文件
    pub(crate) fn extract_legacy<P: AsRef<Path>>(&mut self, output_dir: P) -> Result<()> {
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
            }
            else {
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
}
