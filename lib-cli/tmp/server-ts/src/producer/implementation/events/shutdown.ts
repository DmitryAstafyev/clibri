import { Producer } from "./index";
import { emit } from "../../events/shutdown";

export function handler<C>(context: C, producer: Producer<C>): Promise<void> {
	return emit(context, producer);
}
