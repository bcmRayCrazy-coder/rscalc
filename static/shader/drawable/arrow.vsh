#version 330 core
layout(location = 0) in vec3 aPos;
// layout(location = 1) in vec3 aDirection;
// layout(location = 2) in float aRotation;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    // vec4 v_dir4 = view * projection * vec4(aDirection, 1.0);
    // vec3 v_direction = normalize(vec3(v_dir4.x, v_dir4.y, v_dir4.z));

    gl_Position = model * projection * view * vec4(aPos, 1.0);

    // if (aRotation != 0.0)
    // {
    //     // gl_Position.x += 0.5;
    //     // gl_Position.x += cos(aRotation) * 0.1;
    //     // gl_Position.y += sin(aRotation) * 0.1;
    //     gl_Position.x += cos(aRotation + acos(v_direction.x)) * 0.4;
    //     gl_Position.y += sin(aRotation + asin(v_direction.y)) * 0.4;
    //     gl_Position.z += v_direction.z * 0.4;
    // }
}