#include "LIEF/LIEF.hpp"
#include "rust/cxx.h"

#include <iostream>

class Binary {
private:
    std::unique_ptr<LIEF::ELF::Binary> impl;

public:
    Binary(std::unique_ptr<LIEF::ELF::Binary> impl) : impl(std::move(impl)) {}

    void add_library(rust::Str name) const {
        impl->add_library(std::string(name));
    }

    bool has_library(rust::Str name) const {
        return impl->has_library(std::string(name));
    }

    auto build() const {
        auto data = impl->raw();
        rust::Vec<uint8_t> vec;
        std::copy(data.begin(), data.end(), std::back_inserter(vec));
        return vec;
    }
};

std::unique_ptr<Binary> load_binary(rust::Slice<const uint8_t> data)
{
    std::vector<uint8_t> data_vec(data.data(), data.data() + data.length());
    auto binary = LIEF::ELF::Parser::parse(data_vec);
    if (binary == nullptr) throw std::runtime_error("invalid elf");
    return std::make_unique<Binary>(std::move(binary));
}

void test() {
    std::cout << "Hello world" << "\n";
}
