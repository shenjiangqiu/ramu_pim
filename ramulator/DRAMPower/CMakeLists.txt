# Copyright (c) 2020, Fraunhofer IESE
# All rights reserved.
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are
# met:
#
# 1. Redistributions of source code must retain the above copyright notice,
#    this list of conditions and the following disclaimer.
#
# 2. Redistributions in binary form must reproduce the above copyright
#    notice, this list of conditions and the following disclaimer in the
#    documentation and/or other materials provided with the distribution.
#
# 3. Neither the name of the copyright holder nor the names of its
#    contributors may be used to endorse or promote products derived from
#    this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
# "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
# TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
# PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER
# OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
# EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
# PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
# PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
# LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
# NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
# SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
#
# Author: Matthias Jung

cmake_minimum_required(VERSION 3.10)

# Set the project name:
project(DRAMPower)

# Configuration:
include_directories(
	src/
	src/libdrampower
)

# Build:
add_library(DRAMPower
	src/libdrampower/LibDRAMPower.cc
	src/CommandAnalysis.cc
	src/MemArchitectureSpec.cc
	src/MemCommand.cc
	src/MemPowerSpec.cc
	src/MemTimingSpec.cc
	src/MemoryPowerModel.cc
	src/MemorySpecification.cc
	src/Parameter.cc
	src/Parametrisable.cc
	src/libdrampower/LibDRAMPower.cc
	src/CAHelpers.cc
	src/CmdHandlers.cc
	src/MemBankWiseParams.cc

)
target_include_directories(DRAMPower PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
target_link_libraries(DRAMPower PUBLIC xerces-c)



aux_source_directory(${CMAKE_CURRENT_SOURCE_DIR}/src/xmlparser XMLSOURCE)
add_library(xmlparser ${XMLSOURCE})
target_include_directories(xmlparser PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src/xmlparser)
target_link_libraries(xmlparser PUBLIC DRAMPower)

