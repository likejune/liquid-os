#version 100
// Dual Kawase blur, downsample. Render into a half-size target.
precision mediump float;
uniform sampler2D tex;
uniform float alpha;
uniform vec2 halfpixel;   // 0.5 / target size
uniform float offset;     // 1.0 .. 4.0
varying vec2 v_coords;

void main() {
    vec2 uv = v_coords;
    vec4 sum = texture2D(tex, uv) * 4.0;
    sum += texture2D(tex, uv - halfpixel * offset);
    sum += texture2D(tex, uv + halfpixel * offset);
    sum += texture2D(tex, uv + vec2(halfpixel.x, -halfpixel.y) * offset);
    sum += texture2D(tex, uv - vec2(halfpixel.x, -halfpixel.y) * offset);
    gl_FragColor = (sum / 8.0) * alpha;
}
