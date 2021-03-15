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
    float viewport_width = aspect_ratio * cam.viewport_height;
    float viewport_height = cam.viewport_height;

    vec3 origin = vec3(0);
    vec3 horizontal = vec3(viewport_width, 0, 0);
    vec3 vertical = vec3(0, viewport_height, 0);
    vec3 lower_left = origin - horizontal/2 - vertical/2 - vec3(0, 0, cam.focal_length);

    float u = float(x) / float(scene.width);
    float v = float(y) / float(scene.height);

    buf.data[x + y * scene.width] = 0xb3cfff;
}
