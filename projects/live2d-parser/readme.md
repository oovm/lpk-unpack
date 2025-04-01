# live2d-parser

一个用于解析Live2D模型文件的Rust库，支持Cubism 3.0/4.0格式。

## 功能特性

- 解析Cubism 3.0/4.0的.moc3文件
- 提取模型参数、部件和网格信息
## 安装

在`Cargo.toml`中添加依赖：

```toml
[dependencies]
live2d-parser = { path = "../live2d-parser" }
```

## 使用示例

```rust
use live2d_parser::cubism_v3::Moc3;

fn main() {
    let data = std::fs::read("model.moc3").unwrap();
    let moc3 = unsafe {
        Moc3::parse(&data).unwrap()
    };
    
    // 遍历所有参数
    for param in moc3.parameters() {
        println!("参数: {}, 默认值: {}", param.name, param.default_value);
    }
    
    // 遍历所有网格
    for mesh in moc3.art_meshes() {
        println!("网格: {}", mesh.name);
    }
}
```
