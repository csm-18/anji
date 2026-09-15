#include "cli/cli.hpp"

void run(std::vector<std::string> &args) {
  if (args.size() > 0)
    std::cout << args[0] << "\n";
}