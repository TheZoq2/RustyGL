#version 140

in vec3 position;

in vec3 vertex_color;

in vec4 vertex_position;
in vec3 vertex_normal;

out vec4 color;

const int MAX_LIGHTS = 32;
struct SphericalLight
{
    float range;
    vec3 position;
    vec3 color;
};

uniform lights
{
    uint sphere_light_count;

    SphericalLight sphere_lights[32];
};

SphericalLight sphere_light;


void main()
{
    sphere_light.position = vec3(2.0, 2.0, 0.0);

    float brightness = dot(normalize(vec3(vertex_normal)), normalize(vec3(vertex_position) - vec3(sphere_light.position)));

    brightness = (brightness + 0.5) / 1.5;

    vec3 dark_color = vec3(1.0, 0.0, 0.0);
    vec3 bright_color = vec3(0.0, 1.0, 0.0);

    //color = vec4(vertex_color, 1.0) * brightness;
    color = vec4(mix(dark_color, bright_color, brightness), 1.0) * brightness;
}

