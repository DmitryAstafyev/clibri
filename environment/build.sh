echo "Build protocol"
cd ./protocol
if ! sh build.sh; then
    exit 1
fi
cd ..

echo "Build TS lib"
cd ../lib/typescript
rm -rf ./dist
if ! npm run build; then
    exit 1
fi

echo "Build TS client transport"
cd ../../environment/transport/client/typescript
rm -rf ./dist
if ! npm run build; then
    exit 1
fi

echo "Build TS consumer"
cd ../../../consumer/typescript
rm -rf ./dist
if ! npm run build; then
    exit 1
fi

echo "Build RS Producer"
cd ../../producer/rust
if ! cargo build; then
    exit 1
fi
