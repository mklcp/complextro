precision highp float;

uniform float time;
uniform vec2 mouse;
uniform vec2 resolution;

vec3 hsb2rgb( in vec3 c ){
    vec3 rgb = clamp(abs(mod(c.x*6.0+vec3(0.0,4.0,2.0),
                             6.0)-3.0)-1.0,
                     0.0,
                     1.0 );
    rgb = rgb*rgb*(3.0-2.0*rgb);
    return c.z * mix( vec3(1.0), rgb, c.y);
}


vec2 polar(vec2 z) {
    return vec2(length(z), atan(z.y,z.x));
}

vec2 id(vec2 z) {
    return z;
}


vec2 compact(vec2 z) {
    float rho = z.s;
    float theta = z.t;
    return vec2(log(abs(rho))/(1. + abs(log(abs(rho)))) + 0.5, theta);

}

#define TWO_PI 6.28318530718

vec3 color(vec2 z) {
    float h = z.t/TWO_PI + 0.5;
    float s = 1.;
    float b = z.s*0.7;
    return hsb2rgb(vec3(h, s, b));
}

void main(void) {
    vec2 z = (gl_FragCoord.xy * 2. - resolution.xy) / resolution.y; 

    gl_FragColor = vec4( color( compact(polar(z*4.)) ), 1 );
}

