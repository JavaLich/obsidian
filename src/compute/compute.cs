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
    vec3 center;
    float radius;
};

bool ray_sphere_intersection(Ray ray, Sphere sphere) {
    vec3 oc = ray.position - sphere.center;
    float a = dot(ray.direction, ray.direction);
    float b = 2.0 * dot(oc, ray.direction);
    float c = dot(oc, oc) - sphere.radius * sphere.radius;
    float discriminant = b * b - 4 * a * c;
    return (discriminant > 0);
}

uint ray_color(Ray ray) {
    Sphere sphere;
    sphere.center = vec3(0, 0, -1);
    sphere.radius = 0.5;
    if (ray_sphere_intersection(ray, sphere))
        return 0xff0000;
    
    return 0xb3cfff;
}

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

    Ray ray;
    ray.position = origin;
    ray.direction = lower_left + u * horizontal + v * vertical - origin;

    buf.data[x + y * scene.width] = ray_color(ray);
}
