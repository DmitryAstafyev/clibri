import { test as test_request_structa } from "./test_request_structa";

export class Test {
	public run(): Promise<void> {
		return test_request_structa();
	}
}
