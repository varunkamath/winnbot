#!/bin/sh
# Read in the file of environment settings
. /usr/local/bin/env_vars
# Then run the CMD
exec "$@"