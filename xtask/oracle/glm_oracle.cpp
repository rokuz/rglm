#define GLM_FORCE_RADIANS
#define GLM_FORCE_DEPTH_ZERO_TO_ONE
#define GLM_ENABLE_EXPERIMENTAL

#include <cstdint>
#include <cstdio>
#include <cstring>
#include <vector>

#include <glm/glm.hpp>
#include <glm/gtc/color_space.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/packing.hpp>
#include <glm/gtc/quaternion.hpp>
#include <glm/gtc/type_ptr.hpp>

namespace {

bool read_exact(void* dst, std::size_t n) {
    return std::fread(dst, 1, n, stdin) == n;
}

bool write_exact(const void* src, std::size_t n) {
    return std::fwrite(src, 1, n, stdout) == n;
}

template <typename T>
T load_val(const std::uint8_t* p, std::size_t& off) {
    T v{};
    std::memcpy(&v, p + off, sizeof(T));
    off += sizeof(T);
    return v;
}

glm::vec2 load_vec2(const std::uint8_t* p, std::size_t& off) {
    float x = load_val<float>(p, off);
    float y = load_val<float>(p, off);
    return glm::vec2(x, y);
}

glm::vec3 load_vec3(const std::uint8_t* p, std::size_t& off) {
    float x = load_val<float>(p, off);
    float y = load_val<float>(p, off);
    float z = load_val<float>(p, off);
    return glm::vec3(x, y, z);
}

glm::vec4 load_vec4(const std::uint8_t* p, std::size_t& off) {
    float x = load_val<float>(p, off);
    float y = load_val<float>(p, off);
    float z = load_val<float>(p, off);
    float w = load_val<float>(p, off);
    return glm::vec4(x, y, z, w);
}

glm::ivec4 load_ivec4(const std::uint8_t* p, std::size_t& off) {
    std::int32_t x = load_val<std::int32_t>(p, off);
    std::int32_t y = load_val<std::int32_t>(p, off);
    std::int32_t z = load_val<std::int32_t>(p, off);
    std::int32_t w = load_val<std::int32_t>(p, off);
    return glm::ivec4(x, y, z, w);
}

glm::uvec4 load_uvec4(const std::uint8_t* p, std::size_t& off) {
    std::uint32_t x = load_val<std::uint32_t>(p, off);
    std::uint32_t y = load_val<std::uint32_t>(p, off);
    std::uint32_t z = load_val<std::uint32_t>(p, off);
    std::uint32_t w = load_val<std::uint32_t>(p, off);
    return glm::uvec4(x, y, z, w);
}

glm::quat load_quat_xyzw(const std::uint8_t* p, std::size_t& off) {
    float x = load_val<float>(p, off);
    float y = load_val<float>(p, off);
    float z = load_val<float>(p, off);
    float w = load_val<float>(p, off);
    return glm::quat(w, x, y, z);
}

template <int C, int R, typename T>
glm::mat<C, R, T, glm::defaultp> load_mat(const std::uint8_t* p, std::size_t& off) {
    glm::mat<C, R, T, glm::defaultp> m(T(0));
    for (int c = 0; c < C; ++c) {
        for (int r = 0; r < R; ++r) {
            m[c][r] = load_val<T>(p, off);
        }
    }
    return m;
}

template <typename T>
void push_val(std::vector<std::uint8_t>& out, const T& v) {
    const auto* bytes = reinterpret_cast<const std::uint8_t*>(&v);
    out.insert(out.end(), bytes, bytes + sizeof(T));
}

void push_vec2(std::vector<std::uint8_t>& out, const glm::vec2& v) {
    push_val(out, v.x);
    push_val(out, v.y);
}

void push_vec3(std::vector<std::uint8_t>& out, const glm::vec3& v) {
    push_val(out, v.x);
    push_val(out, v.y);
    push_val(out, v.z);
}

void push_vec4(std::vector<std::uint8_t>& out, const glm::vec4& v) {
    push_val(out, v.x);
    push_val(out, v.y);
    push_val(out, v.z);
    push_val(out, v.w);
}

void push_ivec4(std::vector<std::uint8_t>& out, const glm::ivec4& v) {
    push_val<std::int32_t>(out, v.x);
    push_val<std::int32_t>(out, v.y);
    push_val<std::int32_t>(out, v.z);
    push_val<std::int32_t>(out, v.w);
}

void push_uvec4(std::vector<std::uint8_t>& out, const glm::uvec4& v) {
    push_val<std::uint32_t>(out, v.x);
    push_val<std::uint32_t>(out, v.y);
    push_val<std::uint32_t>(out, v.z);
    push_val<std::uint32_t>(out, v.w);
}

void push_quat_xyzw(std::vector<std::uint8_t>& out, const glm::quat& q) {
    push_val(out, q.x);
    push_val(out, q.y);
    push_val(out, q.z);
    push_val(out, q.w);
}

template <int C, int R, typename T>
void push_mat(std::vector<std::uint8_t>& out, const glm::mat<C, R, T, glm::defaultp>& m) {
    for (int c = 0; c < C; ++c) {
        for (int r = 0; r < R; ++r) {
            push_val<T>(out, m[c][r]);
        }
    }
}

void dispatch(std::uint32_t op, const std::uint8_t* p, std::vector<std::uint8_t>& out) {
    std::size_t off = 0;
    switch (op) {
        case 1: push_vec4(out, glm::abs(load_vec4(p, off))); break;
        case 2: push_vec4(out, glm::sign(load_vec4(p, off))); break;
        case 3: push_vec4(out, glm::floor(load_vec4(p, off))); break;
        case 4: push_vec4(out, glm::trunc(load_vec4(p, off))); break;
        case 5: push_vec4(out, glm::round(load_vec4(p, off))); break;
        case 6: push_vec4(out, glm::roundEven(load_vec4(p, off))); break;
        case 7: push_vec4(out, glm::ceil(load_vec4(p, off))); break;
        case 8: push_vec4(out, glm::fract(load_vec4(p, off))); break;
        case 9: {
            auto a = load_vec4(p, off);
            auto b = load_vec4(p, off);
            push_vec4(out, glm::mod(a, b));
            break;
        }
        case 10: {
            auto a = load_vec4(p, off);
            auto b = load_vec4(p, off);
            push_vec4(out, glm::min(a, b));
            break;
        }
        case 11: {
            auto a = load_vec4(p, off);
            auto b = load_vec4(p, off);
            push_vec4(out, glm::max(a, b));
            break;
        }
        case 12: {
            auto x = load_vec4(p, off);
            auto lo = load_vec4(p, off);
            auto hi = load_vec4(p, off);
            push_vec4(out, glm::clamp(x, lo, hi));
            break;
        }
        case 13: {
            auto x = load_vec4(p, off);
            auto y = load_vec4(p, off);
            auto a = load_vec4(p, off);
            push_vec4(out, glm::mix(x, y, a));
            break;
        }
        case 14: {
            auto edge = load_vec4(p, off);
            auto x = load_vec4(p, off);
            push_vec4(out, glm::step(edge, x));
            break;
        }
        case 15: {
            auto e0 = load_vec4(p, off);
            auto e1 = load_vec4(p, off);
            auto x = load_vec4(p, off);
            push_vec4(out, glm::smoothstep(e0, e1, x));
            break;
        }
        case 16: {
            auto a = load_vec4(p, off);
            auto b = load_vec4(p, off);
            auto c = load_vec4(p, off);
            push_vec4(out, glm::fma(a, b, c));
            break;
        }
        case 20: push_vec4(out, glm::radians(load_vec4(p, off))); break;
        case 21: push_vec4(out, glm::degrees(load_vec4(p, off))); break;
        case 22: push_vec4(out, glm::sin(load_vec4(p, off))); break;
        case 23: push_vec4(out, glm::cos(load_vec4(p, off))); break;
        case 24: push_vec4(out, glm::tan(load_vec4(p, off))); break;
        case 25: push_vec4(out, glm::asin(load_vec4(p, off))); break;
        case 26: push_vec4(out, glm::acos(load_vec4(p, off))); break;
        case 27: push_vec4(out, glm::atan(load_vec4(p, off))); break;
        case 28: {
            auto y = load_vec4(p, off);
            auto x = load_vec4(p, off);
            push_vec4(out, glm::atan(y, x));
            break;
        }
        case 29: push_vec4(out, glm::sinh(load_vec4(p, off))); break;
        case 30: push_vec4(out, glm::cosh(load_vec4(p, off))); break;
        case 31: push_vec4(out, glm::tanh(load_vec4(p, off))); break;
        case 32: push_vec4(out, glm::asinh(load_vec4(p, off))); break;
        case 33: push_vec4(out, glm::acosh(load_vec4(p, off))); break;
        case 34: push_vec4(out, glm::atanh(load_vec4(p, off))); break;
        case 35: {
            auto x = load_vec4(p, off);
            auto y = load_vec4(p, off);
            push_vec4(out, glm::pow(x, y));
            break;
        }
        case 36: push_vec4(out, glm::exp(load_vec4(p, off))); break;
        case 37: push_vec4(out, glm::log(load_vec4(p, off))); break;
        case 38: push_vec4(out, glm::exp2(load_vec4(p, off))); break;
        case 39: push_vec4(out, glm::log2(load_vec4(p, off))); break;
        case 40: push_vec4(out, glm::sqrt(load_vec4(p, off))); break;
        case 41: push_vec4(out, glm::inversesqrt(load_vec4(p, off))); break;
        case 50: push_val<float>(out, glm::length(load_vec3(p, off))); break;
        case 51: {
            auto a = load_vec3(p, off);
            auto b = load_vec3(p, off);
            push_val<float>(out, glm::distance(a, b));
            break;
        }
        case 52: {
            auto a = load_vec4(p, off);
            auto b = load_vec4(p, off);
            push_val<float>(out, glm::dot(a, b));
            break;
        }
        case 53: {
            auto a = load_vec3(p, off);
            auto b = load_vec3(p, off);
            push_vec3(out, glm::cross(a, b));
            break;
        }
        case 54: push_vec3(out, glm::normalize(load_vec3(p, off))); break;
        case 55: {
            auto n = load_vec3(p, off);
            auto i = load_vec3(p, off);
            auto nref = load_vec3(p, off);
            push_vec3(out, glm::faceforward(n, i, nref));
            break;
        }
        case 56: {
            auto i = load_vec3(p, off);
            auto n = load_vec3(p, off);
            push_vec3(out, glm::reflect(i, n));
            break;
        }
        case 57: {
            auto i = load_vec3(p, off);
            auto n = load_vec3(p, off);
            auto eta = load_val<float>(p, off);
            push_vec3(out, glm::refract(i, n, eta));
            break;
        }
        case 60: push_val<std::uint32_t>(out, glm::packUnorm2x16(load_vec2(p, off))); break;
        case 61: push_vec2(out, glm::unpackUnorm2x16(load_val<std::uint32_t>(p, off))); break;
        case 62: push_val<std::uint32_t>(out, glm::packSnorm2x16(load_vec2(p, off))); break;
        case 63: push_vec2(out, glm::unpackSnorm2x16(load_val<std::uint32_t>(p, off))); break;
        case 64: push_val<std::uint32_t>(out, glm::packUnorm4x8(load_vec4(p, off))); break;
        case 65: push_vec4(out, glm::unpackUnorm4x8(load_val<std::uint32_t>(p, off))); break;
        case 66: push_val<std::uint32_t>(out, glm::packSnorm4x8(load_vec4(p, off))); break;
        case 67: push_vec4(out, glm::unpackSnorm4x8(load_val<std::uint32_t>(p, off))); break;
        case 68: push_val<std::uint32_t>(out, glm::packHalf2x16(load_vec2(p, off))); break;
        case 69: push_vec2(out, glm::unpackHalf2x16(load_val<std::uint32_t>(p, off))); break;
        case 70: {
            auto v = load_ivec4(p, off);
            auto offset = load_val<std::int32_t>(p, off);
            auto bits = load_val<std::int32_t>(p, off);
            push_ivec4(out, glm::bitfieldExtract(v, offset, bits));
            break;
        }
        case 71: {
            auto v = load_uvec4(p, off);
            auto offset = load_val<std::int32_t>(p, off);
            auto bits = load_val<std::int32_t>(p, off);
            push_uvec4(out, glm::bitfieldExtract(v, offset, bits));
            break;
        }
        case 72: {
            auto base = load_uvec4(p, off);
            auto ins = load_uvec4(p, off);
            auto offset = load_val<std::int32_t>(p, off);
            auto bits = load_val<std::int32_t>(p, off);
            push_uvec4(out, glm::bitfieldInsert(base, ins, offset, bits));
            break;
        }
        case 73: push_uvec4(out, glm::bitfieldReverse(load_uvec4(p, off))); break;
        case 74: push_ivec4(out, glm::bitCount(load_uvec4(p, off))); break;
        case 75: push_ivec4(out, glm::findLSB(load_uvec4(p, off))); break;
        case 76: push_ivec4(out, glm::findMSB(load_ivec4(p, off))); break;
        case 77: push_ivec4(out, glm::findMSB(load_uvec4(p, off))); break;
        case 80: {
            auto a = load_mat<2, 2, float>(p, off);
            auto b = load_mat<2, 2, float>(p, off);
            push_mat<2, 2, float>(out, a * b);
            break;
        }
        case 81: {
            auto a = load_mat<3, 3, float>(p, off);
            auto b = load_mat<3, 3, float>(p, off);
            push_mat<3, 3, float>(out, a * b);
            break;
        }
        case 82: {
            auto a = load_mat<4, 4, float>(p, off);
            auto b = load_mat<4, 4, float>(p, off);
            push_mat<4, 4, float>(out, a * b);
            break;
        }
        case 83: {
            auto m = load_mat<4, 4, float>(p, off);
            auto v = load_vec4(p, off);
            push_vec4(out, m * v);
            break;
        }
        case 84: {
            auto v = load_vec4(p, off);
            auto m = load_mat<4, 4, float>(p, off);
            push_vec4(out, v * m);
            break;
        }
        case 85: {
            auto m = load_mat<3, 4, float>(p, off);
            auto v = load_vec3(p, off);
            push_vec4(out, m * v);
            break;
        }
        case 86: {
            auto v = load_vec4(p, off);
            auto m = load_mat<3, 4, float>(p, off);
            push_vec3(out, v * m);
            break;
        }
        case 87: {
            auto a = load_mat<2, 3, float>(p, off);
            auto b = load_mat<4, 2, float>(p, off);
            push_mat<4, 3, float>(out, a * b);
            break;
        }
        case 88: {
            auto m = load_mat<3, 4, float>(p, off);
            push_mat<4, 3, float>(out, glm::transpose(m));
            break;
        }
        case 89: {
            auto c = load_vec4(p, off);
            auto r = load_vec3(p, off);
            push_mat<3, 4, float>(out, glm::outerProduct(c, r));
            break;
        }
        case 90: {
            auto a = load_mat<3, 4, float>(p, off);
            auto b = load_mat<3, 4, float>(p, off);
            push_mat<3, 4, float>(out, glm::matrixCompMult(a, b));
            break;
        }
        case 91: {
            auto m2 = load_mat<2, 2, float>(p, off);
            auto m3 = load_mat<3, 3, float>(p, off);
            auto m4 = load_mat<4, 4, float>(p, off);
            push_val<float>(out, glm::determinant(m2));
            push_val<float>(out, glm::determinant(m3));
            push_val<float>(out, glm::determinant(m4));
            break;
        }
        case 92: push_mat<2, 2, float>(out, glm::inverse(load_mat<2, 2, float>(p, off))); break;
        case 93: push_mat<3, 3, float>(out, glm::inverse(load_mat<3, 3, float>(p, off))); break;
        case 94: push_mat<4, 4, float>(out, glm::inverse(load_mat<4, 4, float>(p, off))); break;
        case 95: {
            auto a = load_mat<4, 4, double>(p, off);
            auto b = load_mat<4, 4, double>(p, off);
            push_mat<4, 4, double>(out, a * b);
            break;
        }
        case 96: push_mat<4, 4, double>(out, glm::inverse(load_mat<4, 4, double>(p, off))); break;
        case 100: {
            auto a = load_quat_xyzw(p, off);
            auto b = load_quat_xyzw(p, off);
            push_quat_xyzw(out, a * b);
            break;
        }
        case 101: {
            auto q = load_quat_xyzw(p, off);
            auto v = load_vec3(p, off);
            push_vec3(out, q * v);
            break;
        }
        case 102: {
            auto angle = load_val<float>(p, off);
            auto axis = load_vec3(p, off);
            push_quat_xyzw(out, glm::angleAxis(angle, axis));
            break;
        }
        case 103: push_mat<3, 3, float>(out, glm::mat3_cast(load_quat_xyzw(p, off))); break;
        case 104: push_mat<4, 4, float>(out, glm::mat4_cast(load_quat_xyzw(p, off))); break;
        case 105: push_quat_xyzw(out, glm::quat_cast(load_mat<3, 3, float>(p, off))); break;
        case 106: {
            auto a = load_quat_xyzw(p, off);
            auto b = load_quat_xyzw(p, off);
            auto t = load_val<float>(p, off);
            push_quat_xyzw(out, glm::slerp(a, b, t));
            break;
        }
        case 107: push_quat_xyzw(out, glm::conjugate(load_quat_xyzw(p, off))); break;
        case 108: push_quat_xyzw(out, glm::inverse(load_quat_xyzw(p, off))); break;
        case 109: push_vec3(out, glm::eulerAngles(load_quat_xyzw(p, off))); break;
        case 110: {
            auto dir = load_vec3(p, off);
            auto up = load_vec3(p, off);
            push_quat_xyzw(out, glm::quatLookAtRH(dir, up));
            break;
        }
        case 111: {
            auto dir = load_vec3(p, off);
            auto up = load_vec3(p, off);
            push_quat_xyzw(out, glm::quatLookAtLH(dir, up));
            break;
        }
        case 120: {
            auto m = load_mat<4, 4, float>(p, off);
            auto v = load_vec3(p, off);
            push_mat<4, 4, float>(out, glm::translate(m, v));
            break;
        }
        case 121: {
            auto m = load_mat<4, 4, float>(p, off);
            auto angle = load_val<float>(p, off);
            auto axis = load_vec3(p, off);
            push_mat<4, 4, float>(out, glm::rotate(m, angle, axis));
            break;
        }
        case 122: {
            auto m = load_mat<4, 4, float>(p, off);
            auto v = load_vec3(p, off);
            push_mat<4, 4, float>(out, glm::scale(m, v));
            break;
        }
        case 123: {
            auto eye = load_vec3(p, off);
            auto center = load_vec3(p, off);
            auto up = load_vec3(p, off);
            push_mat<4, 4, float>(out, glm::lookAtRH(eye, center, up));
            break;
        }
        case 124: {
            auto eye = load_vec3(p, off);
            auto center = load_vec3(p, off);
            auto up = load_vec3(p, off);
            push_mat<4, 4, float>(out, glm::lookAtLH(eye, center, up));
            break;
        }
        case 125: {
            auto fovy = load_val<float>(p, off);
            auto aspect = load_val<float>(p, off);
            auto znear = load_val<float>(p, off);
            auto zfar = load_val<float>(p, off);
            push_mat<4, 4, float>(out, glm::perspectiveRH_ZO(fovy, aspect, znear, zfar));
            break;
        }
        case 126: {
            auto fovy = load_val<float>(p, off);
            auto aspect = load_val<float>(p, off);
            auto znear = load_val<float>(p, off);
            auto zfar = load_val<float>(p, off);
            push_mat<4, 4, float>(out, glm::perspectiveLH_ZO(fovy, aspect, znear, zfar));
            break;
        }
        case 127: {
            auto fovy = load_val<float>(p, off);
            auto aspect = load_val<float>(p, off);
            auto znear = load_val<float>(p, off);
            auto zfar = load_val<float>(p, off);
            push_mat<4, 4, float>(out, glm::perspectiveRH_NO(fovy, aspect, znear, zfar));
            break;
        }
        case 128: {
            auto l = load_val<float>(p, off);
            auto r = load_val<float>(p, off);
            auto b = load_val<float>(p, off);
            auto t = load_val<float>(p, off);
            auto n = load_val<float>(p, off);
            auto f = load_val<float>(p, off);
            push_mat<4, 4, float>(out, glm::orthoRH_ZO(l, r, b, t, n, f));
            break;
        }
        case 129: {
            auto l = load_val<float>(p, off);
            auto r = load_val<float>(p, off);
            auto b = load_val<float>(p, off);
            auto t = load_val<float>(p, off);
            auto n = load_val<float>(p, off);
            auto f = load_val<float>(p, off);
            push_mat<4, 4, float>(out, glm::orthoLH_ZO(l, r, b, t, n, f));
            break;
        }
        case 130: {
            auto l = load_val<float>(p, off);
            auto r = load_val<float>(p, off);
            auto b = load_val<float>(p, off);
            auto t = load_val<float>(p, off);
            auto n = load_val<float>(p, off);
            auto f = load_val<float>(p, off);
            push_mat<4, 4, float>(out, glm::orthoRH_NO(l, r, b, t, n, f));
            break;
        }
        case 131: {
            auto obj = load_vec3(p, off);
            auto model = load_mat<4, 4, float>(p, off);
            auto proj = load_mat<4, 4, float>(p, off);
            auto vp = load_vec4(p, off);
            push_vec3(out, glm::projectZO(obj, model, proj, vp));
            break;
        }
        case 132: {
            auto win = load_vec3(p, off);
            auto model = load_mat<4, 4, float>(p, off);
            auto proj = load_mat<4, 4, float>(p, off);
            auto vp = load_vec4(p, off);
            push_vec3(out, glm::unProjectZO(win, model, proj, vp));
            break;
        }
        case 133: push_vec4(out, glm::convertLinearToSRGB(load_vec4(p, off), 2.4f)); break;
        case 134: push_vec4(out, glm::convertSRGBToLinear(load_vec4(p, off))); break;
        default: break;
    }
}

}

int main() {
    std::vector<std::uint8_t> in_buf;
    std::vector<std::uint8_t> out_buf;
    for (;;) {
        std::uint32_t hdr[2] = {0, 0};
        if (!read_exact(hdr, sizeof(hdr))) {
            return 0;
        }
        std::uint32_t op = hdr[0];
        std::uint32_t len = hdr[1];
        if (op == 0) {
            return 0;
        }
        in_buf.resize(len);
        if (len > 0 && !read_exact(in_buf.data(), len)) {
            return 1;
        }
        out_buf.clear();
        dispatch(op, in_buf.data(), out_buf);
        auto out_len = static_cast<std::uint32_t>(out_buf.size());
        if (!write_exact(&out_len, sizeof(out_len))) {
            return 1;
        }
        if (out_len > 0 && !write_exact(out_buf.data(), out_len)) {
            return 1;
        }
        std::fflush(stdout);
    }
}
