import Subject from "./tools/tools.subject";
import Subscription from "./tools/tools.subscription";
import guid from "./tools/tools.guid";
import globals from "./tools/tools.globals";
import { init as initBrowserSupport } from "./tools/tools.browser";

export { Client, IClientSubjects } from "./interfaces/client.interface";
export {
	Server,
	IServerSubjects,
	IServerReceivedEvent,
	IServerError,
	EServerErrorContext,
	EServerErrorType,
	Options,
	ProducerIdentificationStrategy,
	ConsumerErrorHandelingStrategy,
	IOptions,
} from "./interfaces/server.interface";
export {
	Logger,
	ELogLevel,
	DefaultLogger,
} from "./interfaces/logger.interface";

export { Subject, Subscription, guid, globals };

initBrowserSupport();
