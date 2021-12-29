import { test as test_request_structa } from "./test_request_structa";
import { test as test_request_structc } from "./test_request_structc";
import { test as test_request_structd } from "./test_request_structd";
import { test as test_request_structf } from "./test_request_structf";
import { test as test_request_structempty } from "./test_request_structempty";
import { test as test_request_groupa_structa } from "./test_request_groupa_structa";
import { test as test_request_groupa_structb } from "./test_request_groupa_structb";
import { test as test_request_groupb_structa } from "./test_request_groupb_structa";
import { test as test_request_groupb_groupc_structa } from "./test_request_groupb_groupc_structa";
import { test as test_request_groupb_groupc_structb } from "./test_request_groupb_groupc_structb";
import { Consumer } from "../consumer";

export class Test {
	public run(consumer: Consumer): Promise<void> {
		return new Promise((resolve, reject) => {
			const tests = [
				test_request_structa,
				test_request_structc,
				test_request_structd,
				test_request_groupa_structa,
				test_request_groupa_structb,
				test_request_groupb_structa,
				test_request_groupb_groupc_structa,
				test_request_groupb_groupc_structb,
				test_request_structempty,
				test_request_structf,
			];
			function test() {
				if (tests.length === 0) {
					return resolve();
				}
				const current = tests[0];
				tests.splice(0, 1);
				current(consumer)
					.then(() => {
						test();
					})
					.catch(reject);
			}
			test();
		});
	}
}
