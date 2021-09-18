echo "Install TS lib"
cd ../../lib/typescript
rm -rf ./node_modules
rm package-lock.json
if ! npm install; then
    exit 1
fi

echo "Install TS client transport"
cd ../../environment/transport/client/typescript
rm -rf ./node_modules
rm package-lock.json
if ! npm install; then
    exit 1
fi

echo "Install TS consumer"
cd ../../../../examples/chat/consumer-typescript
rm -rf ./node_modules
rm package-lock.json
if ! npm install; then
    exit 1
fi
