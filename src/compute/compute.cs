#version 450

#define NUM_SPHERES 10

struct HitRecord {
    vec3 point;
    vec3 normal;
    float t;
    bool front_face;

};

struct Ray {
    vec3 position;
    vec3 direction;
};

struct Sphere {
    vec3 center;
    float radius; 
};

struct Camera {
    vec3 origin;
    vec3 lower_left_corner;
    vec3 horizontal;
    vec3 vertical;
    int num_samples;
} camera;

layout(local_size_x = 48, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

layout(set = 0, binding = 1) buffer SceneData {
    Sphere spheres[NUM_SPHERES];
    uint width;
    uint height;
} scene;

layout (set = 0, binding = 2) buffer CamData {
    float viewport_height;
    float focal_length;
    vec3 position;
} cam_data;

void set_front_face(inout HitRecord rec, Ray r, vec3 outward_normal) {
    rec.front_face = dot(r.direction, outward_normal) < 0;
    rec.normal = rec.front_face ? outward_normal : -outward_normal;
}

float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898,78.233))) * 43758.5453);
}

uint get_color(vec3 color) {
    uint r = uint(color.r * 255.0);
    uint g = uint(color.g * 255.0);
    uint b = uint(color.b * 255.0);
    return (r << 16) | (g << 8) | (b);
}

vec3 write_color(vec3 color, int num_samples) {
    float r = color.x;
    float g = color.y;
    float b = color.z;

    float scale = 1.0 / num_samples;
    r *= scale;
    g *= scale;
    b *= scale;

    return vec3(clamp(r, 0.0, 0.999), clamp(g, 0.0, 0.999), clamp(b, 0.0, 0.999));
}

vec3 ray_at(Ray ray, float t) {
    return ray.position + t * ray.direction;
}

Ray get_ray(float u, float v) {
    Ray ray;
    ray.position = camera.origin;
    ray.direction = camera.lower_left_corner + u * camera.horizontal + v * camera.vertical - camera.origin;
    return ray;
}

bool is_on_sphere(vec3 p, Sphere sphere) {
    float length = length(p - sphere.center);
    if (abs(length - sphere.radius) > 0.1)
        return false;
    return true;
}

bool ray_hit(Ray ray, Sphere sphere, inout HitRecord hit, float t_min, float t_max) {
    vec3 oc = ray.position - sphere.center;
    float a = dot(ray.direction, ray.direction);
    float half_b = dot(oc, ray.direction);
    float c = dot(oc, oc) - sphere.radius * sphere.radius;
    float discriminant = half_b * half_b - a * c;

    if (discriminant < 0)
        return false;

    float sqrtd = sqrt(discriminant);
    float root = (-half_b - sqrtd) / a ;
    if (root < t_min || t_max < root) {
        root = (-half_b + sqrtd) / a;
        if (root < t_min || t_max < root)
            return false;
    }

    hit.t = root;
    hit.point = ray_at(ray, hit.t);
    vec3 outward_normal = (hit.point - sphere.center) / sphere.radius; 
    set_front_face(hit, ray, outward_normal);
    
    return true;
}

vec3 ray_color(Ray ray) {
    HitRecord hit;
    bool is_hit = false;
    float t_max = 100.0;
    float t_min = 0.0;
    float closest = t_max;

    vec3 ret_color = vec3(0.0, 0.8, 1.0);

    for (int i = 0; i < NUM_SPHERES; i++) {
        HitRecord temp;
        if (ray_hit(ray, scene.spheres[i], temp, t_min, closest)) {
            is_hit = true;
            closest = temp.t;
            hit = temp;
        }
    }

    if (is_hit) {
        ret_color = (normalize(hit.normal) + 1.0) / 2.0; 
    }

    return ret_color;
}

void main() {
    uint x = gl_GlobalInvocationID.x % scene.width;
    uint y = gl_GlobalInvocationID.x / scene.width;


    float aspect_ratio = float(scene.width) / float(scene.height); 
    float viewport_width = aspect_ratio * cam_data.viewport_height;
    float viewport_height = cam_data.viewport_height;

    camera.origin = vec3(0);
    camera.horizontal = vec3(viewport_width, 0, 0);
    camera.vertical = vec3(0, viewport_height, 0);
    camera.lower_left_corner = camera.origin - camera.horizontal/2 - camera.vertical/2 - vec3(0, 0, cam_data.focal_length);
    camera.num_samples = 10;

    vec3 color = vec3(0);

    for (int i = 0; i < camera.num_samples; i++) {
        float u = float(x + rand(vec2(x * i, x))) / float(scene.width - 1);
        float v = float(y + rand(vec2(y, y * i))) / float(scene.height - 1);
        Ray ray = get_ray(u, v);
        color += ray_color(ray);
    }

    buf.data[x + y * scene.width] = get_color(write_color(color, camera.num_samples));
}
