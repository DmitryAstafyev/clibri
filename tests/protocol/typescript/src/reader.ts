import * as fs from "fs";
import * as path from "path";
import * as Protocol from "./protocol";

import { state } from "./state";
import { usecases as samples } from "./writer";

const usecases: Array<{ name: string; entity: any }> = [
	{ name: "EnumExampleA.a", entity: Protocol.EnumExampleA },
	{ name: "EnumExampleA.b", entity: Protocol.EnumExampleA },
	{ name: "EnumExampleB.str", entity: Protocol.EnumExampleB },
	{ name: "EnumExampleB.u8", entity: Protocol.EnumExampleB },
	{ name: "EnumExampleB.u16", entity: Protocol.EnumExampleB },
	{ name: "EnumExampleB.u32", entity: Protocol.EnumExampleB },
	{ name: "EnumExampleB.u64", entity: Protocol.EnumExampleB },
	{ name: "EnumExampleB.i8", entity: Protocol.EnumExampleB },
	{ name: "EnumExampleB.i16", entity: Protocol.EnumExampleB },
	{ name: "EnumExampleB.i32", entity: Protocol.EnumExampleB },
	{ name: "EnumExampleB.i64", entity: Protocol.EnumExampleB },
	{ name: "EnumExampleB.f32", entity: Protocol.EnumExampleB },
	{ name: "EnumExampleB.f64", entity: Protocol.EnumExampleB },
	{
		name: "GroupD.EnumExampleP.Option_a",
		entity: Protocol.GroupD.EnumExampleP,
	},
	{
		name: "GroupD.EnumExampleP.Option_b",
		entity: Protocol.GroupD.EnumExampleP,
	},
	{
		name: "GroupD.EnumExampleP.Option_c",
		entity: Protocol.GroupD.EnumExampleP,
	},
	{
		name: "GroupD.EnumExampleP.Option_d",
		entity: Protocol.GroupD.EnumExampleP,
	},
	{ name: "StructExampleA", entity: Protocol.StructExampleA },
	{ name: "StructExampleB", entity: Protocol.StructExampleB },
	{ name: "StructExampleC", entity: Protocol.StructExampleC },
	{ name: "StructExampleD", entity: Protocol.StructExampleD },
	{ name: "StructExampleE", entity: Protocol.StructExampleE },
	{ name: "StructExampleF", entity: Protocol.StructExampleF },
	{ name: "StructExampleG", entity: Protocol.StructExampleG },
	{ name: "StructExampleJ", entity: Protocol.StructExampleJ },
	{ name: "StructExampleEmpty", entity: Protocol.StructExampleEmpty },
	{ name: "GroupAStructExampleA", entity: Protocol.GroupA.StructExampleA },
	{ name: "GroupAStructExampleB", entity: Protocol.GroupA.StructExampleB },
	{ name: "GroupBStructExampleA", entity: Protocol.GroupB.StructExampleA },
	{
		name: "GroupCStructExampleA",
		entity: Protocol.GroupB.GroupC.StructExampleA,
	},
	{
		name: "GroupCStructExampleB",
		entity: Protocol.GroupB.GroupC.StructExampleB,
	},
	{ name: "GroupDStructExampleP", entity: Protocol.GroupD.StructExampleP },
];

function getSampleByName(
	name: string
): Protocol.Convertor<any> | Protocol.Enum<any> {
	const found = samples.find((sample) => sample.name === name);
	if (found === undefined) {
		throw new Error(`Fail to find sample "${name}"`);
	}
	return found.entity;
}

function isFloat(n: any) {
	return Number(n) === n && n % 1 !== 0;
}

function isEqualProp(a: any, b: any): boolean {
	if (typeof a !== typeof b) {
		console.log(`Left: ${a}, right: ${b}`);
		return false;
	}
	if (typeof a === "bigint") {
		if (a !== b) {
			console.log(`Left: ${a}, right: ${b}`);
			return false;
		}
		return true;
	}
	// JS has problems with float... 0.1 can be after parsing 0.1000000001, well, let's prevent it
	if (isFloat(a)) {
		if (a.toFixed(2) !== b.toFixed(2)) {
			console.log(`Left: ${a}, right: ${b}`);
			return false;
		}
		return true;
	}
	if (a instanceof Array) {
		try {
			a.forEach((v, i) => {
				if (!isEqualProp(a[i], b[i])) {
					throw false;
				}
			});
		} catch (e) {
			return false;
		}
		return true;
	}
	if (typeof a === "object") {
		if (!isEqual(a, b)) {
			console.log(`Left: ${a}, right: ${b}`);
			return false;
		}
		return true;
	}
	if (a !== b) {
		console.log(`Left: ${a}, right: ${b}`);
		return false;
	}
	return true;
}

function isEqual(a: any, b: any): boolean {
	if (Object.keys(a).length !== Object.keys(b).length) {
		return false;
	}
	try {
		Object.keys(a).forEach((key: string) => {
			if (typeof a[key] === "function") {
				return;
			}
			if (!isEqualProp(a[key], b[key])) {
				throw false;
			}
		});
	} catch (e) {
		return false;
	}
	return true;
}

export function read(): Promise<void> {
	const dest: string = path.resolve(
		path.dirname(module.filename),
		"../../rust/binary"
	);
	return new Promise((resolve, reject) => {
		if (!fs.existsSync(dest)) {
			return reject(new Error(`Fail to find dest: ${dest}`));
		}
		return Promise.all(
			usecases.map((usecase, index) => {
				return new Promise((res, rej) => {
					if (state.getMiddleware()) {
						return res(undefined);
					}
					const target = path.resolve(
						dest,
						`${usecase.name}.prot.bin`
					);
					fs.open(target, "r", (errOpen, file) => {
						if (errOpen) {
							return rej(
								new Error(
									`Fail to open file ${target} due error: ${errOpen.message}`
								)
							);
						}
						fs.readFile(
							file,
							(
								errWrite: Error | undefined | null,
								buffer: Buffer
							) => {
								if (errWrite) {
									return rej(
										new Error(
											`Fail to read file ${target} due error: ${errWrite.message}`
										)
									);
								}
								const inst = usecase.entity.from(buffer);
								if (inst instanceof Error) {
									return rej(
										new Error(
											`Fail to parse usecase "${usecase.name}": ${inst.message}`
										)
									);
								}
								const sample =
									samples[index].entity instanceof
									Protocol.Enum
										? (
												samples[index]
													.entity as Protocol.Enum<any>
										  ).get()
										: samples[index].entity;
								if (!isEqual(sample, inst)) {
									console.log(
										`\n[TS] File: ${target} has a problems.\n`
									);
									console.log(`Sample:\n${"=".repeat(30)}`);
									console.log(sample);
									console.log(`Instance:\n${"=".repeat(30)}`);
									console.log(inst);
									console.log(`${"=".repeat(30)}`);
									return rej(
										new Error(
											`Parsed object from ${target} isn't equal to sample.`
										)
									);
								}
								fs.stat(
									target,
									(
										err: NodeJS.ErrnoException | null,
										stat: fs.Stats
									) => {
										if (err) {
											return rej(new Error(err.message));
										}
										console.log(
											`[OK]\t[TS] ${stat.size} bytes of file: ${target} has beed read.`
										);
										res(undefined);
									}
								);
							}
						);
					});
				});
			})
		)
			.then(() => {
				const target = path.resolve(
					dest,
					`buffer.prot.${
						state.getMiddleware() ? "middleware" : "bin"
					}`
				);
				fs.open(target, "r", (errOpen, file) => {
					if (errOpen) {
						return reject(
							new Error(
								`Fail to open file ${target} due error: ${errOpen.message}`
							)
						);
					}
					fs.readFile(
						file,
						(
							errWrite: Error | undefined | null,
							buffer: Buffer
						) => {
							if (errWrite) {
								return reject(
									new Error(
										`Fail to read file ${target} due error: ${errWrite.message}`
									)
								);
							}
							const reader: Protocol.BufferReaderMessages =
								new Protocol.BufferReaderMessages();
							const errors: Error[] | undefined =
								reader.chunk(buffer);
							if (errors !== undefined) {
								return reject(
									new Error(
										errors.map((e) => e.message).join("\n")
									)
								);
							}
							let count: number = 0;
							let done: number = 0;
							const marker: string = state.getMiddleware()
								? "[MID]"
								: "";
							do {
								const pack:
									| Protocol.IAvailableMessage<Protocol.IAvailableMessages>
									| undefined = reader.next();
								if (pack === undefined) {
									break;
								}
								count += 1;
								if (pack.msg.EnumExampleA !== undefined) {
									if (
										pack.msg.EnumExampleA.Option_a !==
											undefined &&
										pack.msg.EnumExampleA.Option_a !==
											"Option_a"
									) {
										return reject(
											new Error(
												`EnumExampleA.Option_a incorrect: ${pack.msg.EnumExampleA.Option_a}`
											)
										);
									} else if (
										pack.msg.EnumExampleA.Option_a !==
											undefined &&
										pack.msg.EnumExampleA.Option_a ===
											"Option_a"
									) {
										console.log(
											`[OK]\t${marker} Package EnumExampleA.Option_a is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleA.Option_b !==
											undefined &&
										pack.msg.EnumExampleA.Option_b !==
											"Option_b"
									) {
										return reject(
											new Error(
												`EnumExampleA.Option_b incorrect: ${pack.msg.EnumExampleA.Option_a}`
											)
										);
									} else if (
										pack.msg.EnumExampleA.Option_b !==
											undefined &&
										pack.msg.EnumExampleA.Option_b ===
											"Option_b"
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleA.Option_b is OK`
										);
										done += 1;
									}
								}
								if (pack.msg.EnumExampleB !== undefined) {
									if (
										pack.msg.EnumExampleB.Option_str !==
											undefined &&
										pack.msg.EnumExampleB.Option_str !==
											"Option_str"
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_str incorrect: ${pack.msg.EnumExampleB.Option_str}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_str !==
											undefined &&
										pack.msg.EnumExampleB.Option_str ===
											"Option_str"
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_str is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleB.Option_u8 !==
											undefined &&
										pack.msg.EnumExampleB.Option_u8 !== 8
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_u8 incorrect: ${pack.msg.EnumExampleB.Option_u8}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_u8 !==
											undefined &&
										pack.msg.EnumExampleB.Option_u8 === 8
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_u8 is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleB.Option_u16 !==
											undefined &&
										pack.msg.EnumExampleB.Option_u16 !== 16
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_u16 incorrect: ${pack.msg.EnumExampleB.Option_u16}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_u16 !==
											undefined &&
										pack.msg.EnumExampleB.Option_u16 === 16
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_u16 is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleB.Option_u32 !==
											undefined &&
										pack.msg.EnumExampleB.Option_u32 !== 32
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_u32 incorrect: ${pack.msg.EnumExampleB.Option_u32}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_u32 !==
											undefined &&
										pack.msg.EnumExampleB.Option_u32 === 32
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_u32 is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleB.Option_u64 !==
											undefined &&
										pack.msg.EnumExampleB.Option_u64 !==
											BigInt(64)
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_u64 incorrect: ${pack.msg.EnumExampleB.Option_u64}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_u64 !==
											undefined &&
										pack.msg.EnumExampleB.Option_u64 ===
											BigInt(64)
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_u64 is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleB.Option_i8 !==
											undefined &&
										pack.msg.EnumExampleB.Option_i8 !== -8
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_i8 incorrect: ${pack.msg.EnumExampleB.Option_i8}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_i8 !==
											undefined &&
										pack.msg.EnumExampleB.Option_i8 === -8
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_i8 is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleB.Option_i16 !==
											undefined &&
										pack.msg.EnumExampleB.Option_i16 !== -16
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_i16 incorrect: ${pack.msg.EnumExampleB.Option_i16}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_i16 !==
											undefined &&
										pack.msg.EnumExampleB.Option_i16 === -16
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_i16 is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleB.Option_i32 !==
											undefined &&
										pack.msg.EnumExampleB.Option_i32 !== -32
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_i32 incorrect: ${pack.msg.EnumExampleB.Option_i32}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_i32 !==
											undefined &&
										pack.msg.EnumExampleB.Option_i32 === -32
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_i32 is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleB.Option_i64 !==
											undefined &&
										pack.msg.EnumExampleB.Option_i64 !==
											-BigInt(64)
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_i64 incorrect: ${pack.msg.EnumExampleB.Option_i64}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_i64 !==
											undefined &&
										pack.msg.EnumExampleB.Option_i64 ===
											-BigInt(64)
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_i64 is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleB.Option_f32 !==
											undefined &&
										Math.round(
											pack.msg.EnumExampleB.Option_f32 *
												100
										) /
											100 !==
											0.02
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_f32 incorrect: ${pack.msg.EnumExampleB.Option_f32}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_f32 !==
											undefined &&
										Math.round(
											pack.msg.EnumExampleB.Option_f32 *
												100
										) /
											100 ===
											0.02
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_f32 is OK`
										);
										done += 1;
									}
									if (
										pack.msg.EnumExampleB.Option_f64 !==
											undefined &&
										Math.round(
											pack.msg.EnumExampleB.Option_f64 *
												100
										) /
											100 !==
											0.02
									) {
										return reject(
											new Error(
												`EnumExampleB.Option_f64 incorrect: ${pack.msg.EnumExampleB.Option_f64}`
											)
										);
									} else if (
										pack.msg.EnumExampleB.Option_f64 !==
											undefined &&
										Math.round(
											pack.msg.EnumExampleB.Option_f64 *
												100
										) /
											100 ===
											0.02
									) {
										console.log(
											`[OK]\t${marker}  EnumExampleB.Option_f64 is OK`
										);
										done += 1;
									}
								}
								if (
									pack.msg.StructExampleA !== undefined &&
									!isEqual(
										pack.msg.StructExampleA,
										getSampleByName("StructExampleA")
									)
								) {
									return reject(
										new Error(
											`StructExampleA incorrect: ${pack.msg.StructExampleA}`
										)
									);
								} else if (
									pack.msg.StructExampleA !== undefined &&
									isEqual(
										pack.msg.StructExampleA,
										getSampleByName("StructExampleA")
									)
								) {
									console.log(
										`[OK]\t${marker}  StructExampleA is OK`
									);
									done += 1;
								}
								if (
									pack.msg.StructExampleB !== undefined &&
									!isEqual(
										pack.msg.StructExampleB,
										getSampleByName("StructExampleB")
									)
								) {
									return reject(
										new Error(
											`StructExampleB incorrect: ${pack.msg.StructExampleB}`
										)
									);
								} else if (
									pack.msg.StructExampleB !== undefined &&
									isEqual(
										pack.msg.StructExampleB,
										getSampleByName("StructExampleB")
									)
								) {
									console.log(
										`[OK]\t${marker}  StructExampleB is OK`
									);
									done += 1;
								}
								if (
									pack.msg.StructExampleC !== undefined &&
									!isEqual(
										pack.msg.StructExampleC,
										getSampleByName("StructExampleC")
									)
								) {
									return reject(
										new Error(
											`StructExampleC incorrect: ${pack.msg.StructExampleC}`
										)
									);
								} else if (
									pack.msg.StructExampleC !== undefined &&
									isEqual(
										pack.msg.StructExampleC,
										getSampleByName("StructExampleC")
									)
								) {
									console.log(
										`[OK]\t${marker}  StructExampleC is OK`
									);
									done += 1;
								}
								if (
									pack.msg.StructExampleD !== undefined &&
									!isEqual(
										pack.msg.StructExampleD,
										getSampleByName("StructExampleD")
									)
								) {
									return reject(
										new Error(
											`StructExampleD incorrect: ${pack.msg.StructExampleD}`
										)
									);
								} else if (
									pack.msg.StructExampleD !== undefined &&
									isEqual(
										pack.msg.StructExampleD,
										getSampleByName("StructExampleD")
									)
								) {
									console.log(
										`[OK]\t${marker}  StructExampleD is OK`
									);
									done += 1;
								}
								if (
									pack.msg.StructExampleE !== undefined &&
									!isEqual(
										pack.msg.StructExampleE,
										getSampleByName("StructExampleE")
									)
								) {
									return reject(
										new Error(
											`StructExampleE incorrect: ${pack.msg.StructExampleE}`
										)
									);
								} else if (
									pack.msg.StructExampleE !== undefined &&
									isEqual(
										pack.msg.StructExampleE,
										getSampleByName("StructExampleE")
									)
								) {
									console.log(
										`[OK]\t${marker}  StructExampleE is OK`
									);
									done += 1;
								}
								if (
									pack.msg.StructExampleF !== undefined &&
									!isEqual(
										pack.msg.StructExampleF,
										getSampleByName("StructExampleF")
									)
								) {
									return reject(
										new Error(
											`StructExampleF incorrect: ${pack.msg.StructExampleF}`
										)
									);
								} else if (
									pack.msg.StructExampleF !== undefined &&
									isEqual(
										pack.msg.StructExampleF,
										getSampleByName("StructExampleF")
									)
								) {
									console.log(
										`[OK]\t${marker}  StructExampleF is OK`
									);
									done += 1;
								}
								if (
									pack.msg.StructExampleG !== undefined &&
									!isEqual(
										pack.msg.StructExampleG,
										getSampleByName("StructExampleG")
									)
								) {
									return reject(
										new Error(
											`StructExampleG incorrect: ${pack.msg.StructExampleG}`
										)
									);
								} else if (
									pack.msg.StructExampleG !== undefined &&
									isEqual(
										pack.msg.StructExampleG,
										getSampleByName("StructExampleG")
									)
								) {
									console.log(
										`[OK]\t${marker}  StructExampleG is OK`
									);
									done += 1;
								}
								if (
									pack.msg.StructExampleJ !== undefined &&
									!isEqual(
										pack.msg.StructExampleJ,
										getSampleByName("StructExampleJ")
									)
								) {
									return reject(
										new Error(
											`StructExampleJ incorrect: ${pack.msg.StructExampleJ}`
										)
									);
								} else if (
									pack.msg.StructExampleJ !== undefined &&
									isEqual(
										pack.msg.StructExampleJ,
										getSampleByName("StructExampleJ")
									)
								) {
									console.log(
										`[OK]\t${marker}  StructExampleJ is OK`
									);
									done += 1;
								}
								if (
									pack.msg.StructExampleEmpty !== undefined &&
									!isEqual(
										pack.msg.StructExampleEmpty,
										getSampleByName("StructExampleEmpty")
									)
								) {
									return reject(
										new Error(
											`StructExampleEmpty incorrect: ${pack.msg.StructExampleEmpty}`
										)
									);
								} else if (
									pack.msg.StructExampleEmpty !== undefined &&
									isEqual(
										pack.msg.StructExampleEmpty,
										getSampleByName("StructExampleEmpty")
									)
								) {
									console.log(
										`[OK]\t${marker}  StructExampleEmpty is OK`
									);
									done += 1;
								}
								if (pack.msg.GroupA !== undefined) {
									if (
										pack.msg.GroupA.StructExampleA !==
											undefined &&
										!isEqual(
											pack.msg.GroupA.StructExampleA,
											getSampleByName(
												"GroupAStructExampleA"
											)
										)
									) {
										return reject(
											new Error(
												`GroupA.StructExampleA incorrect: ${pack.msg.GroupA.StructExampleA}`
											)
										);
									} else if (
										pack.msg.GroupA.StructExampleA !==
											undefined &&
										isEqual(
											pack.msg.GroupA.StructExampleA,
											getSampleByName(
												"GroupAStructExampleA"
											)
										)
									) {
										console.log(
											`[OK]\t${marker}  GroupA.StructExampleA is OK`
										);
										done += 1;
									}
									if (
										pack.msg.GroupA.StructExampleB !==
											undefined &&
										!isEqual(
											pack.msg.GroupA.StructExampleB,
											getSampleByName(
												"GroupAStructExampleB"
											)
										)
									) {
										return reject(
											new Error(
												`GroupA.StructExampleB incorrect: ${pack.msg.GroupA.StructExampleB}`
											)
										);
									} else if (
										pack.msg.GroupA.StructExampleB !==
											undefined &&
										isEqual(
											pack.msg.GroupA.StructExampleB,
											getSampleByName(
												"GroupAStructExampleB"
											)
										)
									) {
										console.log(
											`[OK]\t${marker}  GroupA.StructExampleB is OK`
										);
										done += 1;
									}
								}
								if (pack.msg.GroupB !== undefined) {
									if (
										pack.msg.GroupB.StructExampleA !==
											undefined &&
										!isEqual(
											pack.msg.GroupB.StructExampleA,
											getSampleByName(
												"GroupBStructExampleA"
											)
										)
									) {
										return reject(
											new Error(
												`GroupB.StructExampleA incorrect: ${pack.msg.GroupB.StructExampleA}`
											)
										);
									} else if (
										pack.msg.GroupB.StructExampleA !==
											undefined &&
										isEqual(
											pack.msg.GroupB.StructExampleA,
											getSampleByName(
												"GroupBStructExampleA"
											)
										)
									) {
										console.log(
											`[OK]\t${marker}  GroupB.StructExampleA is OK`
										);
										done += 1;
									}
									if (pack.msg.GroupB.GroupC !== undefined) {
										if (
											pack.msg.GroupB.GroupC
												.StructExampleA !== undefined &&
											!isEqual(
												pack.msg.GroupB.GroupC
													.StructExampleA,
												getSampleByName(
													"GroupCStructExampleA"
												)
											)
										) {
											return reject(
												new Error(
													`GroupB.GroupC.StructExampleA incorrect: ${pack.msg.GroupB.GroupC.StructExampleA}`
												)
											);
										} else if (
											pack.msg.GroupB.GroupC
												.StructExampleA !== undefined &&
											isEqual(
												pack.msg.GroupB.GroupC
													.StructExampleA,
												getSampleByName(
													"GroupCStructExampleA"
												)
											)
										) {
											console.log(
												`[OK]\t${marker}  GroupB.GroupC.StructExampleA is OK`
											);
											done += 1;
										}
										if (
											pack.msg.GroupB.GroupC
												.StructExampleB !== undefined &&
											!isEqual(
												pack.msg.GroupB.GroupC
													.StructExampleB,
												getSampleByName(
													"GroupCStructExampleB"
												)
											)
										) {
											return reject(
												new Error(
													`GroupB.GroupC.StructExampleB incorrect: ${pack.msg.GroupB.GroupC.StructExampleB}`
												)
											);
										} else if (
											pack.msg.GroupB.GroupC
												.StructExampleB !== undefined &&
											isEqual(
												pack.msg.GroupB.GroupC
													.StructExampleB,
												getSampleByName(
													"GroupCStructExampleB"
												)
											)
										) {
											console.log(
												`[OK]\t${marker}  GroupB.GroupC.StructExampleB is OK`
											);
											done += 1;
										}
									}
								}
								if (pack.msg.GroupD !== undefined) {
									if (
										pack.msg.GroupD.StructExampleP !==
											undefined &&
										!isEqual(
											pack.msg.GroupD.StructExampleP,
											getSampleByName(
												"GroupDStructExampleP"
											)
										)
									) {
										return reject(
											new Error(
												`GroupD.StructExampleP incorrect: ${pack.msg.GroupD.StructExampleP}`
											)
										);
									} else if (
										pack.msg.GroupD.StructExampleP !==
											undefined &&
										isEqual(
											pack.msg.GroupD.StructExampleP,
											getSampleByName(
												"GroupDStructExampleP"
											)
										)
									) {
										console.log(
											`[OK]\t${marker}  GroupD.StructExampleP is OK`
										);
										done += 1;
									}
									if (
										pack.msg.GroupD.EnumExampleP !==
											undefined &&
										pack.msg.GroupD.EnumExampleP
											.Option_a !== undefined &&
										!isEqual(
											pack.msg.GroupD.EnumExampleP,
											(
												getSampleByName(
													"GroupD.EnumExampleP.Option_a"
												) as any
											).get()
										)
									) {
										return reject(
											new Error(
												`GroupD.EnumExampleP.Option_a incorrect: ${pack.msg.GroupD.EnumExampleP.Option_a}`
											)
										);
									} else if (
										pack.msg.GroupD.EnumExampleP !==
											undefined &&
										pack.msg.GroupD.EnumExampleP
											.Option_a !== undefined &&
										isEqual(
											pack.msg.GroupD.EnumExampleP,
											(
												getSampleByName(
													"GroupD.EnumExampleP.Option_a"
												) as any
											).get()
										)
									) {
										console.log(
											`[OK]\t${marker}  GroupD.EnumExampleP.Option_a is OK`
										);
										done += 1;
									}
									if (
										pack.msg.GroupD.EnumExampleP !==
											undefined &&
										pack.msg.GroupD.EnumExampleP
											.Option_b !== undefined &&
										!isEqual(
											pack.msg.GroupD.EnumExampleP,
											(
												getSampleByName(
													"GroupD.EnumExampleP.Option_b"
												) as any
											).get()
										)
									) {
										return reject(
											new Error(
												`GroupD.EnumExampleP.Option_b incorrect: ${pack.msg.GroupD.EnumExampleP.Option_b}`
											)
										);
									} else if (
										pack.msg.GroupD.EnumExampleP !==
											undefined &&
										pack.msg.GroupD.EnumExampleP
											.Option_b !== undefined &&
										isEqual(
											pack.msg.GroupD.EnumExampleP,
											(
												getSampleByName(
													"GroupD.EnumExampleP.Option_b"
												) as any
											).get()
										)
									) {
										console.log(
											`[OK]\t${marker}  GroupD.EnumExampleP.Option_b is OK`
										);
										done += 1;
									}
									if (
										pack.msg.GroupD.EnumExampleP !==
											undefined &&
										pack.msg.GroupD.EnumExampleP
											.Option_c !== undefined &&
										!isEqual(
											pack.msg.GroupD.EnumExampleP,
											(
												getSampleByName(
													"GroupD.EnumExampleP.Option_c"
												) as any
											).get()
										)
									) {
										return reject(
											new Error(
												`GroupD.EnumExampleP.Option_c incorrect: ${pack.msg.GroupD.EnumExampleP.Option_c}`
											)
										);
									} else if (
										pack.msg.GroupD.EnumExampleP !==
											undefined &&
										pack.msg.GroupD.EnumExampleP
											.Option_c !== undefined &&
										isEqual(
											pack.msg.GroupD.EnumExampleP,
											(
												getSampleByName(
													"GroupD.EnumExampleP.Option_c"
												) as any
											).get()
										)
									) {
										console.log(
											`[OK]\t${marker}  GroupD.EnumExampleP.Option_c is OK`
										);
										done += 1;
									}
									if (
										pack.msg.GroupD.EnumExampleP !==
											undefined &&
										pack.msg.GroupD.EnumExampleP
											.Option_d !== undefined &&
										!isEqual(
											pack.msg.GroupD.EnumExampleP,
											(
												getSampleByName(
													"GroupD.EnumExampleP.Option_d"
												) as any
											).get()
										)
									) {
										return reject(
											new Error(
												`GroupD.EnumExampleP.Option_d incorrect: ${pack.msg.GroupD.EnumExampleP.Option_d}`
											)
										);
									} else if (
										pack.msg.GroupD.EnumExampleP !==
											undefined &&
										pack.msg.GroupD.EnumExampleP
											.Option_d !== undefined &&
										isEqual(
											pack.msg.GroupD.EnumExampleP,
											(
												getSampleByName(
													"GroupD.EnumExampleP.Option_d"
												) as any
											).get()
										)
									) {
										console.log(
											`[OK]\t${marker}  GroupD.EnumExampleP.Option_d is OK`
										);
										done += 1;
									}
								}
							} while (true);
							if (
								count !== 32 ||
								done !== count ||
								reader.pending() > 0 ||
								reader.len() > 0
							) {
								return reject(
									new Error(
										`Fail to correctly read buffer file:\n\tcount = ${count};\n\tpending=${reader.pending()};\n\tlen=${reader.len()}\n\tbuffer=${
											buffer.byteLength
										}`
									)
								);
							}
							fs.stat(
								target,
								(
									err: NodeJS.ErrnoException | null,
									stat: fs.Stats
								) => {
									if (err) {
										return reject(new Error(err.message));
									}
									console.log(
										`[OK]\t[TS] ${stat.size} bytes of file: ${target} has beed read.`
									);
									resolve(undefined);
								}
							);
						}
					);
				});
			})
			.catch(reject);
	});
}
