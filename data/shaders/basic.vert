
#version 140

in vec3 position;
in vec3 normal;

uniform mat4 modelMatrix;

uniform worldData
{
    mat4 view_matrix;
    mat4 projection_matrix;
};

out vec3 vertex_color;

vec4 pos;

void main()
{
    mat4 modelViewMatrix = view_matrix * modelMatrix;
    pos = projection_matrix * modelViewMatrix * (vec4(position, 1.0));

    vertex_color = (position.xyz + vec3(1, 1, 1)) / 2;

    gl_Position = pos;
}

