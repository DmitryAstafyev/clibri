import { Context, Producer } from "./index";
import { emit } from "../../events/shutdown";

export function handler(context: Context, producer: Producer): Promise<void> {
	return emit(context, producer);
}
