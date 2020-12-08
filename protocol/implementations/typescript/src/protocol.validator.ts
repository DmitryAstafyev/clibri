/*
const map = [
    { prop: 'u8', types: Protocol.Primitives.u8, repeated: false, optional: true },
    { prop: 'u16', types: Protocol.Primitives.u16, repeated: false, optional: false },
    { prop: 'u32', types: Protocol.Primitives.u32, repeated: false, optional: false },
    { prop: 'opt', repeated: false, optional: false, options: [
        { prop: 'u8', types: Protocol.Primitives.u8, repeated: false },
        { prop: 'u16', types: Protocol.Primitives.u16, repeated: false },
    ] },
];
*/
export interface IValidator {
    validate(value: any): Error | undefined;
}

export interface IPropScheme {
    prop: string;
    optional?: boolean;
    types?: Required<IValidator>,
    options?: IPropScheme[],
}

export function validate(obj: any, scheme: IPropScheme[]): Error | undefined {
    if (typeof obj !== 'object' || obj === null) {
        return new Error(`Expecting input to be object`);
    }
    const errors: string[] = scheme.map((property: IPropScheme) => {
        if (property.optional && obj[property.prop] === undefined) {
            return undefined;
        }
        if (property.types !== undefined) {
            const err: Error | undefined = property.types.validate(obj[property.prop]);
            if (err instanceof Error) {
                return err.message;
            } else {
                return undefined;
            }
        } else if (property.options instanceof Array) {
            if (typeof obj[property.prop] !== 'object' || obj[property.prop] === null) {
                return `Property "${property.prop}" should be an object, because it's enum`;
            }
            const target: any = obj[property.prop];
            try {
                property.options.forEach((prop: IPropScheme) => {
                    if (prop.types === undefined) {
                        throw new Error(`Invalid option description for option "${prop.prop}" of option "${property.prop}"`);
                    }
                    if (target[prop.prop] !== undefined) {
                        const err: Error | undefined = prop.types.validate(target[prop.prop]);
                        if (err instanceof Error) {
                            throw new Error(`Fail to validate option "${prop.prop}" of option "${property.prop}" due: ${err.message}`);
                        }
                    }
                });
            } catch (e) {
                return e.message;
            }
            return undefined;
        } else {
            return `Invalid map definition for property ${property.prop}`
        }
    });
    return errors.length > 0 ? new Error(errors.join('\n')) : undefined;
}
