#include "RepairProperties.hpp"

namespace model {

RepairProperties::RepairProperties(std::vector<model::EntityType> validTargets, int power) : validTargets(validTargets), power(power) { }

// Read RepairProperties from input stream
RepairProperties RepairProperties::readFrom(InputStream& stream) {
    std::vector<model::EntityType> validTargets = std::vector<model::EntityType>();
    size_t validTargetsSize = stream.readInt();
    validTargets.reserve(validTargetsSize);
    for (size_t validTargetsIndex = 0; validTargetsIndex < validTargetsSize; validTargetsIndex++) {
        model::EntityType validTargetsElement = readEntityType(stream);
        validTargets.emplace_back(validTargetsElement);
    }
    int power = stream.readInt();
    return RepairProperties(validTargets, power);
}

// Write RepairProperties to output stream
void RepairProperties::writeTo(OutputStream& stream) const {
    stream.write((int)(validTargets.size()));
    for (const model::EntityType& validTargetsElement : validTargets) {
        stream.write((int)(validTargetsElement));
    }
    stream.write(power);
}

// Get string representation of RepairProperties
std::string RepairProperties::toString() const {
    std::stringstream ss;
    ss << "RepairProperties { ";
    ss << "validTargets: ";
    ss << "[ ";
    for (size_t validTargetsIndex = 0; validTargetsIndex < validTargets.size(); validTargetsIndex++) {
        const model::EntityType& validTargetsElement = validTargets[validTargetsIndex];
        if (validTargetsIndex != 0) {
            ss << ", ";
        }
        ss << entityTypeToString(validTargetsElement);
    }
    ss << " ]";
    ss << ", ";
    ss << "power: ";
    ss << power;
    ss << " }";
    return ss.str();
}

}