#!/bin/sh

if [ -f /usr/local/bin/env_vars ]; then
    . /usr/local/bin/env_vars
fi

exec "$@"