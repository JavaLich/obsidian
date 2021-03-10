#version 450

layout(local_size_x = 48, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

struct Ray {
    vec3 position;
    vec3 direction;
};

struct Sphere {
    vec3 position:
    float radius;
};

void main() {
    uint idx = gl_GlobalInvocationID.x;
    buf.data[idx] = 0x00ff00;
}
