#include <iostream>
#include <vector>

#include "cli/cli.hpp"

int main(int argc, char *argv[]) {
  std::vector<std::string> args;
  for (int x = 1; x < argc; x += 1) {
    args.push_back(argv[x]);
  }

  run(args);

  return 0;
}
