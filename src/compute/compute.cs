#version 450

layout(local_size_x = 48, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

layout(set = 0, binding = 1) buffer SceneData {
    uint width;
    uint height;
} scene;

layout (set = 0, binding = 2) buffer Camera {
    float viewport_height;
    float focal_length;
    vec3 position;
} cam;

struct Ray {
    vec3 position;
    vec3 direction;
};

struct Sphere {
    vec3 position;
    float radius;
};

void main() {
    uint x = gl_GlobalInvocationID.x % scene.width;
    uint y = gl_GlobalInvocationID.x / scene.width;

    float aspect_ratio = float(scene.width) / float(scene.height); 

    if (x > 400) 
        buf.data[x + scene.width * y] = 0x00ff00;
    else 
        buf.data[x + scene.width * y] = 0xffff00;
}
