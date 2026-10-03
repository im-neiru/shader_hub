struct Camera {
    view_proj: mat4x4<f32>,
};
struct Time {
    seconds: f32,
    _padding0: f32,
    _padding1: f32,
    _padding2: f32,
};

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<uniform> time: Time;

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
    var out: VertexOutput;
    out.position = camera.view_proj * vec4<f32>(input.position, 1.0);
    out.color = input.color;
    out.local_position = input.position;
    return out;
}

fn hash3(p: vec3<f32>) -> f32 {
    var q = fract(p * 0.3183099 + vec3<f32>(0.1, 0.2, 0.3));
    q *= 17.0;
    return fract(q.x * q.y * q.z * (q.x + q.y + q.z));
}

fn noise3(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    let x00 = mix(hash3(i + vec3<f32>(0.0, 0.0, 0.0)), hash3(i + vec3<f32>(1.0, 0.0, 0.0)), u.x);
    let x10 = mix(hash3(i + vec3<f32>(0.0, 1.0, 0.0)), hash3(i + vec3<f32>(1.0, 1.0, 0.0)), u.x);
    let x01 = mix(hash3(i + vec3<f32>(0.0, 0.0, 1.0)), hash3(i + vec3<f32>(1.0, 0.0, 1.0)), u.x);
    let x11 = mix(hash3(i + vec3<f32>(0.0, 1.0, 1.0)), hash3(i + vec3<f32>(1.0, 1.0, 1.0)), u.x);

    return mix(mix(x00, x10, u.y), mix(x01, x11, u.y), u.z);
}

fn fbm(p_in: vec3<f32>) -> f32 {
    var p = p_in;
    var sum = 0.0;
    var amp = 0.5;
    for (var i = 0; i < 5; i++) {
        sum += noise3(p) * amp;
        p = p * 2.03 + vec3<f32>(1.7, 9.2, 3.1);
        amp *= 0.5;
    }
    return sum;
}

fn arc(v: f32, width: f32) -> f32 {
    let d = abs(v * 2.0 - 1.0);
    return width / (d + width);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let p = input.local_position;
    let t = time.seconds;

    let q = fbm(p * 1.8 + vec3<f32>(0.0, t * 0.25, 0.0));
    let f = fbm(p * 2.4 + vec3<f32>(q * 2.0) + vec3<f32>(t * 0.12, -t * 0.3, 0.0));
    let g = fbm(p * 3.6 - vec3<f32>(q * 1.5) + vec3<f32>(-t * 0.2, t * 0.15, t * 0.1));

    let core1 = pow(arc(f, 0.012), 1.5);
    let glow1 = arc(f, 0.10);
    let core2 = pow(arc(g, 0.008), 1.5);
    let glow2 = arc(g, 0.07);

    let bg = vec3<f32>(0.015, 0.01, 0.07);
    let violet = vec3<f32>(0.35, 0.10, 0.95);
    let cyan = vec3<f32>(0.10, 0.65, 1.0);
    let white = vec3<f32>(0.85, 0.95, 1.0);

    var color = bg;
    color += violet * glow1 * 0.35;
    color += cyan * glow2 * 0.25;
    color += cyan * core1 * 1.2;
    color += white * core1 * core1 * 1.5;
    color += white * core2 * 0.8;

    let flicker = 0.8 + 0.4 * sin(t * 18.0 + q * 25.0) * sin(t * 7.0 + g * 12.0);
    color *= flicker;

    color *= 1.6;
    color = color / (1.0 + color * 0.6);

    return vec4<f32>(color, 1.0);
}
