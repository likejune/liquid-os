#version 100
// Dual Kawase blur, upsample. Render into a double-size target.
precision mediump float;
uniform sampler2D tex;
uniform float alpha;
uniform vec2 halfpixel;
uniform float offset;
varying vec2 v_coords;

void main() {
    vec2 uv = v_coords;
    vec2 h = halfpixel;
    vec4 sum = texture2D(tex, uv + vec2(-h.x * 2.0, 0.0) * offset);
    sum += texture2D(tex, uv + vec2(-h.x, h.y) * offset) * 2.0;
    sum += texture2D(tex, uv + vec2(0.0, h.y * 2.0) * offset);
    sum += texture2D(tex, uv + vec2(h.x, h.y) * offset) * 2.0;
    sum += texture2D(tex, uv + vec2(h.x * 2.0, 0.0) * offset);
    sum += texture2D(tex, uv + vec2(h.x, -h.y) * offset) * 2.0;
    sum += texture2D(tex, uv + vec2(0.0, -h.y * 2.0) * offset);
    sum += texture2D(tex, uv + vec2(-h.x, -h.y) * offset) * 2.0;
    gl_FragColor = (sum / 12.0) * alpha;
}
