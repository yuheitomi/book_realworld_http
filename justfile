default:
    just --list

test_01:
    curl --http1.0 --data-urlencode title="Real World HTTP" \
    --data-urlencode desc="Sample & Tutorial" http://localhost:18888

test_02:
    curl --http1.0 --compressed \
    -F title="The Art of Commuinity" \
    -F author="John" -F attachment-file=@justfile http://localhost:18888

test_03:
    curl --http1.0 --basic -u user:pass http://localhost:18888

