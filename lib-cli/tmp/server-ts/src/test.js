function success(label) {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            console.log(`${label}: resolved`);
            resolve();
        }, 150);
    });
}
function fail(label) {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            console.log(`${label}: rejected`);
            reject();
        }, 150);
    });

}
function testA() {
    return new Promise((resolve, reject) => {
        return success(`Test A`).then(resolve).catch(reject);
    });
}
function testB() {
    return new Promise((resolve, reject) => {
        return success(`Test B`).then(() => {
            return fail(`TestB`);
        });
    });
}

function testC() {
    return new Promise((resolve, reject) => {
        success(`Test C`).then(() => {
            return fail(`TestC`);
        });
 
    });
}
function testD() {
    return success(`Test D`).then(() => {
        return success(`Test D`).then(() => {
            return success(`Test D1`).then(() => {
                return Promise.reject(new Error(`ugu`));
            });
        });
    });
}

testA().then(() => {
    console.log(`TestA: finish - resolve`);
}).catch(() => {
    console.log(`TestA: finish - reject`);
});
// testB().then(() => {
//     console.log(`TestB: finish - resolve`);
// }).catch(() => {
//     console.log(`TestB: finish - reject`);
// });
// testC().then(() => {
//     console.log(`TestC: finish - resolve`);
// }).catch(() => {
//     console.log(`TestC: finish - reject`);
// });
testD().then(() => {
    console.log(`TestD: finish - resolve`);
}).catch((err) => {
    console.log(`TestD: finish - reject: ${err.message}`);
});

setTimeout(() => {
    console.log('done');
}, 1000);