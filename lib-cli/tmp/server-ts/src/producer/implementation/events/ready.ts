import { Producer } from "./index";
import { emit } from "../../events/ready";

export function handler<C>(context: C, producer: Producer<C>): Promise<void> {
	return emit(context, producer);
}
