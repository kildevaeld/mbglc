

set(SOURCES
    main.c
)

add_executable(mbgl main.c)
target_link_libraries(mbgl mbglc)