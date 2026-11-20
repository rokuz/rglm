#define GLM_FORCE_RADIANS
#define GLM_FORCE_DEPTH_ZERO_TO_ONE

#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstddef>
#include <cstdio>
#include <vector>

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/quaternion.hpp>
#if GLM_CONFIG_ALIGNED_GENTYPES == GLM_ENABLE
#include <glm/gtc/type_aligned.hpp>
#endif

namespace {

constexpr std::size_t BATCH = 256;
constexpr std::size_t WARMUP = 64;
constexpr std::size_t ROUNDS = 1500;
constexpr std::size_t SAMPLES = 15;

#if GLM_CONFIG_ALIGNED_GENTYPES == GLM_ENABLE
using BenchVec3A = glm::aligned_vec3;
using BenchQuatA = glm::aligned_quat;
#else
using BenchVec3A = glm::vec3;
using BenchQuatA = glm::quat;
#endif

template <typename T>
inline void black_box(T& value) {
#if defined(__clang__) || defined(__GNUC__)
    asm volatile("" : "+r,m"(value) : : "memory");
#else
    volatile T* sink = &value;
    (void)sink;
#endif
}

template <typename F>
double measure_ns_per_op(F&& fn) {
    for (std::size_t w = 0; w < WARMUP; ++w) {
        fn();
    }
    double times[SAMPLES];
    for (std::size_t s = 0; s < SAMPLES; ++s) {
        auto start = std::chrono::steady_clock::now();
        for (std::size_t r = 0; r < ROUNDS; ++r) {
            fn();
        }
        auto end = std::chrono::steady_clock::now();
        double ns = static_cast<double>(
            std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count());
        times[s] = ns / static_cast<double>(ROUNDS * BATCH);
    }
    std::sort(times, times + SAMPLES);
    return times[SAMPLES / 2];
}

}

int main() {
    std::vector<glm::mat4> mats_a(BATCH);
    std::vector<glm::mat4> mats_b(BATCH);
    std::vector<glm::mat4> mats_chain(BATCH);
    std::vector<glm::mat3> mats3(BATCH);
    std::vector<glm::vec4> vecs4_a(BATCH);
    std::vector<glm::vec4> vecs4_b(BATCH);
    std::vector<glm::vec3> vecs3(BATCH);
    std::vector<glm::vec3> eyes(BATCH);
    std::vector<glm::vec3> centers(BATCH);
    std::vector<float> fovs(BATCH);
    std::vector<BenchVec3A> vecs3a_a(BATCH);
    std::vector<BenchVec3A> vecs3a_b(BATCH);
    std::vector<glm::quat> quats_a(BATCH);
    std::vector<glm::quat> quats_b(BATCH);
    std::vector<BenchQuatA> quats_aa(BATCH);

    for (std::size_t i = 0; i < BATCH; ++i) {
        float f = static_cast<float>(i) * 0.01f + 0.1f;
        glm::vec3 axis = glm::normalize(glm::vec3(1.0f, 2.0f, 3.0f));
        glm::mat4 m = glm::rotate(
            glm::translate(glm::mat4(1.0f), glm::vec3(f, -f * 0.5f, f * 2.0f)),
            f,
            axis);
        mats_a[i] = m;
        mats_b[i] = glm::transpose(m) + glm::mat4(2.0f);
        mats_chain[i] = glm::rotate(glm::mat4(1.0f), 0.005f + f * 0.001f, axis);
        mats3[i] = glm::mat3(mats_b[i]);
        vecs4_a[i] = glm::vec4(f, f * 0.5f, -f * 0.3f, 1.0f);
        vecs4_b[i] = glm::vec4(0.4f + f * 0.1f, 0.7f - f * 0.05f, -0.5f + f * 0.08f, 0.9f);
        vecs3[i] = glm::vec3(f, -f * 0.7f, 1.5f);
        eyes[i] = glm::vec3(f * 2.0f, 1.5f + f * 0.3f, 5.0f + f);
        centers[i] = glm::vec3(-f * 0.2f, 0.5f, 0.0f);
        fovs[i] = 0.8f + (static_cast<float>(i % 16) * 0.02f);
        vecs3a_a[i] = BenchVec3A(f + 0.2f, -f * 0.7f - 0.1f, 1.5f);
        vecs3a_b[i] = BenchVec3A(-0.6f, 1.1f + f * 0.2f, 0.4f - f * 0.1f);
        glm::quat qa = glm::angleAxis(f, glm::normalize(glm::vec3(0.3f, 0.8f, -0.5f)));
        glm::quat qb = glm::angleAxis(f * 0.7f + 0.2f, glm::normalize(glm::vec3(-0.6f, 0.2f, 0.7f)));
        quats_a[i] = qa;
        quats_b[i] = qb;
        quats_aa[i] = BenchQuatA(qa.w, qa.x, qa.y, qa.z);
    }

    std::vector<glm::mat4> out_m4(BATCH, glm::mat4(0.0f));
    std::vector<glm::mat3> out_m3(BATCH, glm::mat3(0.0f));
    std::vector<glm::vec4> out_v4(BATCH, glm::vec4(0.0f));
    std::vector<glm::vec3> out_v3(BATCH, glm::vec3(0.0f));
    std::vector<BenchVec3A> out_v3a(BATCH, BenchVec3A(0.0f));
    std::vector<glm::quat> out_q(BATCH, glm::quat(1.0f, 0.0f, 0.0f, 0.0f));
    std::vector<float> out_f(BATCH, 0.0f);

    auto emit = [](const char* name, double ns) {
        std::printf("%s\t%.3f\n", name, ns);
    };

    emit("mat4_mul_mat4", measure_ns_per_op([&]() {
        const glm::mat4* a = mats_a.data();
        const glm::mat4* b = mats_b.data();
        glm::mat4* out = out_m4.data();
        black_box(a);
        black_box(b);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = a[i] * b[i];
        }
        black_box(out);
    }));

    emit("mat4_chain", measure_ns_per_op([&]() {
        const glm::mat4* m = mats_chain.data();
        black_box(m);
        glm::mat4 acc = m[0];
        for (std::size_t i = 0; i < BATCH; ++i) {
            acc = acc * m[i];
        }
        black_box(acc);
    }));

    emit("mat4_mul_vec4", measure_ns_per_op([&]() {
        const glm::mat4* m = mats_a.data();
        const glm::vec4* v = vecs4_a.data();
        glm::vec4* out = out_v4.data();
        black_box(m);
        black_box(v);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = m[i] * v[i];
        }
        black_box(out);
    }));

    emit("mat4_inverse", measure_ns_per_op([&]() {
        const glm::mat4* m = mats_b.data();
        glm::mat4* out = out_m4.data();
        black_box(m);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = glm::inverse(m[i]);
        }
        black_box(out);
    }));

    emit("mat4_determinant", measure_ns_per_op([&]() {
        const glm::mat4* m = mats_b.data();
        float* out = out_f.data();
        black_box(m);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = glm::determinant(m[i]);
        }
        black_box(out);
    }));

    emit("mat3_inverse", measure_ns_per_op([&]() {
        const glm::mat3* m = mats3.data();
        glm::mat3* out = out_m3.data();
        black_box(m);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = glm::inverse(m[i]);
        }
        black_box(out);
    }));

    emit("quat_mul_quat", measure_ns_per_op([&]() {
        const glm::quat* a = quats_a.data();
        const glm::quat* b = quats_b.data();
        glm::quat* out = out_q.data();
        black_box(a);
        black_box(b);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = a[i] * b[i];
        }
        black_box(out);
    }));

    emit("quat_mul_vec3", measure_ns_per_op([&]() {
        const glm::quat* q = quats_a.data();
        const glm::vec3* v = vecs3.data();
        glm::vec3* out = out_v3.data();
        black_box(q);
        black_box(v);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = q[i] * v[i];
        }
        black_box(out);
    }));

    emit("quat_mul_vec3a", measure_ns_per_op([&]() {
        const BenchQuatA* q = quats_aa.data();
        const BenchVec3A* v = vecs3a_a.data();
        BenchVec3A* out = out_v3a.data();
        black_box(q);
        black_box(v);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = q[i] * v[i];
        }
        black_box(out);
    }));

    emit("vec3a_geom", measure_ns_per_op([&]() {
        const BenchVec3A* a = vecs3a_a.data();
        const BenchVec3A* b = vecs3a_b.data();
        BenchVec3A* out = out_v3a.data();
        black_box(a);
        black_box(b);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = glm::normalize(glm::cross(a[i], b[i]) + a[i] * glm::dot(a[i], b[i]));
        }
        black_box(out);
    }));

    emit("camera_vp", measure_ns_per_op([&]() {
        const glm::vec3* e = eyes.data();
        const glm::vec3* c = centers.data();
        const float* f = fovs.data();
        glm::mat4* out = out_m4.data();
        black_box(e);
        black_box(c);
        black_box(f);
        glm::vec3 up(0.0f, 1.0f, 0.0f);
        for (std::size_t i = 0; i < BATCH; ++i) {
            glm::mat4 proj = glm::perspectiveRH_ZO(f[i], 16.0f / 9.0f, 0.1f, 1000.0f);
            glm::mat4 view = glm::lookAtRH(e[i], c[i], up);
            out[i] = proj * view;
        }
        black_box(out);
    }));

    emit("vec4_sin_cos", measure_ns_per_op([&]() {
        const glm::vec4* v = vecs4_a.data();
        glm::vec4* out = out_v4.data();
        black_box(v);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = glm::sin(v[i]) + glm::cos(v[i]);
        }
        black_box(out);
    }));

    emit("vec4_exp", measure_ns_per_op([&]() {
        const glm::vec4* v = vecs4_b.data();
        glm::vec4* out = out_v4.data();
        black_box(v);
        for (std::size_t i = 0; i < BATCH; ++i) {
            out[i] = glm::exp(v[i]);
        }
        black_box(out);
    }));

    return 0;
}
