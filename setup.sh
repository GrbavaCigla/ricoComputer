#!/bin/bash

BASE_DIR=$(dirname $(readlink -f "$0"))

WINEPREFIX="${BASE_DIR}/prefix" WINEARCH="win32" wine wineboot -i
WINEPREFIX="${BASE_DIR}/prefix" WINEARCH="win32" winetricks dotnet6
WINEPREFIX="${BASE_DIR}/prefix" WINEARCH="win32" wine "MessyLab-1.01.exe"