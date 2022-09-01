#include <array>
#include <cstdint>
#include <string>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

namespace {
template <typename T>
class impl;
} // namespace

class String;

#ifndef CXXBRIDGE1_RUST_STR
#define CXXBRIDGE1_RUST_STR
class Str final {
public:
  Str() noexcept;
  Str(const String &) noexcept;
  Str(const std::string &);
  Str(const char *);
  Str(const char *, std::size_t);

  Str &operator=(const Str &) &noexcept = default;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  Str(const Str &) noexcept = default;
  ~Str() noexcept = default;

  using iterator = const char *;
  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const Str &) const noexcept;
  bool operator!=(const Str &) const noexcept;
  bool operator<(const Str &) const noexcept;
  bool operator<=(const Str &) const noexcept;
  bool operator>(const Str &) const noexcept;
  bool operator>=(const Str &) const noexcept;

  void swap(Str &) noexcept;

private:
  class uninit;
  Str(uninit) noexcept;
  friend impl<Str>;

  std::array<std::uintptr_t, 2> repr;
};
#endif // CXXBRIDGE1_RUST_STR
} // namespace cxxbridge1
} // namespace rust

extern "C" {
void cxxbridge1$init_logger() noexcept;

void cxxbridge1$rdebug(::rust::Str msg) noexcept;

void cxxbridge1$rinfo(::rust::Str msg) noexcept;

void cxxbridge1$rerror(::rust::Str msg) noexcept;

void cxxbridge1$rdebug_with_target(::rust::Str target, ::rust::Str msg) noexcept;

void cxxbridge1$rinfo_with_target(::rust::Str target, ::rust::Str msg) noexcept;

void cxxbridge1$rerror_with_target(::rust::Str target, ::rust::Str msg) noexcept;
} // extern "C"

void init_logger() noexcept {
  cxxbridge1$init_logger();
}

void rdebug(::rust::Str msg) noexcept {
  cxxbridge1$rdebug(msg);
}

void rinfo(::rust::Str msg) noexcept {
  cxxbridge1$rinfo(msg);
}

void rerror(::rust::Str msg) noexcept {
  cxxbridge1$rerror(msg);
}

void rdebug_with_target(::rust::Str target, ::rust::Str msg) noexcept {
  cxxbridge1$rdebug_with_target(target, msg);
}

void rinfo_with_target(::rust::Str target, ::rust::Str msg) noexcept {
  cxxbridge1$rinfo_with_target(target, msg);
}

void rerror_with_target(::rust::Str target, ::rust::Str msg) noexcept {
  cxxbridge1$rerror_with_target(target, msg);
}
