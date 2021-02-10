#include "Color.hpp"

Color::Color(float r, float g, float b, float a) : r(r), g(g), b(b), a(a) { }

// Read Color from input stream
Color Color::readFrom(InputStream& stream) {
    float r = stream.readFloat();
    float g = stream.readFloat();
    float b = stream.readFloat();
    float a = stream.readFloat();
    return Color(r, g, b, a);
}

// Write Color to output stream
void Color::writeTo(OutputStream& stream) const {
    stream.write(r);
    stream.write(g);
    stream.write(b);
    stream.write(a);
}

// Get string representation of Color
std::string Color::toString() const {
    std::stringstream ss;
    ss << "Color { ";
    ss << "r: ";
    ss << r;
    ss << ", ";
    ss << "g: ";
    ss << g;
    ss << ", ";
    ss << "b: ";
    ss << b;
    ss << ", ";
    ss << "a: ";
    ss << a;
    ss << " }";
    return ss.str();
}