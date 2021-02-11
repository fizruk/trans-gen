#ifndef __MODEL_ENTITY_PROPERTIES_HPP__
#define __MODEL_ENTITY_PROPERTIES_HPP__

#include "Stream.hpp"
#include "model/AttackProperties.hpp"
#include "model/BuildProperties.hpp"
#include "model/EntityType.hpp"
#include "model/RepairProperties.hpp"
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace model {

// Entity properties
class EntityProperties {
public:
    // Size. Entity has a form of a square with side of this length
    int size;
    // Score for building this entity
    int buildScore;
    // Score for destroying this entity
    int destroyScore;
    // Whether this entity can move
    bool canMove;
    // Number of population points this entity provides, if active
    int populationProvide;
    // Number of population points this entity uses
    int populationUse;
    // Maximum health points
    int maxHealth;
    // Cost to build this first entity of this type. If this is a unit (entity can move), the cost is increased by 1 for each existing unit of this type
    int initialCost;
    // If fog of war is enabled, maximum distance at which other entities are considered visible
    int sightRange;
    // Amount of resource added to enemy able to collect resource on dealing damage for 1 health point
    int resourcePerHealth;
    // Build properties, if entity can build
    std::optional<model::BuildProperties> build;
    // Attack properties, if entity can attack
    std::optional<model::AttackProperties> attack;
    // Repair properties, if entity can repair
    std::optional<model::RepairProperties> repair;

    EntityProperties(int size, int buildScore, int destroyScore, bool canMove, int populationProvide, int populationUse, int maxHealth, int initialCost, int sightRange, int resourcePerHealth, std::optional<model::BuildProperties> build, std::optional<model::AttackProperties> attack, std::optional<model::RepairProperties> repair);

    // Read EntityProperties from input stream
    static EntityProperties readFrom(InputStream& stream);

    // Write EntityProperties to output stream
    void writeTo(OutputStream& stream) const;

    // Get string representation of EntityProperties
    std::string toString() const;
};

}

#endif