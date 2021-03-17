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

uint get_color(vec3 color) {
    uint r = uint(color.r * 255.0);
    uint g = uint(color.g * 255.0);
    uint b = uint(color.b * 255.0);
    return (r << 16) | (g << 8) | (b);
}

vec3 ray_at(Ray ray, float t) {
    return ray.position + t * ray.direction;
}

bool is_on_sphere(vec3 p, Sphere sphere) {
    float length = length(p - sphere.center);
    if (abs(length - sphere.radius) > 0.1)
        return false;
    return true;
}

float ray_hit(Ray ray, Sphere sphere) {
    vec3 oc = ray.position - sphere.center;
    float a = dot(ray.direction, ray.direction);
    float half_b = dot(oc, ray.direction);
    float c = dot(oc, oc) - sphere.radius * sphere.radius;
    float discriminant = half_b * half_b - a * c;
    if (discriminant < 0)
        return -1.0;
    else 
        return (-half_b - sqrt(discriminant)) / a;
}

uint ray_color(Ray ray) {
    Sphere sphere;
    sphere.center = vec3(0, 0, -1);
    sphere.radius = 0.5;

    float t = ray_hit(ray, sphere);

    if (t > 0.0) {
        vec3 normal = normalize(ray_at(ray, t) - sphere.center);
        normal = (normal + 1.0) * 0.5;
        return get_color(normal);
    }
    
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
