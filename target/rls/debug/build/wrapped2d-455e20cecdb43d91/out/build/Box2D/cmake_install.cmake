# Install script for directory: /Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/hiroko/rl/gymnasium/target/rls/debug/build/wrapped2d-455e20cecdb43d91/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/Box2D" TYPE FILE FILES "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Box2D.h")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/Box2D/Collision" TYPE FILE FILES
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Collision/b2BroadPhase.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Collision/b2Collision.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Collision/b2Distance.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Collision/b2DynamicTree.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Collision/b2TimeOfImpact.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/Box2D/Collision/Shapes" TYPE FILE FILES
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Collision/Shapes/b2CircleShape.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Collision/Shapes/b2EdgeShape.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Collision/Shapes/b2ChainShape.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Collision/Shapes/b2PolygonShape.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Collision/Shapes/b2Shape.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/Box2D/Common" TYPE FILE FILES
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Common/b2BlockAllocator.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Common/b2Draw.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Common/b2GrowableStack.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Common/b2Math.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Common/b2Settings.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Common/b2StackAllocator.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Common/b2Timer.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/Box2D/Dynamics" TYPE FILE FILES
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/b2Body.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/b2ContactManager.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/b2Fixture.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/b2Island.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/b2TimeStep.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/b2World.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/b2WorldCallbacks.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/Box2D/Dynamics/Contacts" TYPE FILE FILES
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Contacts/b2CircleContact.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Contacts/b2Contact.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Contacts/b2ContactSolver.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Contacts/b2PolygonAndCircleContact.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Contacts/b2EdgeAndCircleContact.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Contacts/b2EdgeAndPolygonContact.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Contacts/b2ChainAndCircleContact.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Contacts/b2ChainAndPolygonContact.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Contacts/b2PolygonContact.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/Box2D/Dynamics/Joints" TYPE FILE FILES
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2DistanceJoint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2FrictionJoint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2GearJoint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2Joint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2MotorJoint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2MouseJoint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2PrismaticJoint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2PulleyJoint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2RevoluteJoint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2RopeJoint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2WeldJoint.h"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Dynamics/Joints/b2WheelJoint.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/Box2D/Rope" TYPE FILE FILES "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/Rope/b2Rope.h")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/Users/hiroko/rl/gymnasium/target/rls/debug/build/wrapped2d-455e20cecdb43d91/out/build/Box2D/libBox2D.a")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libBox2D.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libBox2D.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libBox2D.a")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/Box2D/Box2D-targets.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/Box2D/Box2D-targets.cmake"
         "/Users/hiroko/rl/gymnasium/target/rls/debug/build/wrapped2d-455e20cecdb43d91/out/build/Box2D/CMakeFiles/Export/lib/Box2D/Box2D-targets.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/Box2D/Box2D-targets-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/Box2D/Box2D-targets.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/Box2D" TYPE FILE FILES "/Users/hiroko/rl/gymnasium/target/rls/debug/build/wrapped2d-455e20cecdb43d91/out/build/Box2D/CMakeFiles/Export/lib/Box2D/Box2D-targets.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/Box2D" TYPE FILE FILES "/Users/hiroko/rl/gymnasium/target/rls/debug/build/wrapped2d-455e20cecdb43d91/out/build/Box2D/CMakeFiles/Export/lib/Box2D/Box2D-targets-debug.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/Box2D" TYPE FILE FILES
    "/Users/hiroko/rl/gymnasium/target/rls/debug/build/wrapped2d-455e20cecdb43d91/out/build/Box2D/Box2DConfig.cmake"
    "/Users/hiroko/.cargo/registry/src/github.com-1ecc6299db9ec823/wrapped2d-0.4.1/Box2D/Box2D/Box2D/UseBox2D.cmake"
    )
endif()

