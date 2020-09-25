cmake_minimum_required(VERSION 3.0)

find_program(CCACHE_PROGRAM ccache)
if(CCACHE_PROGRAM)
    set_property(GLOBAL PROPERTY RULE_LAUNCH_COMPILE "${CCACHE_PROGRAM}")
endif()

project(mbglc LANGUAGES CXX C)



option(MBGLC_INSTALL "add pistache as install target (recommended)" ON)

set (CMAKE_CXX_STANDARD 14)

include(GNUInstallDirs)


add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/vendor/mapbox EXCLUDE_FROM_ALL)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Werror=sign-compare")



# Set release version...

    # Retrieve from external file...
    file(READ "version.txt" VERSION_FILE)

    # Extract version...

        # Major...
        string(REGEX MATCH "VERSION_MAJOR *([0-9]*)" _ ${VERSION_FILE})
        set(VERSION_MAJOR ${CMAKE_MATCH_1})

        # Minor...
        string(REGEX MATCH "VERSION_MINOR *([0-9]+)" _ ${VERSION_FILE})
        set(VERSION_MINOR ${CMAKE_MATCH_1})

        # Patch...
        string(REGEX MATCH "VERSION_PATCH *([0-9]+)" _ ${VERSION_FILE})
        set(VERSION_PATCH ${CMAKE_MATCH_1})

        # Git date...
        string(REGEX MATCH "VERSION_GIT_DATE *([0-9]+)" _ ${VERSION_FILE})
        set(VERSION_GIT_DATE ${CMAKE_MATCH_1})

    # Make available in a header file...
    configure_file (
        "include/mbglc/version.h.in"
        "include/mbglc/version.h"
        @ONLY
    )

    # Install version header...
    if(MBGLC_INSTALL)
        install (
            FILES
            ${CMAKE_CURRENT_BINARY_DIR}/include/mbglc/version.h
            DESTINATION
            ${CMAKE_INSTALL_INCLUDEDIR}/mbglc/
        )
    endif()


# Set public interface version...

    # Major should be incremented every time a non-backward compatible change
    #  occurs in the ABI...
    string(REGEX MATCH "SONAME_VERSION_MAJOR *([0-9]*)" _ ${VERSION_FILE})
    set(SONAME_VERSION_MAJOR ${CMAKE_MATCH_1})

    # Minor should be incremented every time a backward compatible change occurs
    #  in the ABI...
    string(REGEX MATCH "SONAME_VERSION_MINOR *([0-9]+)" _ ${VERSION_FILE})
    set(SONAME_VERSION_MINOR ${CMAKE_MATCH_1})

# Configure the pkg-config metadata...

    set(LIBS "-lmbglc")

    # Initialize the metadata variables and to support remote builds...
    set(prefix         ${CMAKE_INSTALL_PREFIX})
    set(exec_prefix    ${CMAKE_INSTALL_PREFIX}/bin)
    set(libdir         ${CMAKE_INSTALL_FULL_LIBDIR}/)
    set(libs           ${LIBS})
    set(includedir     ${CMAKE_INSTALL_FULL_INCLUDEDIR}/)
    set(version        ${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}-git${VERSION_GIT_DATE})

    # Perform substitutions...
    configure_file (
        "mbglc.pc.in"
        "mbglc.pc"
        @ONLY
    )

    # Install pkg-config metadata into standard location within the prefix...
    if(MBGLC_INSTALL)
        install (
            FILES
            ${CMAKE_CURRENT_BINARY_DIR}/mbglc.pc
            DESTINATION
            ${CMAKE_INSTALL_LIBDIR}/pkgconfig/
        )
    endif()



add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/src)

add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/example)




# Generate source tarball...

    # Define version of source archive...
    set(CPACK_PACKAGE_VERSION_MAJOR     ${VERSION_MAJOR})
    set(CPACK_PACKAGE_VERSION_MINOR     ${VERSION_MINOR})
    set(CPACK_PACKAGE_VERSION_PATCH     ${VERSION_PATCH})

   
    SET(CPACK_GENERATOR "DEB")
    SET(CPACK_DEBIAN_PACKAGE_MAINTAINER "Rasmus Kildevæld") #required
    #SET(CPACK_DEBIAN_PACKAGE_DEPENDS "")
    # INCLUDE(CPack)
    set (CPACK_DEBIAN_PACKAGE_SHLIBDEPS ON)
    set( CPACK_SOURCE_IGNORE_FILES "/bin/")
    # # Set filename...
    # set(CPACK_SOURCE_PACKAGE_FILE_NAME
    #     "${CMAKE_PROJECT_NAME}-${CPACK_PACKAGE_VERSION_MAJOR}.${CPACK_PACKAGE_VERSION_MINOR}.${CPACK_PACKAGE_VERSION_PATCH}")

    # # Don't include these files in the archive...
    # set(CPACK_SOURCE_IGNORE_FILES
    #   "/build/;/debian/;/prefix/;/.git*;~$;${CPACK_SOURCE_IGNORE_FILES}")

    # Include the CPack module for generating the archive and add the standard
    #  dist target to the generated makefile...
    include(CPack)
    # add_custom_target(dist COMMAND ${CMAKE_MAKE_PROGRAM} package_source)



# uninstall target
if(NOT TARGET uninstall)
  configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/cmake_uninstall.cmake.in"
    "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
    IMMEDIATE @ONLY)

  add_custom_target(uninstall
    COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake)
endif()
