Minimal config files to expose the wasm build with the correct headers set up.

Make sure the path to the build in `docker-compose` is correct, and run:

```
docker-compose up -d
```

If you reuse this, make sure the path to your .html file is present in `try_files` (in `nginx.conf`)