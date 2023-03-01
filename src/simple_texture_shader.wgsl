
struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex_pos: vec2<f32>,
}
struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) tex_pos: vec2<f32> 
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out:VertexOutput;
    out.pos = vec4<f32>(in.pos,1.0);
    out.tex_pos = in.tex_pos;

    return out;
}

@group(0) @binding(0)
var tex: texture_2d<f32>;
@group(0) @binding(1)
var samp: sampler;


@fragment 
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(tex,samp,in.tex_pos);
}