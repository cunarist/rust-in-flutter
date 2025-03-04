cmake_policy(SET CMP0079 NEW)
apply_standard_settings(${BINARY_NAME})

set_target_properties(
  ${BINARY_NAME} PROPERTIES
  CXX_VISIBILITY_PRESET hidden
  BUILD_RPATH_USE_ORIGIN ON
)

target_compile_definitions(${BINARY_NAME} PRIVATE FLUTTER_PLUGIN_IMPL)
target_link_libraries(${BINARY_NAME} PRIVATE flutter)

include("../cargokit/cmake/cargokit.cmake")
apply_cargokit(${BINARY_NAME} ${CMAKE_SOURCE_DIR}/../native/hub hub "")

set(
  rinf_bundled_libraries
  "${${BINARY_NAME}_cargokit_lib}"
  PARENT_SCOPE
)