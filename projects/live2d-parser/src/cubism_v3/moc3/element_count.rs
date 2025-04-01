use super::*;

// 表示元素计数的结构体
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ElementCountTable {
    /// 部件数量
    pub parts: u32,
    /// 变形器数量
    pub deformers: u32,
    /// 变形器扭曲数量
    pub warp_deformers: u32,
    /// 旋转变形器数量
    pub rotation_deformers: u32,
    /// 艺术网格数量
    pub art_meshes: u32,
    /// 参数数量
    pub parameters: u32,
    /// 部件关键形态数量
    pub part_keyforms: u32,
    /// 扭曲变形器关键形态数量
    pub warp_deformer_keyforms: u32,
    /// 旋转变形器关键形态数量
    pub rotation_deformer_keyforms: u32,
    /// 艺术网格关键形态数量
    pub art_mesh_keyforms: u32,
    /// 关键形态位置数量
    pub keyform_positions: u32,
    /// 参数绑定索引数量
    pub parameter_binding_indices: u32,
    /// 关键形态绑定数量
    pub keyform_bindings: u32,
    /// 参数绑定数量
    pub parameter_bindings: u32,
    /// 键数量
    pub keys: u32,
    /// UV数量
    pub uvs: u32,
    /// 位置索引数量
    pub position_indices: u32,
    /// 可绘制掩码数量
    pub drawable_masks: u32,
    /// 绘制顺序组数量
    pub draw_order_groups: u32,
    /// 绘制顺序组对象数量
    pub draw_order_group_objects: u32,
    /// 粘合数量
    pub glue: u32,
    /// 粘合信息数量
    pub glue_info: u32,
    /// 粘合关键形态数量
    pub glue_keyforms: u32,
    /// 关键形态颜色（乘法）数量
    pub keyform_multiply_colors: u32,
    /// 关键形态颜色（屏幕）数量（隐藏）
    pub keyform_screen_colors: u32,
    /// 混合形状参数绑定数量
    pub blend_shape_parameter_bindings: u32,
    /// 混合形状关键形态绑定数量
    pub blend_shape_keyform_bindings: u32,
    /// 混合形状（扭曲变形器）数量
    pub blend_shapes_warp_deformers: u32,
    /// 混合形状（艺术网格）数量（隐藏）
    pub blend_shapes_art_meshes: u32,
    /// 混合形状约束索引数量
    pub blend_shape_constraint_indices: u32,
    /// 混合形状约束数量
    pub blend_shape_constraints: u32,
    /// 混合形状约束值数量
    pub blend_shape_constraint_values: u32,
    /// 混合形状（部件）数量
    pub blend_shapes_parts: u32,
    /// 混合形状（旋转变形器）数量
    pub blend_shapes_rotation_deformers: u32,
    /// 混合形状（粘合）数量
    pub blend_shapes_glue: u32,
}
