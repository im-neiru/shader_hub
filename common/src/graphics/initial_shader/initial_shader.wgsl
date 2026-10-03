struct Camera {
    view_proj: mat4x4<f32>,
};

struct Time {
    seconds: f32,
};

@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var<uniform> time: Time;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) local_position: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = camera.view_proj * vec4<f32>(input.position, 1.0);
    output.color = input.color;
    output.local_position = input.position;
    return output;
}

fn palette(t: f32) -> vec3<f32> {
    let tau = 6.2831853;
    return 0.5 + 0.5 * cos(tau * (t + vec3<f32>(0.00, 0.17, 0.33)));
}



struct Face {
    normal: vec3<f32>,
    bevel: vec3<f32>,
    u: f32,
    v: f32,
};

fn edge_bend(x: f32) -> f32 {
    return sign(x) * smoothstep(0.80, 1.0, abs(x));
}

fn get_face(p: vec3<f32>) -> Face {
    let a = abs(p.x);
    let b = abs(p.y);
    let c = abs(p.z);
    var f: Face;

    if (a >= b && a >= c) {
        f.normal = vec3<f32>(sign(p.x), 0.0, 0.0);
        f.bevel  = vec3<f32>(0.0, edge_bend(p.y), edge_bend(p.z));
        f.u = p.y; f.v = p.z;
    } else if (b >= a && b >= c) {
        f.normal = vec3<f32>(0.0, sign(p.y), 0.0);
        f.bevel  = vec3<f32>(edge_bend(p.x), 0.0, edge_bend(p.z));
        f.u = p.x; f.v = p.z;
    } else {
        f.normal = vec3<f32>(0.0, 0.0, sign(p.z));
        f.bevel  = vec3<f32>(edge_bend(p.x), edge_bend(p.y), 0.0);
        f.u = p.x; f.v = p.y;
    }
    return f;
}



fn blinn(n: vec3<f32>, l: vec3<f32>, v: vec3<f32>, shininess: f32) -> f32 {
    let h = normalize(l + v);
    return pow(max(dot(n, h), 0.0), shininess);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let p = input.local_position;
    let t = time.seconds;

    let face = get_face(p);


    let n = normalize(face.normal + face.bevel * 0.9);
    let view = normalize(vec3<f32>(0.45, 0.55, 1.0));


    let wave =
        sin(p.x * 4.0 + t * 1.7) +
        sin(p.y * 5.0 - t * 1.2) +
        sin(p.z * 6.0 + t * 1.4);
    let animated = palette(wave * 0.08 + t * 0.035);
    let albedo = mix(animated, animated * input.color, 0.35);

    let key_pos = vec3<f32>(
        cos(t * 0.9) * 3.2,
        1.6 + sin(t * 0.6) * 1.4,
        sin(t * 0.9) * 3.2
    );
    let key_vec = key_pos - p;
    let key_dist = length(key_vec);
    let key_dir = key_vec / key_dist;
    let key_atten = 1.0 / (1.0 + 0.08 * key_dist * key_dist);
    let key_color = vec3<f32>(1.0, 0.88, 0.7) * 2.4;

    let fill_dir = normalize(vec3<f32>(
        -cos(t * 0.5 + 1.0),
        -0.3,
        -sin(t * 0.5 + 1.0)
    ));
    let fill_color = vec3<f32>(0.25, 0.45, 1.0) * 0.8;

    let key_diff  = clamp((dot(n, key_dir)  + 0.25) / 1.25, 0.0, 1.0);
    let fill_diff = clamp((dot(n, fill_dir) + 0.5)  / 1.5,  0.0, 1.0);

    let up = n.y * 0.5 + 0.5;
    let ambient = mix(vec3<f32>(0.05, 0.04, 0.07), vec3<f32>(0.12, 0.16, 0.24), up);

    var lit = albedo * (
        ambient +
        key_color * key_diff * key_atten +
        fill_color * fill_diff
    );

    let bevel_amount = clamp(length(face.bevel), 0.0, 1.0);
    let spec_key = blinn(n, key_dir, view, 64.0) * key_atten;
    let spec_fill = blinn(n, fill_dir, view, 24.0) * 0.25;
    lit += key_color * spec_key * (0.5 + bevel_amount * 1.2);
    lit += fill_color * spec_fill;

    let fres = pow(1.0 - max(dot(n, view), 0.0), 3.0);
    lit += vec3<f32>(0.1, 0.5, 0.9) * fres * 0.6;

    let edge_distance = min(1.0 - abs(face.u), 1.0 - abs(face.v));
    let edge = 1.0 - smoothstep(0.0, 0.12, edge_distance);
    let edge_pulse = 0.75 + 0.25 * sin(t * 2.5);
    var emissive = vec3<f32>(0.15, 0.75, 1.0) * edge * 0.9 * edge_pulse;

    let band = sin((face.u + face.v) * 9.0 - t * 4.0);
    emissive += vec3<f32>(0.05, 0.2, 0.3) * smoothstep(0.2, 0.95, band);

    let core = 1.0 - smoothstep(0.0, 1.5, length(p));
    emissive += vec3<f32>(0.02, 0.08, 0.15) * core;

    var color = lit + emissive;

    color = color / (1.0 + color * 0.6);

    return vec4<f32>(color, 1.0);
}
