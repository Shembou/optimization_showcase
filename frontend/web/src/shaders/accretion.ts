export const ACCRETION_VERT_SRC = `#version 300 es
in vec4 a_position;
void main() {
    gl_Position = a_position;
}`;

export const ACCRETION_FRAG_SRC = `#version 300 es
precision highp float;

uniform vec3  iResolution;  // viewport resolution (in pixels)
uniform float iTime;        // shader playback time (in seconds)

out vec4 outColor;

/*
    "Accretion" by @XorDev
    https://www.shadertoy.com/view/WcKXDV
*/

void mainImage(out vec4 O, vec2 I)
{
    //Raymarch depth
    float z,
    //Step distance
    d,
    //Raymarch iterator
    i;
    //Clear fragColor and raymarch 100 steps
    for(O*=i; i++<2e1; )
    {
        //Sample point (from ray direction)
        vec3 p = z*normalize(vec3(I+I,0)-iResolution.xyx)+.1;
        
        //Polar coordinates and additional transformations
        p = vec3(atan(p.y/.2,p.x)*2., p.z/3., length(p.xy)-5.-z*.2);
        
        //Apply turbulence and refraction effect
        for(d=0.; d++<7.;)
            p += sin(p.yzx*d+iTime+.3*i)/d;
            
        //Distance to cylinder and waves with refraction
        z += d = length(vec4(.4*cos(p)-.4, p.z));
        
        //Coloring and brightness
        O += (1.+cos(p.x+i*.4+z+vec4(6,1,2,0)))/d;
    }
    //Tanh tonemap
    O = tanh(O*O/4e2);
}

void main() {
    mainImage(outColor, gl_FragCoord.xy);
}`;