#ifndef __MODEL_PLAYER_HPP__
#define __MODEL_PLAYER_HPP__

#include "Stream.hpp"
#include <sstream>
#include <string>

namespace model {

// Player (strategy, client)
class Player {
public:
    // Player's ID
    int id;
    // Current score
    int score;
    // Current amount of resource
    int resource;

    Player(int id, int score, int resource);

    // Read Player from input stream
    static Player readFrom(InputStream& stream);

    // Write Player to output stream
    void writeTo(OutputStream& stream) const;

    // Get string representation of Player
    std::string toString() const;

    bool operator ==(const Player& other) const;
};

}

namespace std {
    template<>
    struct hash<model::Player> {
        size_t operator ()(const model::Player& value) const;
    };
}

#endif