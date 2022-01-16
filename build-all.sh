#!/bin/bash
ROOT=$PWD
for d in */ ; do
    if [ "$d" != "pvrustlib/" ] && [ "$d" != "target/" ]; then
        cd "$ROOT/$d"
        if test -f "build.sh"; then
            ./build.sh
            mv *.tar "$ROOT/"
        fi
    fi
done
