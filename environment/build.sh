echo "Build protocol"
cd ./protocol
if ! sh build.sh; then
    exit 1
fi
cd ..

echo "Build TS lib"
cd ../lib/typescript
npm install
if ! npm run build; then
    exit 1
fi

echo "Build TS client transport"
cd ../../environment/transport/client/typescript
npm install
if ! npm run build; then
    exit 1
fi

echo "Build TS consumer"
cd ../../../consumer/typescript
npm install
if ! npm run build; then
    exit 1
fi
