#ifndef _MODEL_STRUCTURE_HPP_
#define _MODEL_STRUCTURE_HPP_

#include "../Stream.hpp"
#include "Enumeration.hpp"
#include "OneOf.hpp"
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

class Structure {
public:
    std::shared_ptr<OneOf> oneOfOne;
    std::shared_ptr<OneOf> oneOfTwo;
    std::unordered_map<Enumeration, int> hashMap;
    std::string text;
    float floatNumber;
    double doubleNumber;
    Structure();
    Structure(std::shared_ptr<OneOf> oneOfOne, std::shared_ptr<OneOf> oneOfTwo, std::unordered_map<Enumeration, int> hashMap, std::string text, float floatNumber, double doubleNumber);
    static Structure readFrom(InputStream& stream);
    void writeTo(OutputStream& stream) const;
};

#endif
