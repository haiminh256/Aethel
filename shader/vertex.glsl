#version 330 core
layout (location = 0) in vec3 aPos;      // Khớp với location 0
layout (location = 1) in vec4 aColor;    // Khớp với location 1
layout (location = 2) in vec3 aNormal;   // Khớp với location 2 (Mới thêm)
layout (location = 3) in vec2 aTexCoord; // Thay đổi từ location 2 thành location 

out vec4 ourColor;
out vec2 TexCoord;

uniform mat4 camMatrix;
uniform mat4 model;

void main() {
    gl_Position = camMatrix * model * vec4(aPos, 1.0);
    ourColor = aColor;
    TexCoord = aTexCoord;
}
