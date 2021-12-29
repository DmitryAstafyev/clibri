// tslint:disable: variable-name
import { Protocol, StructA } from "../consumer/index";
import * as samples from "./samples";

export function test(): Promise<void> {
	return new Promise((resolve, reject) => {
		let case_StructB = false;
		let case_StructC = false;
		let case_StructD = false;
		let case_StructE = false;
		function send(): Promise<void> {
			return new Promise((res, rej) => {
				const request = new StructA(samples.StructA.get());
				request.send().then((response) => {
					if (response instanceof Protocol.StructB) {
						if (!case_StructB) {
							case_StructB = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructB`
								)
							);
						}
					} else if (response instanceof Protocol.StructC) {
						if (!case_StructC) {
							case_StructC = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructC`
								)
							);
						}
					} else if (response instanceof Protocol.StructD) {
						if (!case_StructD) {
							case_StructD = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructD`
								)
							);
						}
					} else if (response instanceof Protocol.StructE) {
						if (!case_StructE) {
							case_StructE = true;
							res(undefined);
						} else {
							rej(
								new Error(
									`Multiple times get: Protocol.StructE`
								)
							);
						}
					}
				});
			});
		}
		function trigger() {
			send()
				.then(() => {
					if (
						case_StructB &&
						case_StructC &&
						case_StructD &&
						case_StructE
					) {
						resolve(undefined);
					} else {
						trigger();
					}
				})
				.catch(reject);
		}
		trigger();
	});
}
