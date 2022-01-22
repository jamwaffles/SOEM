
set(SOURCES akd.c)
add_executable(akd ${SOURCES})
target_link_libraries(akd soem)
install(TARGETS akd DESTINATION bin)
