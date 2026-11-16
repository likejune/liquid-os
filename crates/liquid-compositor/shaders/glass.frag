#version 100
// Liquid Glass composite. `tex` is the blurred backdrop under the surface,
// v_coords are 0..1 across the surface. Output is premultiplied alpha.
precision mediump float;
uniform sampler2D tex;
uniform float alpha;
uniform vec2 size;            // surface size, px
uniform float radius;         // corner radius, px
uniform vec4 tint;            // straight alpha
uniform float saturation;
uniform float rim_width;      // px
uniform vec4 rim_color;
uniform float iridescent_edge;
uniform float specular;
uniform float refraction;     // px
uniform vec3 iri0;
uniform vec3 iri1;
uniform vec3 iri2;
uniform vec3 iri3;
varying vec2 v_coords;

float sdf_rounded_rect(vec2 p, vec2 half_size, float r) {
    vec2 q = abs(p) - half_size + vec2(r);
    return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - r;
}

vec3 saturate_color(vec3 c, float s) {
    float l = dot(c, vec3(0.2126, 0.7152, 0.0722));
    return mix(vec3(l), c, s);
}

vec3 iridescent(float t) {
    t = fract(t);
    if (t < 0.3333) return mix(iri0, iri1, t * 3.0);
    if (t < 0.6666) return mix(iri1, iri2, (t - 0.3333) * 3.0);
    return mix(iri2, iri3, (t - 0.6666) * 3.0);
}

void main() {
    vec2 p = v_coords * size - size * 0.5;
    float r = min(radius, min(size.x, size.y) * 0.5);
    float d = sdf_rounded_rect(p, size * 0.5, r);
    if (d > 1.0) discard;
    float coverage = clamp(0.5 - d, 0.0, 1.0);

    // Refraction: near the edge the backdrop is pulled toward the center.
    float edge = clamp(1.0 + d / max(refraction * 4.0, 1.0), 0.0, 1.0);
    vec2 dir = normalize(p + vec2(0.0001));
    vec2 uv = v_coords - dir * edge * refraction / size;

    vec3 col = saturate_color(texture2D(tex, uv).rgb, saturation);
    col = mix(col, tint.rgb, tint.a);

    // Single light source, top-left.
    float s = pow(clamp(1.0 - distance(v_coords, vec2(0.0, 0.0)) * 0.9, 0.0, 1.0), 3.0);
    col += vec3(specular * s);

    // Iridescent outer edge (3px band).
    float angle = atan(p.y, p.x) / 6.2831853 + 0.5;
    float band = 1.0 - smoothstep(0.0, 3.0, -d);
    col = mix(col, iridescent(angle), band * iridescent_edge);

    // 1px white rim.
    float rim = 1.0 - smoothstep(0.0, rim_width, -d);
    col = mix(col, rim_color.rgb, rim * rim_color.a);

    gl_FragColor = vec4(clamp(col, 0.0, 1.0), 1.0) * coverage * alpha;
}
