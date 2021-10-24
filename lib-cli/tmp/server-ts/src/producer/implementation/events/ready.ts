import { Context, Producer } from "./index";
import { emit } from "../../events/ready";

export function handler(
	context: Context,
	producer: Producer
): Promise<void> {
	return emit(context, producer);
}
