# Compiler & Flags
CXX      := g++
# -MMD -MP generates dependency files (.d) for tracked headers
CXXFLAGS := -std=c++17 -Wall -Wextra -Wpedantic -MMD -MP -Iinclude
LDFLAGS  := 

# Optimization Flags
OPT1 := -O1
OPT2 := -O2

# Directories
SRC_DIR   := src
BUILD_DIR := build
TARGET    := anji

# Find all .cpp files recursively in src/ and map them to .o and .d files in build/
SRCS := $(shell find $(SRC_DIR) -type f -name "*.cpp" 2>/dev/null)
OBJS := $(SRCS:$(SRC_DIR)/%.cpp=$(BUILD_DIR)/%.o)
DEPS := $(OBJS:.o=.d)

# Phony targets
.PHONY: all release clean

# Default target: Build with Optimization Level 1 (-O1)
all: CXXFLAGS += $(OPT1)
all: $(TARGET)

# Release target: Cleans output first, then builds with Optimization Level 2 (-O2)
release: clean
release: CXXFLAGS += $(OPT2)
release: $(TARGET)

# Link executable
$(TARGET): $(OBJS)
	@mkdir -p $(dir $@)
	$(CXX) $(OBJS) -o $@ $(LDFLAGS)

# Compile C++ source files into object files
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Include auto-generated dependency files
-include $(DEPS)

# Clean target
clean:
	rm -rf $(BUILD_DIR) $(TARGET)