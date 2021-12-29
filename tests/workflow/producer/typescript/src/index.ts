import { Server } from "clibri-websocket-server";
import { Logger, ELogLevel } from "clibri";
import { Producer, Context } from "./producer";
import { panic } from "./tools";

Logger.setGlobalLevel(ELogLevel.warn);

const server: Server = new Server("127.0.0.1:8080");
const context: Context = new Context();
const producer: Producer = new Producer(server, context);
producer.listen().catch((err: Error) => {
	panic(`Fail to start producer: ${err.message}`);
});
