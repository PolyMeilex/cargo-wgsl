[[block]]
struct ViewUniform {
    transform: mat4x4<f32>;
    size: vec2<f32>;
};

[[group(0), binding(0)]] var<uniform> view_uniform: ViewUniform;

// 
// Vertex shader
// 
struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};


[[stage(vertex)]]
fn vs_main(
    [[location(0)]] vertex_position: vec2<f32>,
    [[location(1)]] instance_position: vec2<f32>,
    [[location(2)]] instance_size: vec2<f32>,
    [[location(3)]] instance_color: vec4<f32>,
) -> VertexOutput {
    let i_transform: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(instance_size.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, instance_size.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(instance_position, 0.0, 1.0)
    );

    var output: VertexOutput;
    output.color = instance_color;
    output.position = view_uniform.transform * i_transform * vec4<f32>(vertex_position, 0.0, 1.0);
    
    return output;
}

// 
// Fragment shader
// 

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32>{
    return in.color;
}