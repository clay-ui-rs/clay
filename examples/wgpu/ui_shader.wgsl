struct Vertex {
    @location(0)position: vec3<f32>,
    @location(1)color: vec3<f32>,
    @location(2)size: vec2<f32>
};

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(vertex: Vertex) -> VertexPayload {
    var out: VertexPayload;
    out.position = vec4<f32>(
        (vertex.position.x/(vertex.size.x/2.0))-1,
        -((vertex.position.y/(vertex.size.y/2.0))-1),
        vertex.position.z, 
        1.0
    );
    out.color = vertex.color;
    return out;
}

@fragment
fn fs_main(in:VertexPayload) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}