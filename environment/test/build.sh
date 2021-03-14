echo "Build environment"
cd ..
if ! sh build.sh; then
    exit 1
fi
cd ./test

echo "Build Case A"
cd ./case-a/consumer-ts
npm install
rm -rf ./dist
if ! npm run build; then
    exit 1
fi