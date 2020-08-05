#ifndef NUMERICAL_INTEGRATION_LIB_HPP_
#define NUMERICAL_INTEGRATION_LIB_HPP_

#include <cmath>
#include <utility>
#include <unordered_map>


namespace numerical_integration {

    struct hash_pair {
        template <class T1, class T2>
        size_t operator()(const std::pair<T1, T2>& p) const
        {
            auto hash1 = std::hash<T1>{}(p.first);
            auto hash2 = std::hash<T2>{}(p.second);
            return hash1 ^ hash2;
        }
    };

    template<typename T>
    using RombergCacheMap = std::unordered_map<std::pair<size_t, size_t>, T, hash_pair>;

    template<typename T>
    inline T trapezoidal(T (* const func)(T const), size_t const n, T const left, T const right) {
        T res{ (func(left) + func(right)) / T(2.0) };
        T const delta{ (right - left) / n };
        T cur{ left + delta };
        while (cur < right) {
            res += func(cur);
            cur += delta;
        }
        return res * delta;
    }

    template<typename T>
    inline T simpson_1_3(T (* const func)(T const), size_t const n, T const left, T const right) {
        T res{ (func(left) + func(right)) / T(2.0) };
        T const delta{ (right - left) / n };
        T cur{ left + delta };
        for (size_t i = 0; cur < right; ++i) {
            res += func(cur) * (i % 2 == 0 ? 4.0 : 2.0);
            cur += delta;
        }
        return res * delta / 3.0;
    }

    template<typename T>
    inline T simpson_3_8(T (* const func)(T const), size_t const n, T const left, T const right) {
        T res{ (func(left) + func(right)) / T(2.0) };
        T const delta{ (right - left) / n };
        T cur{ left + delta };
        for (size_t i = 0; cur < right; ++i) {
            res += func(cur) * ((i + 1) % 3 == 0 ? 2.0 : 3.0);
            cur += delta;
        }
        return res * delta * (3.0 / 8.0);
    }

    template<typename T>
    class Romberg {
    public:
        Romberg(T (* const func)(T const), T const left, T const right) {
            func_ = func;
            left_ = left;
            right_ = right;
            cache_ = {};
        }

        inline T calculate(size_t const j, size_t const k) {
            if (k <= 0) {
                const auto zero_pair{ std::make_pair(0, 0) };
                const auto res{ cache_.find(zero_pair) };
                if (res == cache_.end()) {
                    const auto trapezoidal_res { trapezoidal(func_, pow(2, j), left_, right_) };
                    cache_.insert({zero_pair, trapezoidal_res});
                    return trapezoidal_res;
                }
                return res->second;
            }
            const auto jk_pair{ std::make_pair(j, k) };
            const auto res{ cache_.find(jk_pair) };
            if (res == cache_.end()) {
                T const four_k{ pow(4, k) };
                const auto cur_res{ (four_k * calculate(j, k - 1) - calculate(j - 1, k - 1)) / (four_k - 1) };
                cache_.insert({jk_pair, cur_res});
                return cur_res;
            }
            return res->second;
        }

        ~Romberg() = default;
    private:
        T (*func_)(T const);
        T left_;
        T right_;
        RombergCacheMap<T> cache_;
    };
}


#endif  // NUMERICAL_INTEGRATION_LIB_HPP_
