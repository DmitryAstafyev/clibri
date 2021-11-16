import { Server } from "clibri-websocket-server";
import { Logger, ELogLevel } from "clibri";
import { Producer, Context } from "./producer";

Logger.setGlobalLevel(ELogLevel.verb);

const server: Server = new Server("127.0.0.1:8080");
const context: Context = new Context();
const producer: Producer = new Producer(server, context);
producer
	.listen()
	.then(() => {
		console.log(`Producer has been started`);
	})
	.catch((err: Error) => {
		console.log(`Fail to start producer: ${err.message}`);
	});
