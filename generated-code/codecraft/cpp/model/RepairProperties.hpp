#ifndef __MODEL_REPAIR_PROPERTIES_HPP__
#define __MODEL_REPAIR_PROPERTIES_HPP__

#include "../Stream.hpp"
#include "EntityType.hpp"
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

class RepairProperties {
public:
    std::vector<EntityType> validTargets;
    int power;

    RepairProperties();

    RepairProperties(std::vector<EntityType> validTargets, int power);

    static RepairProperties readFrom(InputStream& stream);

    void writeTo(OutputStream& stream) const;

    std::string toString() const;
};

#endif