#ifndef __UCI__
#define __UCI__

#include <string>
#include <vector>

#include "engine.hpp"

namespace uci {

class Uci {
public:
  Uci(engine::Engine& engine);
  void loop();

private:
  engine::Engine engine;
  void parseEngineOption(std::vector<std::string> tokens);
  void parseGo(std::vector<std::string> tokens);
  void parsePosition(std::vector<std::string> tokens);
  bool isFENValid(std::string fen);
};
} // namespace uci
#endif // __UCI__