import { Server } from "fiber-websocket-server";
import { Logger, ELogLevel } from "fiber";
import { Producer, Context } from "./producer";

Logger.setGlobalLevel(ELogLevel.verb);

const server: Server = new Server("127.0.0.1:8080");
const context: Context = new Context();
const producer: Producer = new Producer(server, context);
producer.listen().catch((err: Error) => {
	console.log(`Fail to start producer: ${err.message}`);
});
