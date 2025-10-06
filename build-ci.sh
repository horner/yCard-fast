#!/bin/bash
# CI/CD build wrapper - delegates to Makefile (Single Source of Truth)
export BUILD_MODE="${BUILD_MODE:-release}"
exec make ci