interface IStructName {
    age: number;
    name: string;
}
class StructName extends Protocol.Convertor implements IStructName {

    public age: number;
    public name: string;
    constructor(params: IStructName)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getSignature(): string {
        return 'StructName';
    }
    public getId(): number {
        return 1;
    }
    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBuffer(2, Protocol.ESize.u8, Protocol.Primitives.u8.getSize(), Protocol.Primitives.u8.encode(this.age)),
            () => this.getBufferFromBuf<string>(3, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.name),
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const age: number | Error = this.getValue<number>(storage, 2, Protocol.Primitives.u8.decode);
        if (age instanceof Error) {
            return age;
        } else {
            this.age = age;
        }
        const name: string | Error = this.getValue<string>(storage, 3, Protocol.Primitives.StrUTF8.decode);
        if (name instanceof Error) {
            return name;
        } else {
            this.name = name;
        }
    }
}

interface IOptionA {
    option_a_field_a: string;
    option_a_field_b: string;
}
class OptionA extends Protocol.Convertor implements IOptionA {

    public option_a_field_a: string;
    public option_a_field_b: string;
    constructor(params: IOptionA)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getSignature(): string {
        return 'OptionA';
    }
    public getId(): number {
        return 4;
    }
    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(5, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.option_a_field_a),
            () => this.getBufferFromBuf<string>(6, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.option_a_field_b),
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const option_a_field_a: string | Error = this.getValue<string>(storage, 5, Protocol.Primitives.StrUTF8.decode);
        if (option_a_field_a instanceof Error) {
            return option_a_field_a;
        } else {
            this.option_a_field_a = option_a_field_a;
        }
        const option_a_field_b: string | Error = this.getValue<string>(storage, 6, Protocol.Primitives.StrUTF8.decode);
        if (option_a_field_b instanceof Error) {
            return option_a_field_b;
        } else {
            this.option_a_field_b = option_a_field_b;
        }
    }
}

interface IOptionB {
    option_b_field_a: string;
    option_b_field_b: string;
}
class OptionB extends Protocol.Convertor implements IOptionB {

    public option_b_field_a: string;
    public option_b_field_b: string;
    constructor(params: IOptionB)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getSignature(): string {
        return 'OptionB';
    }
    public getId(): number {
        return 7;
    }
    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<string>(8, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.option_b_field_a),
            () => this.getBufferFromBuf<string>(9, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.option_b_field_b),
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const option_b_field_a: string | Error = this.getValue<string>(storage, 8, Protocol.Primitives.StrUTF8.decode);
        if (option_b_field_a instanceof Error) {
            return option_b_field_a;
        } else {
            this.option_b_field_a = option_b_field_a;
        }
        const option_b_field_b: string | Error = this.getValue<string>(storage, 9, Protocol.Primitives.StrUTF8.decode);
        if (option_b_field_b instanceof Error) {
            return option_b_field_b;
        } else {
            this.option_b_field_b = option_b_field_b;
        }
    }
}

interface IUser {
    username: Array<string>;
    email: string | undefined;
    type: UserType;
    info: StructName;
}
class User extends Protocol.Convertor implements IUser {

    public username: Array<string>;
    public email: string | undefined;
    public type: UserType;
    public info: StructName;
    constructor(params: IUser)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getSignature(): string {
        return 'User';
    }
    public getId(): number {
        return 13;
    }
    public encode(): ArrayBufferLike {
        return this.collect([
            () => this.getBufferFromBuf<Array<string>>(14, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.username),
            () => this.email === undefined ? this.getBuffer(15, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(15, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.email),
            () => { const buffer = this.type.encode(); return this.getBuffer(16, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            () => { const buffer = this.info.encode(); return this.getBuffer(17, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const username: Array<string> | Error = this.getValue<Array<string>>(storage, 14, Protocol.Primitives.ArrayStrUTF8.decode);
        if (username instanceof Error) {
            return username;
        } else {
            this.username = username;
        }
        const emailBuf: ArrayBufferLike | undefined = storage.get(15);
        if (emailBuf === undefined) {
            return new Error(`Fail to get property email`);
        }
        if (emailBuf.byteLength === 0) {
            this.email = undefined;
        } else {
            const email: string | Error = this.getValue<string>(storage, 15, Protocol.Primitives.StrUTF8.decode);
            if (email instanceof Error) {
                return email;
            } else {
                this.email = email;
            }
        }
        const type: UserType = UserType::Defaults;
        const typeBuf: ArrayBufferLike = storage.get(16);
        if (typeBuf instanceof Error) {
            return typeBuf;
        }
        const typeErr: Error | undefined = type.decode(typeBuf);
        if (typeErr instanceof Error) {
            return typeErr;
        } else {
            this.type = type;
        }
        const info: StructName = new StructName({ age: 0, name: '', });
        const infoBuf: ArrayBufferLike = storage.get(17);
        if (infoBuf instanceof Error) {
            return infoBuf;
        }
        const infoErr: Error | undefined = info.decode(infoBuf);
        if (infoErr instanceof Error) {
            return infoErr;
        } else {
            this.info = info;
        }
    }
}

interface ILogin {
    users: Array<User>;
}
class Login extends Protocol.Convertor implements ILogin {

    public users: Array<User>;
    constructor(params: ILogin)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
    }
    public getSignature(): string {
        return 'Login';
    }
    public getId(): number {
        return 18;
    }
    public encode(): ArrayBufferLike {
        return this.collect([
            () => { const buffer = this.users.encode(); return this.getBuffer(19, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const users: User = new User({ username: [], email: undefined, type: UserType::Defaults, info: new StructName({ age: 0, name: '', }), });
        const usersBuf: ArrayBufferLike = storage.get(19);
        if (usersBuf instanceof Error) {
            return usersBuf;
        }
        const usersErr: Error | undefined = users.decode(usersBuf);
        if (usersErr instanceof Error) {
            return usersErr;
        } else {
            this.users = users;
        }
    }
}

export namespace GroupA {

    interface IUserA {
        username: Array<string>;
        email: string | undefined;
        type: UserType;
    }
    class UserA extends Protocol.Convertor implements IUserA {

        public username: Array<string>;
        public email: string | undefined;
        public type: UserType;
        constructor(params: IUserA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }
        public getSignature(): string {
            return 'UserA';
        }
        public getId(): number {
            return 21;
        }
        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<Array<string>>(22, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.username),
                () => this.email === undefined ? this.getBuffer(23, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(23, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.email),
                () => { const buffer = this.type.encode(); return this.getBuffer(24, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }
        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const username: Array<string> | Error = this.getValue<Array<string>>(storage, 22, Protocol.Primitives.ArrayStrUTF8.decode);
            if (username instanceof Error) {
                return username;
            } else {
                this.username = username;
            }
            const emailBuf: ArrayBufferLike | undefined = storage.get(23);
            if (emailBuf === undefined) {
                return new Error(`Fail to get property email`);
            }
            if (emailBuf.byteLength === 0) {
                this.email = undefined;
            } else {
                const email: string | Error = this.getValue<string>(storage, 23, Protocol.Primitives.StrUTF8.decode);
                if (email instanceof Error) {
                    return email;
                } else {
                    this.email = email;
                }
            }
            const type: UserType = UserType::Defaults;
            const typeBuf: ArrayBufferLike = storage.get(24);
            if (typeBuf instanceof Error) {
                return typeBuf;
            }
            const typeErr: Error | undefined = type.decode(typeBuf);
            if (typeErr instanceof Error) {
                return typeErr;
            } else {
                this.type = type;
            }
        }
    }

    interface ILoginA {
        users: Array<User>;
    }
    class LoginA extends Protocol.Convertor implements ILoginA {

        public users: Array<User>;
        constructor(params: ILoginA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }
        public getSignature(): string {
            return 'LoginA';
        }
        public getId(): number {
            return 25;
        }
        public encode(): ArrayBufferLike {
            return this.collect([
                () => { const buffer = this.users.encode(); return this.getBuffer(26, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }
        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const users: User = new User({ username: [], email: undefined, type: UserType::Defaults, info: new StructName({ age: 0, name: '', }), });
            const usersBuf: ArrayBufferLike = storage.get(26);
            if (usersBuf instanceof Error) {
                return usersBuf;
            }
            const usersErr: Error | undefined = users.decode(usersBuf);
            if (usersErr instanceof Error) {
                return usersErr;
            } else {
                this.users = users;
            }
        }
    }

}

export namespace GroupB {

    interface IUserA {
        username: Array<string>;
        email: string | undefined;
        type: UserType;
    }
    class UserA extends Protocol.Convertor implements IUserA {

        public username: Array<string>;
        public email: string | undefined;
        public type: UserType;
        constructor(params: IUserA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }
        public getSignature(): string {
            return 'UserA';
        }
        public getId(): number {
            return 29;
        }
        public encode(): ArrayBufferLike {
            return this.collect([
                () => this.getBufferFromBuf<Array<string>>(30, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.username),
                () => this.email === undefined ? this.getBuffer(31, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(31, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.email),
                () => { const buffer = this.type.encode(); return this.getBuffer(32, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }
        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const username: Array<string> | Error = this.getValue<Array<string>>(storage, 30, Protocol.Primitives.ArrayStrUTF8.decode);
            if (username instanceof Error) {
                return username;
            } else {
                this.username = username;
            }
            const emailBuf: ArrayBufferLike | undefined = storage.get(31);
            if (emailBuf === undefined) {
                return new Error(`Fail to get property email`);
            }
            if (emailBuf.byteLength === 0) {
                this.email = undefined;
            } else {
                const email: string | Error = this.getValue<string>(storage, 31, Protocol.Primitives.StrUTF8.decode);
                if (email instanceof Error) {
                    return email;
                } else {
                    this.email = email;
                }
            }
            const type: UserType = UserType::Defaults;
            const typeBuf: ArrayBufferLike = storage.get(32);
            if (typeBuf instanceof Error) {
                return typeBuf;
            }
            const typeErr: Error | undefined = type.decode(typeBuf);
            if (typeErr instanceof Error) {
                return typeErr;
            } else {
                this.type = type;
            }
        }
    }

    interface ILoginA {
        users: Array<User>;
    }
    class LoginA extends Protocol.Convertor implements ILoginA {

        public users: Array<User>;
        constructor(params: ILoginA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
        }
        public getSignature(): string {
            return 'LoginA';
        }
        public getId(): number {
            return 33;
        }
        public encode(): ArrayBufferLike {
            return this.collect([
                () => { const buffer = this.users.encode(); return this.getBuffer(34, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
            ]);
        }
        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const users: User = new User({ username: [], email: undefined, type: UserType::Defaults, info: new StructName({ age: 0, name: '', }), });
            const usersBuf: ArrayBufferLike = storage.get(34);
            if (usersBuf instanceof Error) {
                return usersBuf;
            }
            const usersErr: Error | undefined = users.decode(usersBuf);
            if (usersErr instanceof Error) {
                return usersErr;
            } else {
                this.users = users;
            }
        }
    }

    export namespace GroupC {

        interface IUserA {
            username: Array<string>;
            email: string | undefined;
            type: UserType;
        }
        class UserA extends Protocol.Convertor implements IUserA {

            public username: Array<string>;
            public email: string | undefined;
            public type: UserType;
            constructor(params: IUserA)  {
                super();
                Object.keys(params).forEach((key: string) => {
                    this[key] = params[key];
                });
            }
            public getSignature(): string {
                return 'UserA';
            }
            public getId(): number {
                return 37;
            }
            public encode(): ArrayBufferLike {
                return this.collect([
                    () => this.getBufferFromBuf<Array<string>>(38, Protocol.ESize.u64, Protocol.Primitives.ArrayStrUTF8.encode, this.username),
                    () => this.email === undefined ? this.getBuffer(39, Protocol.ESize.u8, 0, new Uint8Array()) : this.getBufferFromBuf<string>(39, Protocol.ESize.u64, Protocol.Primitives.StrUTF8.encode, this.email),
                    () => { const buffer = this.type.encode(); return this.getBuffer(40, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                ]);
            }
            public decode(buffer: ArrayBufferLike): Error | undefined {
                const storage = this.getStorage(buffer);
                if (storage instanceof Error) {
                    return storage;
                }
                const username: Array<string> | Error = this.getValue<Array<string>>(storage, 38, Protocol.Primitives.ArrayStrUTF8.decode);
                if (username instanceof Error) {
                    return username;
                } else {
                    this.username = username;
                }
                const emailBuf: ArrayBufferLike | undefined = storage.get(39);
                if (emailBuf === undefined) {
                    return new Error(`Fail to get property email`);
                }
                if (emailBuf.byteLength === 0) {
                    this.email = undefined;
                } else {
                    const email: string | Error = this.getValue<string>(storage, 39, Protocol.Primitives.StrUTF8.decode);
                    if (email instanceof Error) {
                        return email;
                    } else {
                        this.email = email;
                    }
                }
                const type: UserType = UserType::Defaults;
                const typeBuf: ArrayBufferLike = storage.get(40);
                if (typeBuf instanceof Error) {
                    return typeBuf;
                }
                const typeErr: Error | undefined = type.decode(typeBuf);
                if (typeErr instanceof Error) {
                    return typeErr;
                } else {
                    this.type = type;
                }
            }
        }

        interface ILoginA {
            users: Array<User>;
        }
        class LoginA extends Protocol.Convertor implements ILoginA {

            public users: Array<User>;
            constructor(params: ILoginA)  {
                super();
                Object.keys(params).forEach((key: string) => {
                    this[key] = params[key];
                });
            }
            public getSignature(): string {
                return 'LoginA';
            }
            public getId(): number {
                return 41;
            }
            public encode(): ArrayBufferLike {
                return this.collect([
                    () => { const buffer = this.users.encode(); return this.getBuffer(42, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
                ]);
            }
            public decode(buffer: ArrayBufferLike): Error | undefined {
                const storage = this.getStorage(buffer);
                if (storage instanceof Error) {
                    return storage;
                }
                const users: User = new User({ username: [], email: undefined, type: UserType::Defaults, info: new StructName({ age: 0, name: '', }), });
                const usersBuf: ArrayBufferLike = storage.get(42);
                if (usersBuf instanceof Error) {
                    return usersBuf;
                }
                const usersErr: Error | undefined = users.decode(usersBuf);
                if (usersErr instanceof Error) {
                    return usersErr;
                } else {
                    this.users = users;
                }
            }
        }

    }

}

s = users;
                }
            }
        }

    }

}

