default:
    just --list

test_01:
    curl --http1.0 --data-urlencode title="Real World HTTP" --data-urlencode desc="Sample & Tutorial" http://localhost:18888

