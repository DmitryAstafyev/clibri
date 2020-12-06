import * as Protocol from '@fiber/protocol';
import { Primitives } from '@fiber/protocol';
interface EnumWithSctructs {
    a?: OptionA;
    b?: OptionB;
}

interface SyntaxSugarEnum {
    VariantA?: string;
    VariantB?: string;
    VariantC?: string;
}

interface UserType {
    pointA?: Array<number>;
    pointB?: string;
    pointC?: number;
}

interface IStructName {
    age: number;
    name: string;
}
class StructName extends Protocol.Convertor implements IStructName {

    public static defaults(): StructName {
        return new StructName({ 
            age: 0,
            name: '',
        });
    }
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
    public defaults(): StructName {
        return StructName.defaults();
    }
}

interface IOptionA {
    option_a_field_a: string;
    option_a_field_b: string;
}
class OptionA extends Protocol.Convertor implements IOptionA {

    public static defaults(): OptionA {
        return new OptionA({ 
            option_a_field_a: '',
            option_a_field_b: '',
        });
    }
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
    public defaults(): OptionA {
        return OptionA.defaults();
    }
}

interface IOptionB {
    option_b_field_a: string;
    option_b_field_b: string;
}
class OptionB extends Protocol.Convertor implements IOptionB {

    public static defaults(): OptionB {
        return new OptionB({ 
            option_b_field_a: '',
            option_b_field_b: '',
        });
    }
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
    public defaults(): OptionB {
        return OptionB.defaults();
    }
}

interface IUser {
    username: Array<string>;
    email: string | undefined;
    type: UserType;
    info: StructName;
}
class User extends Protocol.Convertor implements IUser {

    public static defaults(): User {
        return new User({ 
            username: [],
            email: undefined,
            type: {},
            info: new StructName({ 
                age: 0,
                name: '',
            }),
        });
    }
    public username: Array<string>;
    public email: string | undefined;
    public type: UserType;
    public info: StructName;
    private _type: Primitives.Enum;
    constructor(params: IUser)  {
        super();
        Object.keys(params).forEach((key: string) => {
            this[key] = params[key];
        });
        this._type = new Primitives.Enum([
            Protocol.Primitives.ArrayU8.getSignature(),
            Protocol.Primitives.StrUTF8.getSignature(),
            Protocol.Primitives.u16.getSignature(),
        ], (id: number): ISigned<any> | undefined => {
            switch (id) {
                case 0: return new Protocol.Primitives.ArrayU8([0]);
                case 1: return new Protocol.Primitives.StrUTF8('');
                case 2: return new Protocol.Primitives.u16(0);
            }
        });
        if (Object.keys(this.type).length > 1) {
            throw new Error(`Option cannot have more then 1 value. Property "type" or class "User"`);
        }
        if (this.type.pointA !== undefined) {
            const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<Array<number>>(0, new Protocol.Primitives.ArrayU8(this.type.pointA)));
            if (err instanceof Error) {
                throw err;
            }
        }
        if (this.type.pointB !== undefined) {
            const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(this.type.pointB)));
            if (err instanceof Error) {
                throw err;
            }
        }
        if (this.type.pointC !== undefined) {
            const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(this.type.pointC)));
            if (err instanceof Error) {
                throw err;
            }
        }
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
            () => { const buffer = this._type.encode(); return this.getBuffer(16, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
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
        this.type = {};
        const typeBuf: ArrayBufferLike = storage.get(16);
        if (typeBuf === undefined) {
            return new Error(`Fail to get property "type"`);
        }
        if (typeBuf.byteLength > 0) {
            const typeErr: Error | undefined = this._type.decode(typeBuf);
            if (typeErr instanceof Error) {
                return typeErr;
            } else {
                switch (this._type.getValueIndex()) {
                    case 0: this.type.pointA = this._type.get<Array<number>>(); break;
                    case 1: this.type.pointB = this._type.get<string>(); break;
                    case 2: this.type.pointC = this._type.get<number>(); break;
                }
            }
        }
        const info: StructName = new StructName({ 
            age: 0,
            name: '',
        });
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
    public defaults(): User {
        return User.defaults();
    }
}

interface ILogin {
    users: Array<User>;
}
class Login extends Protocol.Convertor implements ILogin {

    public static defaults(): Login {
        return new Login({ 
            users: [],
        });
    }
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
            () => { const self: User = User.defaults(); return this.getBufferFromBuf<User[]>(19, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.users); },
        ]);
    }
    public decode(buffer: ArrayBufferLike): Error | undefined {
        const storage = this.getStorage(buffer);
        if (storage instanceof Error) {
            return storage;
        }
        const arrUserInst: User = User.defaults();
        const arrUser: Array<any> | Error = this.getValue<User[]>(storage, 19, arrUserInst.decodeSelfArray.bind(arrUserInst));
        if (arrUser instanceof Error) {
            return arrUser;
        } else {
            this.users = arrUser as User[];
        }
    }
    public defaults(): Login {
        return Login.defaults();
    }
}

export namespace GroupA {

    interface UserTypeTest {
        pointA?: number;
        pointB?: number;
        pointC?: number;
    }

    interface IUserA {
        username: Array<string>;
        email: string | undefined;
        type: UserType;
    }
    class UserA extends Protocol.Convertor implements IUserA {

        public static defaults(): UserA {
            return new UserA({ 
                username: [],
                email: undefined,
                type: {},
            });
        }
        public username: Array<string>;
        public email: string | undefined;
        public type: UserType;
        private _type: Primitives.Enum;
        constructor(params: IUserA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
            this._type = new Primitives.Enum([
                Protocol.Primitives.ArrayU8.getSignature(),
                Protocol.Primitives.StrUTF8.getSignature(),
                Protocol.Primitives.u16.getSignature(),
            ], (id: number): ISigned<any> | undefined => {
                switch (id) {
                    case 0: return new Protocol.Primitives.ArrayU8([0]);
                    case 1: return new Protocol.Primitives.StrUTF8('');
                    case 2: return new Protocol.Primitives.u16(0);
                }
            });
            if (Object.keys(this.type).length > 1) {
                throw new Error(`Option cannot have more then 1 value. Property "type" or class "UserA"`);
            }
            if (this.type.pointA !== undefined) {
                const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<Array<number>>(0, new Protocol.Primitives.ArrayU8(this.type.pointA)));
                if (err instanceof Error) {
                    throw err;
                }
            }
            if (this.type.pointB !== undefined) {
                const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(this.type.pointB)));
                if (err instanceof Error) {
                    throw err;
                }
            }
            if (this.type.pointC !== undefined) {
                const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(this.type.pointC)));
                if (err instanceof Error) {
                    throw err;
                }
            }
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
                () => { const buffer = this._type.encode(); return this.getBuffer(24, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
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
            this.type = {};
            const typeBuf: ArrayBufferLike = storage.get(24);
            if (typeBuf === undefined) {
                return new Error(`Fail to get property "type"`);
            }
            if (typeBuf.byteLength > 0) {
                const typeErr: Error | undefined = this._type.decode(typeBuf);
                if (typeErr instanceof Error) {
                    return typeErr;
                } else {
                    switch (this._type.getValueIndex()) {
                        case 0: this.type.pointA = this._type.get<Array<number>>(); break;
                        case 1: this.type.pointB = this._type.get<string>(); break;
                        case 2: this.type.pointC = this._type.get<number>(); break;
                    }
                }
            }
        }
        public defaults(): UserA {
            return UserA.defaults();
        }
    }

    interface ILoginA {
        users: Array<User>;
    }
    class LoginA extends Protocol.Convertor implements ILoginA {

        public static defaults(): LoginA {
            return new LoginA({ 
                users: [],
            });
        }
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
                () => { const self: User = User.defaults(); return this.getBufferFromBuf<User[]>(26, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.users); },
            ]);
        }
        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const arrUserInst: User = User.defaults();
            const arrUser: Array<any> | Error = this.getValue<User[]>(storage, 26, arrUserInst.decodeSelfArray.bind(arrUserInst));
            if (arrUser instanceof Error) {
                return arrUser;
            } else {
                this.users = arrUser as User[];
            }
        }
        public defaults(): LoginA {
            return LoginA.defaults();
        }
    }

}

export namespace GroupB {

    interface UserTypeTest {
        pointA?: number;
        pointB?: number;
        pointC?: number;
    }

    interface IUserA {
        username: Array<string>;
        email: string | undefined;
        type: UserType;
    }
    class UserA extends Protocol.Convertor implements IUserA {

        public static defaults(): UserA {
            return new UserA({ 
                username: [],
                email: undefined,
                type: {},
            });
        }
        public username: Array<string>;
        public email: string | undefined;
        public type: UserType;
        private _type: Primitives.Enum;
        constructor(params: IUserA)  {
            super();
            Object.keys(params).forEach((key: string) => {
                this[key] = params[key];
            });
            this._type = new Primitives.Enum([
                Protocol.Primitives.ArrayU8.getSignature(),
                Protocol.Primitives.StrUTF8.getSignature(),
                Protocol.Primitives.u16.getSignature(),
            ], (id: number): ISigned<any> | undefined => {
                switch (id) {
                    case 0: return new Protocol.Primitives.ArrayU8([0]);
                    case 1: return new Protocol.Primitives.StrUTF8('');
                    case 2: return new Protocol.Primitives.u16(0);
                }
            });
            if (Object.keys(this.type).length > 1) {
                throw new Error(`Option cannot have more then 1 value. Property "type" or class "UserA"`);
            }
            if (this.type.pointA !== undefined) {
                const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<Array<number>>(0, new Protocol.Primitives.ArrayU8(this.type.pointA)));
                if (err instanceof Error) {
                    throw err;
                }
            }
            if (this.type.pointB !== undefined) {
                const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(this.type.pointB)));
                if (err instanceof Error) {
                    throw err;
                }
            }
            if (this.type.pointC !== undefined) {
                const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(this.type.pointC)));
                if (err instanceof Error) {
                    throw err;
                }
            }
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
                () => { const buffer = this._type.encode(); return this.getBuffer(32, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
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
            this.type = {};
            const typeBuf: ArrayBufferLike = storage.get(32);
            if (typeBuf === undefined) {
                return new Error(`Fail to get property "type"`);
            }
            if (typeBuf.byteLength > 0) {
                const typeErr: Error | undefined = this._type.decode(typeBuf);
                if (typeErr instanceof Error) {
                    return typeErr;
                } else {
                    switch (this._type.getValueIndex()) {
                        case 0: this.type.pointA = this._type.get<Array<number>>(); break;
                        case 1: this.type.pointB = this._type.get<string>(); break;
                        case 2: this.type.pointC = this._type.get<number>(); break;
                    }
                }
            }
        }
        public defaults(): UserA {
            return UserA.defaults();
        }
    }

    interface ILoginA {
        users: Array<User>;
    }
    class LoginA extends Protocol.Convertor implements ILoginA {

        public static defaults(): LoginA {
            return new LoginA({ 
                users: [],
            });
        }
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
                () => { const self: User = User.defaults(); return this.getBufferFromBuf<User[]>(34, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.users); },
            ]);
        }
        public decode(buffer: ArrayBufferLike): Error | undefined {
            const storage = this.getStorage(buffer);
            if (storage instanceof Error) {
                return storage;
            }
            const arrUserInst: User = User.defaults();
            const arrUser: Array<any> | Error = this.getValue<User[]>(storage, 34, arrUserInst.decodeSelfArray.bind(arrUserInst));
            if (arrUser instanceof Error) {
                return arrUser;
            } else {
                this.users = arrUser as User[];
            }
        }
        public defaults(): LoginA {
            return LoginA.defaults();
        }
    }

    export namespace GroupC {

        interface UserTypeTest {
            pointA?: number;
            pointB?: number;
            pointC?: number;
        }

        interface IUserA {
            username: Array<string>;
            email: string | undefined;
            type: UserType;
        }
        class UserA extends Protocol.Convertor implements IUserA {

            public static defaults(): UserA {
                return new UserA({ 
                    username: [],
                    email: undefined,
                    type: {},
                });
            }
            public username: Array<string>;
            public email: string | undefined;
            public type: UserType;
            private _type: Primitives.Enum;
            constructor(params: IUserA)  {
                super();
                Object.keys(params).forEach((key: string) => {
                    this[key] = params[key];
                });
                this._type = new Primitives.Enum([
                    Protocol.Primitives.ArrayU8.getSignature(),
                    Protocol.Primitives.StrUTF8.getSignature(),
                    Protocol.Primitives.u16.getSignature(),
                ], (id: number): ISigned<any> | undefined => {
                    switch (id) {
                        case 0: return new Protocol.Primitives.ArrayU8([0]);
                        case 1: return new Protocol.Primitives.StrUTF8('');
                        case 2: return new Protocol.Primitives.u16(0);
                    }
                });
                if (Object.keys(this.type).length > 1) {
                    throw new Error(`Option cannot have more then 1 value. Property "type" or class "UserA"`);
                }
                if (this.type.pointA !== undefined) {
                    const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<Array<number>>(0, new Protocol.Primitives.ArrayU8(this.type.pointA)));
                    if (err instanceof Error) {
                        throw err;
                    }
                }
                if (this.type.pointB !== undefined) {
                    const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<string>(1, new Protocol.Primitives.StrUTF8(this.type.pointB)));
                    if (err instanceof Error) {
                        throw err;
                    }
                }
                if (this.type.pointC !== undefined) {
                    const err: Error | undefined = this._type.set(new Protocol.Primitives.Option<number>(2, new Protocol.Primitives.u16(this.type.pointC)));
                    if (err instanceof Error) {
                        throw err;
                    }
                }
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
                    () => { const buffer = this._type.encode(); return this.getBuffer(40, Protocol.ESize.u64, BigInt(buffer.byteLength), buffer); },
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
                this.type = {};
                const typeBuf: ArrayBufferLike = storage.get(40);
                if (typeBuf === undefined) {
                    return new Error(`Fail to get property "type"`);
                }
                if (typeBuf.byteLength > 0) {
                    const typeErr: Error | undefined = this._type.decode(typeBuf);
                    if (typeErr instanceof Error) {
                        return typeErr;
                    } else {
                        switch (this._type.getValueIndex()) {
                            case 0: this.type.pointA = this._type.get<Array<number>>(); break;
                            case 1: this.type.pointB = this._type.get<string>(); break;
                            case 2: this.type.pointC = this._type.get<number>(); break;
                        }
                    }
                }
            }
            public defaults(): UserA {
                return UserA.defaults();
            }
        }

        interface ILoginA {
            users: Array<User>;
        }
        class LoginA extends Protocol.Convertor implements ILoginA {

            public static defaults(): LoginA {
                return new LoginA({ 
                    users: [],
                });
            }
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
                    () => { const self: User = User.defaults(); return this.getBufferFromBuf<User[]>(42, Protocol.ESize.u64, self.encodeSelfArray.bind(self), this.users); },
                ]);
            }
            public decode(buffer: ArrayBufferLike): Error | undefined {
                const storage = this.getStorage(buffer);
                if (storage instanceof Error) {
                    return storage;
                }
                const arrUserInst: User = User.defaults();
                const arrUser: Array<any> | Error = this.getValue<User[]>(storage, 42, arrUserInst.decodeSelfArray.bind(arrUserInst));
                if (arrUser instanceof Error) {
                    return arrUser;
                } else {
                    this.users = arrUser as User[];
                }
            }
            public defaults(): LoginA {
                return LoginA.defaults();
            }
        }

    }

}

