#!/bin/bash

PREFIX_DIR="$(dirname $(readlink -f "$0"))/prefix"

WINEPREFIX="${PREFIX_DIR}" WINEARCH="win32" wine "${PREFIX_DIR}/drive_c/Program Files/Messy Lab/MessyLab.exe"