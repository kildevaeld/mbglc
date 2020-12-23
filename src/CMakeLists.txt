
set(SOURCES
    mbglc.cpp
)

set(MBGLC_INCLUDE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/../include>
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)




add_library(mbglc SHARED ${SOURCES})



target_link_libraries(mbglc PRIVATE mbgl-core mbgl-compiler-options)


set_property(TARGET mbglc PROPERTY C_STANDARD 99)


# target_link_libraries(mbglc
#     PRIVATE
#     mbgl-compiler-options mbgl-core
# )

target_include_directories(mbglc PUBLIC ${MBGLC_INCLUDE})
# target_include_directories(mbglc PRIVATE  ${CMAKE_CURRENT_SOURCE_DIR}/../vendor/mapbox/include)



set(include_install_dir ${CMAKE_INSTALL_INCLUDEDIR})
set(lib_install_dir ${CMAKE_INSTALL_LIBDIR})
set(bin_install_dir ${CMAKE_INSTALL_BINDIR})

# add_library(mbglc_shared SHARED $<TARGET_OBJECTS:mbglc>)
# add_library(mbglc_static STATIC $<TARGET_OBJECTS:mbglc>)

# target_link_libraries(mbglc_shared PRIVATE  mbgl-compiler-options mbgl-core ${CMAKE_REQUIRED_LIBRARIES})
# target_link_libraries(mbglc_static PRIVATE mbgl-compiler-options mbgl-core ${CMAKE_REQUIRED_LIBRARIES})

# target_include_directories(mbglc_shared INTERFACE ${MBGLC_INCLUDE})
# target_include_directories(mbglc_static INTERFACE ${MBGLC_INCLUDE})






set(Mbglc_OUTPUT_NAME "mbglc")
set_target_properties(mbglc PROPERTIES
    OUTPUT_NAME ${Mbglc_OUTPUT_NAME}-${version}
    SOVERSION ${SONAME_VERSION_MAJOR}.${SONAME_VERSION_MINOR}
)

set_target_properties(mbglc PROPERTIES 
    OUTPUT_NAME ${Mbglc_OUTPUT_NAME}
)

if (MBGLC_INSTALL)
    set(Mbglc_CMAKE_INSTALL_PATH "${CMAKE_INSTALL_LIBDIR}/cmake/mbglc")
    set(Mbglc_CONFIG_FILE "MbglcConfig.cmake")
    set(Mbglc_CONFIG_VERSION_FILE "MbglcConfigVersion.cmake")

  
    install(
      TARGETS mbglc
      EXPORT MbglcTargets
      ARCHIVE DESTINATION ${lib_install_dir}
      LIBRARY DESTINATION ${lib_install_dir}
      RUNTIME DESTINATION ${bin_install_dir}
      INCLUDES DESTINATION ${include_install_dir})

    install(
      DIRECTORY "${PROJECT_SOURCE_DIR}/include/mbglc"
      DESTINATION ${include_install_dir}
      FILES_MATCHING PATTERN "*.*h")

    install(EXPORT MbglcTargets
            DESTINATION ${Mbglc_CMAKE_INSTALL_PATH}
            EXPORT_LINK_INTERFACE_LIBRARIES
            COMPONENT cmake-config
    )

    include(CMakePackageConfigHelpers)
    configure_package_config_file(
        "${Mbglc_CONFIG_FILE}.in"
        "${CMAKE_CURRENT_BINARY_DIR}/${Mbglc_CONFIG_FILE}"
        INSTALL_DESTINATION ${Mbglc_CMAKE_INSTALL_PATH}
        PATH_VARS include_install_dir lib_install_dir
    )
    write_basic_package_version_file(
        "${CMAKE_CURRENT_BINARY_DIR}/${Mbglc_CONFIG_VERSION_FILE}"
        VERSION ${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}
        COMPATIBILITY AnyNewerVersion)
    install(
        FILES "${CMAKE_CURRENT_BINARY_DIR}/${Mbglc_CONFIG_FILE}"
        "${CMAKE_CURRENT_BINARY_DIR}/${Mbglc_CONFIG_VERSION_FILE}"
        DESTINATION ${Mbglc_CMAKE_INSTALL_PATH}
        COMPONENT cmake-config
    )
endif()
