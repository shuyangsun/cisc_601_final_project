CMAKE_MINIMUM_REQUIRED (VERSION 3.10)
PROJECT (NumericalIntegration)

# Version Number

SET (NumericalIntegration_VERSION_MAJOR 0)
SET (NumericalIntegration_VERSION_MINOR 1)
SET (NumericalIntegration_VERSION_PATCH 0)

SET(GCC_OPTIMIZATION_FLAGS "-O3")

SET(CPP_STANDARD_COMPILE_FLAGS "-std=c++17")
SET(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} ${GCC_OPTIMIZATION_FLAGS} ${GCC_CONCEPTS_FLAGS}")
ADD_DEFINITIONS(${CPP_STANDARD_COMPILE_FLAGS})

# add the executable
INCLUDE_DIRECTORIES("include/")
ADD_EXECUTABLE (NumericalIntegration src/main.cpp)
TARGET_LINK_LIBRARIES (NumericalIntegration  ${EXTRA_LIBS})

# add the install targets
INSTALL (TARGETS NumericalIntegration DESTINATION bin)
